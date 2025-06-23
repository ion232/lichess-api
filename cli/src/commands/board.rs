use clap::Subcommand;
use color_eyre::Result;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::board::*;
use lichess_api::model::{Color, Room, VariantKey};
use reqwest;

type Lichess = LichessApi<reqwest::Client>;

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
        /// Room (player or spectator)
        #[arg(long, default_value = "player")]
        room: String,
        /// Message text
        text: String,
    },
    /// Claim victory when the opponent has left the game for a while
    ClaimVictory {
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
        #[arg(long, default_value = "standard")]
        variant: String,
        /// Color preference (random, white, black)
        #[arg(long, default_value = "random")]
        color: String,
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
    pub async fn run(self, lichess: Lichess) -> Result<()> {
        match self {
            BoardCommand::Abort { game_id } => {
                let request = abort::PostRequest::new(&game_id);
                let result = lichess.board_abort_game(request).await?;
                println!("Game aborted: {}", result);
                Ok(())
            }
            BoardCommand::Berserk { game_id } => {
                let request = berserk::PostRequest::new(&game_id);
                let result = lichess.board_berserk_game(request).await?;
                println!("Berserk activated: {}", result);
                Ok(())
            }
            BoardCommand::StreamChat { game_id } => {
                let request = chat::GetRequest::new(&game_id);
                let mut stream = lichess.board_stream_game_chat(request).await?;
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
                let room_enum = match room.as_str() {
                    "spectator" => Room::Spectator,
                    _ => Room::Player,
                };
                let request = chat::PostRequest::new(&game_id, room_enum, &text);
                let result = lichess.board_write_in_chat(request).await?;
                println!("Message sent: {}", result);
                Ok(())
            }
            BoardCommand::ClaimVictory { game_id } => {
                let request = claim_victory::PostRequest::new(&game_id);
                let result = lichess.board_claim_victory(request).await?;
                println!("Victory claimed: {}", result);
                Ok(())
            }
            BoardCommand::HandleDraw { game_id, accept } => {
                let request = draw::PostRequest::new(&game_id, accept);
                let result = lichess.board_handle_draw(request).await?;
                println!("Draw handled: {}", result);
                Ok(())
            }
            BoardCommand::MakeMove {
                game_id,
                r#move,
                offering_draw,
            } => {
                let request = r#move::PostRequest::new(&game_id, &r#move, offering_draw);
                let result = lichess.board_make_move(request).await?;
                println!("Move made: {}", result);
                Ok(())
            }
            BoardCommand::Resign { game_id } => {
                let request = resign::PostRequest::new(&game_id);
                let result = lichess.board_resign_game(request).await?;
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
                let variant_key = match variant.as_str() {
                    "standard" => VariantKey::Standard,
                    "chess960" => VariantKey::Chess960,
                    "crazyhouse" => VariantKey::Crazyhouse,
                    "antichess" => VariantKey::Antichess,
                    "atomic" => VariantKey::Atomic,
                    "horde" => VariantKey::Horde,
                    "kingOfTheHill" => VariantKey::KingOfTheHill,
                    "racingKings" => VariantKey::RacingKings,
                    "threeCheck" => VariantKey::ThreeCheck,
                    _ => VariantKey::Standard,
                };

                let color_choice = match color.as_str() {
                    "white" => Color::White,
                    "black" => Color::Black,
                    _ => Color::Random,
                };

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
                    variant: variant_key,
                    color: color_choice,
                    rating_range,
                };

                let request = seek::PostRequest::new(query);
                let mut stream = lichess.board_create_a_seek(request).await?;
                println!("Creating seek:");
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(json) => println!("Event: {}", json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BoardCommand::StreamEvents => {
                let request = stream::events::GetRequest::new();
                let mut stream = lichess.board_stream_incoming_events(request).await?;
                println!("Streaming incoming events:");
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(event) => println!("Event: {:#?}", event),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BoardCommand::StreamGame { game_id } => {
                let request = stream::game::GetRequest::new(&game_id);
                let mut stream = lichess.board_stream_board_state(request).await?;
                println!("Streaming game state:");
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(event) => println!("Event: {:#?}", event),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BoardCommand::HandleTakeback { game_id, accept } => {
                let request = takeback::PostRequest::new(&game_id, accept);
                let result = lichess.board_handle_takeback(request).await?;
                println!("Takeback handled: {}", result);
                Ok(())
            }
        }
    }
}
