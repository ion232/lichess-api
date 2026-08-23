use clap::Subcommand;
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use lichess_api::client::LichessApi;
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Subcommand)]
pub enum FideCommand {
    /// Search FIDE players by name
    Search {
        /// Search query
        query: String,
    },
    /// Get a FIDE player by ID
    Player {
        /// FIDE player ID
        id: u32,
    },
    /// Get the rating history of a FIDE player
    Ratings {
        /// FIDE player ID
        id: u32,
    },
}

impl FideCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            FideCommand::Search { query } => {
                let players = lichess
                    .search_fide_player(query.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to search fide players for '{query}'"))?;
                output::print(&players, json);
                Ok(())
            }
            FideCommand::Player { id } => {
                let player = lichess
                    .get_fide_player(id)
                    .await
                    .wrap_err_with(|| format!("failed to fetch fide player '{id}'"))?;
                output::print(&player, json);
                Ok(())
            }
            FideCommand::Ratings { id } => {
                let ratings = lichess
                    .get_fide_player_ratings(id)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch rating history for fide player '{id}'")
                    })?;
                output::print(&ratings, json);
                Ok(())
            }
        }
    }
}
