use clap::Subcommand;
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use lichess_api::client::LichessApi;
use lichess_api::model::account::*;
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Subcommand)]
pub enum AccountCommand {
    /// Get your public profile
    Profile,
    /// Get your email address
    Email,
    /// Get your preferences
    Preferences,
    /// Get your kid mode status
    KidModeStatus,
    /// Set your kid mode status
    SetKidMode {
        /// Turn kid mode on or off
        #[arg(long)]
        on: bool,
    },
    /// Get your timeline
    Timeline {
        /// Only include entries since this timestamp (ms)
        #[arg(long)]
        since: Option<u64>,
        /// Max number of entries to fetch
        #[arg(long)]
        nb: Option<u32>,
    },
}

impl AccountCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            AccountCommand::Profile => {
                let profile = lichess
                    .get_profile()
                    .await
                    .wrap_err("failed to fetch profile")?;
                output::print(&profile, json);
                Ok(())
            }
            AccountCommand::Email => {
                let email = lichess
                    .get_email_address()
                    .await
                    .wrap_err("failed to fetch email address")?;
                output::print(&email, json);
                Ok(())
            }
            AccountCommand::Preferences => {
                let preferences = lichess
                    .get_preferences()
                    .await
                    .wrap_err("failed to fetch preferences")?;
                output::print(&preferences, json);
                Ok(())
            }
            AccountCommand::KidModeStatus => {
                let status = lichess
                    .get_kid_mode_status()
                    .await
                    .wrap_err("failed to fetch kid mode status")?;
                output::print(&status, json);
                Ok(())
            }
            AccountCommand::SetKidMode { on } => {
                let result = lichess
                    .set_kid_mode_status(on)
                    .await
                    .wrap_err("failed to set kid mode status")?;
                println!("Kid mode set to {on}: {}", result);
                Ok(())
            }
            AccountCommand::Timeline { since, nb } => {
                let query = timeline::GetQuery { since, nb };
                let timeline = lichess
                    .get_timeline(query)
                    .await
                    .wrap_err("failed to fetch timeline")?;
                output::print(&timeline, json);
                Ok(())
            }
        }
    }
}
