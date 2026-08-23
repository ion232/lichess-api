use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::Room;
use lichess_api::model::bot::*;
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

#[derive(Debug, Subcommand)]
pub enum BotCommand {
    /// Abort a game
    Abort {
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
    /// Claim a draw, or agree to an opponent's draw offer
    ClaimDraw {
        /// Game ID
        game_id: String,
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
    /// Get online bot accounts
    Online {
        /// Number of bots to fetch
        #[arg(default_value = "50")]
        nb: u32,
    },
    /// Resign a game
    Resign {
        /// Game ID
        game_id: String,
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
    /// Upgrade your account to a Bot account (irreversible)
    UpgradeAccount,
}

impl BotCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            BotCommand::Abort { game_id } => {
                let result = lichess
                    .bot_abort_game(game_id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to abort game '{game_id}'"))?;
                println!("Game aborted: {}", result);
                Ok(())
            }
            BotCommand::StreamChat { game_id } => {
                let mut stream = lichess
                    .bot_stream_game_chat(game_id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to stream chat for game '{game_id}'"))?;
                println!("Streaming chat messages:");
                while let Some(Ok(line)) = stream.next().await {
                    println!("{}: {}", line.user, line.text);
                }
                Ok(())
            }
            BotCommand::WriteChat {
                game_id,
                room,
                text,
            } => {
                let request = chat::PostRequest::new(&game_id, room.into(), &text);
                let result = lichess.bot_write_in_chat(request).await.wrap_err_with(|| {
                    format!("failed to write chat message to game '{game_id}'")
                })?;
                println!("Message sent: {}", result);
                Ok(())
            }
            BotCommand::ClaimDraw { game_id } => {
                let result = lichess
                    .bot_claim_draw(game_id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to claim draw for game '{game_id}'"))?;
                println!("Draw claimed: {}", result);
                Ok(())
            }
            BotCommand::ClaimVictory { game_id } => {
                let result = lichess
                    .bot_claim_victory(game_id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to claim victory for game '{game_id}'"))?;
                println!("Victory claimed: {}", result);
                Ok(())
            }
            BotCommand::HandleDraw { game_id, accept } => {
                let request = draw::PostRequest::new(&game_id, accept);
                let result = lichess
                    .bot_draw_game(request)
                    .await
                    .wrap_err_with(|| format!("failed to handle draw for game '{game_id}'"))?;
                println!("Draw handled: {}", result);
                Ok(())
            }
            BotCommand::MakeMove {
                game_id,
                r#move,
                offering_draw,
            } => {
                let request = r#move::PostRequest::new(&game_id, &r#move, offering_draw);
                let result = lichess.bot_make_move(request).await.wrap_err_with(|| {
                    format!("failed to make move '{move}' in game '{game_id}'")
                })?;
                println!("Move made: {}", result);
                Ok(())
            }
            BotCommand::Online { nb } => {
                let mut stream = lichess
                    .bot_get_online(nb)
                    .await
                    .wrap_err("failed to fetch online bots")?;
                while let Some(bot) = stream.next().await {
                    match bot {
                        Ok(bot) => output::print(&bot, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BotCommand::Resign { game_id } => {
                let result = lichess
                    .bot_resign_game(game_id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to resign game '{game_id}'"))?;
                println!("Game resigned: {}", result);
                Ok(())
            }
            BotCommand::StreamEvents => {
                let request = stream::events::GetRequest::new();
                let mut stream = lichess
                    .bot_stream_incoming_events(request)
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
            BotCommand::StreamGame { game_id } => {
                let request = stream::game::GetRequest::new(&game_id);
                let mut stream = lichess
                    .bot_stream_board_state(request)
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
            BotCommand::HandleTakeback { game_id, accept } => {
                let request = takeback::PostRequest::new(&game_id, accept);
                let result = lichess
                    .bot_handle_takeback(request)
                    .await
                    .wrap_err_with(|| format!("failed to handle takeback for game '{game_id}'"))?;
                println!("Takeback handled: {}", result);
                Ok(())
            }
            BotCommand::UpgradeAccount => {
                let result = lichess
                    .bot_upgrade_account(upgrade::PostRequest::new())
                    .await
                    .wrap_err("failed to upgrade account to a bot account")?;
                println!("Account upgraded to Bot account: {}", result);
                Ok(())
            }
        }
    }
}
