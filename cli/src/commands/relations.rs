use clap::Subcommand;
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::relations::*;
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Subcommand)]
pub enum RelationsCommand {
    /// Get users you follow
    Following,
    /// Follow a user
    Follow {
        /// Username
        username: String,
    },
    /// Unfollow a user
    Unfollow {
        /// Username
        username: String,
    },
    /// Block a user
    Block {
        /// Username
        username: String,
    },
    /// Unblock a user
    Unblock {
        /// Username
        username: String,
    },
}

impl RelationsCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            RelationsCommand::Following => {
                let mut stream = lichess
                    .get_following(following::GetRequest::new())
                    .await
                    .wrap_err("failed to fetch followed users")?;
                while let Some(user) = stream.next().await {
                    match user {
                        Ok(user) => output::print(&user, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            RelationsCommand::Follow { username } => {
                let result = lichess
                    .follow_user(username.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to follow '{username}'"))?;
                println!("Followed '{username}': {}", result);
                Ok(())
            }
            RelationsCommand::Unfollow { username } => {
                let result = lichess
                    .unfollow_user(username.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to unfollow '{username}'"))?;
                println!("Unfollowed '{username}': {}", result);
                Ok(())
            }
            RelationsCommand::Block { username } => {
                let result = lichess
                    .block_user(username.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to block '{username}'"))?;
                println!("Blocked '{username}': {}", result);
                Ok(())
            }
            RelationsCommand::Unblock { username } => {
                let result = lichess
                    .unblock_user(username.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to unblock '{username}'"))?;
                println!("Unblocked '{username}': {}", result);
                Ok(())
            }
        }
    }
}
