use clap::Subcommand;
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use lichess_api::client::LichessApi;
use lichess_api::model::messaging::*;
use reqwest;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Subcommand)]
pub enum MessagingCommand {
    /// Send a private message to a user
    Send {
        /// Username to message
        username: String,
        /// Message text
        text: String,
    },
}

impl MessagingCommand {
    pub async fn run(self, lichess: Lichess, _json: bool) -> Result<()> {
        match self {
            MessagingCommand::Send { username, text } => {
                let request = inbox::PostRequest::new(&username, &text);
                let result = lichess
                    .send_message(request)
                    .await
                    .wrap_err_with(|| format!("failed to send message to '{username}'"))?;
                println!("Message sent to '{username}': {}", result);
                Ok(())
            }
        }
    }
}
