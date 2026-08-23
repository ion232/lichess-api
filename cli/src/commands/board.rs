use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::board::*;
use lichess_api::model::{Color as LichessColor, Room, VariantKey};
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Clone, ValueEnum)]
pub enum ChatRoom {
    Player,
    Spectator,
}

impl From<ChatRoom> for Room {
    fn from(room: ChatRoom) -> Self {
        match room {
            ChatRoom::Player => Room::Player,
            ChatRoom::Spectator => Room::Spectator,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Variant {
    Standard,
    Chess960,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    ThreeCheck,
}

impl From<Variant> for VariantKey {
    fn from(variant: Variant) -> Self {
        match variant {
            Variant::Standard => VariantKey::Standard,
            Variant::Chess960 => VariantKey::Chess960,
            Variant::Crazyhouse => VariantKey::Crazyhouse,
            Variant::Antichess => VariantKey::Antichess,
            Variant::Atomic => VariantKey::Atomic,
            Variant::Horde => VariantKey::Horde,
            Variant::KingOfTheHill => VariantKey::KingOfTheHill,
            Variant::RacingKings => VariantKey::RacingKings,
            Variant::ThreeCheck => VariantKey::ThreeCheck,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SeekColor {
    Random,
    White,
    Black,
}

impl From<SeekColor> for LichessColor {
    fn from(color: SeekColor) -> Self {
        match color {
            SeekColor::Random => LichessColor::Random,
            SeekColor::White => LichessColor::White,
            SeekColor::Black => LichessColor::Black,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum BoardCommand {
    /// Abort a game
    Abort {
        /// Game ID
        game_id: String,
    },
    /// Go berserk on an arena tournament game
    Berserk {
        /// Game ID
        game_id: String,
    },
    /// Stream the messages of a game chat
    StreamChat {
        /// Game ID
        game_id: String,
    },
    /// Write in the chat of a game
    WriteChat {
        /// Game ID
        game_id: String,
        /// Room
        #[arg(long, value_enum, default_value = "player")]
        room: ChatRoom,
        /// Message text
        text: String,
    },
    /// Claim victory when the opponent has left the game for a while
    ClaimVictory {
        /// Game ID
        game_id: String,
    },
    /// Claim a draw, or agree to an opponent's draw offer
    ClaimDraw {
        /// Game ID
        game_id: String,
    },
    /// Create/accept/decline draw offers
    HandleDraw {
        /// Game ID
        game_id: String,
        /// Accept a draw offer
        #[arg(long)]
        accept: bool,
    },
    /// Make a move in a game
    MakeMove {
        /// Game ID
        game_id: String,
        /// Move in UCI format (e.g., e2e4)
        r#move: String,
        /// Whether to offer a draw
        #[arg(long)]
        offering_draw: bool,
    },
    /// Resign a game
    Resign {
        /// Game ID
        game_id: String,
    },
    /// Create a seek to get paired with another player
    CreateSeek {
        /// Whether the game is rated
        #[arg(long)]
        rated: bool,
        /// Clock time in minutes
        #[arg(long)]
        time: Option<f32>,
        /// Clock increment in seconds
        #[arg(long)]
        increment: Option<u32>,
        /// Days per turn for correspondence games
        #[arg(long)]
        days: Option<u32>,
        /// Chess variant
        #[arg(long, value_enum, default_value = "standard")]
        variant: Variant,
        /// Color preference
        #[arg(long, value_enum, default_value = "random")]
        color: SeekColor,
        /// Rating range minimum
        #[arg(long)]
        rating_range_min: Option<u32>,
        /// Rating range maximum
        #[arg(long)]
        rating_range_max: Option<u32>,
    },
    /// Stream incoming events (challenges, game starts)
    StreamEvents,
    /// Stream the state of a game being played
    StreamGame {
        /// Game ID
        game_id: String,
    },
    /// Propose/accept/decline takebacks
    HandleTakeback {
        /// Game ID
        game_id: String,
        /// Accept a takeback offer
        #[arg(long)]
        accept: bool,
    },
}

impl BoardCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            BoardCommand::Abort { game_id } => {
                let request = abort::PostRequest::new(&game_id);
                let result = lichess
                    .board_abort_game(request)
                    .await
                    .wrap_err_with(|| format!("failed to abort game '{game_id}'"))?;
                println!("Game aborted: {}", result);
                Ok(())
            }
            BoardCommand::Berserk { game_id } => {
                let request = berserk::PostRequest::new(&game_id);
                let result = lichess
                    .board_berserk_game(request)
                    .await
                    .wrap_err_with(|| format!("failed to berserk game '{game_id}'"))?;
                println!("Berserk activated: {}", result);
                Ok(())
            }
            BoardCommand::StreamChat { game_id } => {
                let request = chat::GetRequest::new(&game_id);
                let mut stream = lichess
                    .board_stream_game_chat(request)
                    .await
                    .wrap_err_with(|| format!("failed to stream chat for game '{game_id}'"))?;
                println!("Streaming chat messages:");
                while let Some(Ok(messages)) = stream.next().await {
                    for chat_line in messages {
                        println!("{}: {}", chat_line.user, chat_line.text);
                    }
                }
                Ok(())
            }
            BoardCommand::WriteChat {
                game_id,
                room,
                text,
            } => {
                let request = chat::PostRequest::new(&game_id, room.into(), &text);
                let result = lichess
                    .board_write_in_chat(request)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to write chat message to game '{game_id}'")
                    })?;
                println!("Message sent: {}", result);
                Ok(())
            }
            BoardCommand::ClaimVictory { game_id } => {
                let request = claim_victory::PostRequest::new(&game_id);
                let result = lichess
                    .board_claim_victory(request)
                    .await
                    .wrap_err_with(|| format!("failed to claim victory for game '{game_id}'"))?;
                println!("Victory claimed: {}", result);
                Ok(())
            }
            BoardCommand::ClaimDraw { game_id } => {
                let request = claim_draw::PostRequest::new(&game_id);
                let result = lichess
                    .board_claim_draw(request)
                    .await
                    .wrap_err_with(|| format!("failed to claim draw for game '{game_id}'"))?;
                println!("Draw claimed: {}", result);
                Ok(())
            }
            BoardCommand::HandleDraw { game_id, accept } => {
                let request = draw::PostRequest::new(&game_id, accept);
                let result = lichess
                    .board_handle_draw(request)
                    .await
                    .wrap_err_with(|| format!("failed to handle draw for game '{game_id}'"))?;
                println!("Draw handled: {}", result);
                Ok(())
            }
            BoardCommand::MakeMove {
                game_id,
                r#move,
                offering_draw,
            } => {
                let request = r#move::PostRequest::new(&game_id, &r#move, offering_draw);
                let result = lichess.board_make_move(request).await.wrap_err_with(|| {
                    format!("failed to make move '{move}' in game '{game_id}'")
                })?;
                println!("Move made: {}", result);
                Ok(())
            }
            BoardCommand::Resign { game_id } => {
                let request = resign::PostRequest::new(&game_id);
                let result = lichess
                    .board_resign_game(request)
                    .await
                    .wrap_err_with(|| format!("failed to resign game '{game_id}'"))?;
                println!("Game resigned: {}", result);
                Ok(())
            }
            BoardCommand::CreateSeek {
                rated,
                time,
                increment,
                days,
                variant,
                color,
                rating_range_min,
                rating_range_max,
            } => {
                let seek_type = if let Some(days) = days {
                    seek::SeekType::Correspondence { days: days.into() }
                } else {
                    let time_mins = time.unwrap_or(10.0);
                    let time_secs = (time_mins * 60.0) as u32;
                    seek::SeekType::RealTime {
                        time: time_secs,
                        increment: increment.unwrap_or(0),
                    }
                };

                let rating_range = if rating_range_min.is_some() || rating_range_max.is_some() {
                    format!(
                        "{}-{}",
                        rating_range_min.unwrap_or(800),
                        rating_range_max.unwrap_or(2800)
                    )
                } else {
                    "".to_string()
                };

                let query = seek::PostQuery {
                    seek_type,
                    rated,
                    variant: variant.into(),
                    color: color.into(),
                    rating_range,
                };

                let request = seek::PostRequest::new(query);
                let mut stream = lichess
                    .board_create_a_seek(request)
                    .await
                    .wrap_err("failed to create seek")?;
                println!("Creating seek:");
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(value) => output::print(&value, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BoardCommand::StreamEvents => {
                let request = stream::events::GetRequest::new();
                let mut stream = lichess
                    .board_stream_incoming_events(request)
                    .await
                    .wrap_err("failed to stream incoming events")?;
                println!("Streaming incoming events:");
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(event) => output::print(&event, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BoardCommand::StreamGame { game_id } => {
                let request = stream::game::GetRequest::new(&game_id);
                let mut stream = lichess
                    .board_stream_board_state(request)
                    .await
                    .wrap_err_with(|| format!("failed to stream game state for '{game_id}'"))?;
                println!("Streaming game state:");
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(event) => output::print(&event, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BoardCommand::HandleTakeback { game_id, accept } => {
                let request = takeback::PostRequest::new(&game_id, accept);
                let result = lichess
                    .board_handle_takeback(request)
                    .await
                    .wrap_err_with(|| format!("failed to handle takeback for game '{game_id}'"))?;
                println!("Takeback handled: {}", result);
                Ok(())
            }
        }
    }
}
