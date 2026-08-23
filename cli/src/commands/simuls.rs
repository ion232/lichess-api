use clap::Subcommand;
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use lichess_api::client::LichessApi;
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Subcommand)]
pub enum SimulsCommand {
    /// Get current simuls
    Current,
}

impl SimulsCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            SimulsCommand::Current => {
                let simuls = lichess
                    .get_current_simuls()
                    .await
                    .wrap_err("failed to fetch current simuls")?;
                output::print(&simuls, json);
                Ok(())
            }
        }
    }
}
