use clap::Subcommand;
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use lichess_api::client::LichessApi;
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Subcommand)]
pub enum TablebaseCommand {
    /// Look up an antichess position
    Antichess {
        /// FEN of the position
        fen: String,
    },
    /// Look up an atomic chess position
    Atomic {
        /// FEN of the position
        fen: String,
    },
    /// Look up a standard chess position
    Standard {
        /// FEN of the position
        fen: String,
    },
}

impl TablebaseCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            TablebaseCommand::Antichess { fen } => {
                let result = lichess
                    .lookup_antichess(fen.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to look up antichess position '{fen}'"))?;
                output::print(&result, json);
                Ok(())
            }
            TablebaseCommand::Atomic { fen } => {
                let result = lichess
                    .lookup_atomic(fen.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to look up atomic position '{fen}'"))?;
                output::print(&result, json);
                Ok(())
            }
            TablebaseCommand::Standard { fen } => {
                let result = lichess
                    .lookup_standard(fen.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to look up standard position '{fen}'"))?;
                output::print(&result, json);
                Ok(())
            }
        }
    }
}
