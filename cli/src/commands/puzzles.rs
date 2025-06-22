use clap::Subcommand;
use color_eyre::Result;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::puzzles::{self, *};
use reqwest;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Subcommand)]
pub enum PuzzleCommand {
    /// Get the daily puzzle
    Daily,
    /// Get a puzzle by its ID
    Get { id: String },
    /// Get your puzzle activity
    Activity { max_rounds: Option<u32> },
    /// Get your puzzle dashboard
    Dashboard { days: Option<u32> },
    /// Get the storm dashboard of a player
    Storm { username: String, days: Option<u32> },
}

impl PuzzleCommand {
    pub async fn run(self, lichess: Lichess) -> Result<()> {
        match self {
            PuzzleCommand::Daily => {
                let puzzle = lichess.get_daily_puzzle().await?;
                println!("{puzzle:#?}");
                Ok(())
            }
            PuzzleCommand::Get { id } => {
                let request = puzzles::id::GetRequest::new(&id);
                let puzzle = lichess.get_puzzle(request).await?;
                println!("{puzzle:#?}");
                Ok(())
            }
            PuzzleCommand::Activity { max_rounds } => {
                let request = activity::GetRequest::new(max_rounds);
                let mut stream = lichess.get_puzzle_activity(request).await?;
                while let Some(round) = stream.next().await {
                    let round = round?;
                    println!("Round: {round:#?}");
                }
                Ok(())
            }
            PuzzleCommand::Dashboard { days } => {
                let request = dashboard::GetRequest::new(days.unwrap_or(30));
                let dashboard = lichess.get_puzzle_dashboard(request).await?;
                println!("{dashboard:#?}");
                Ok(())
            }
            PuzzleCommand::Storm { username, days } => {
                let request = storm_dashboard::GetRequest::new(&username, days);
                let dashboard = lichess.get_puzzle_storm_dashboard(request).await?;
                println!("{dashboard:#?}");
                Ok(())
            }
        }
    }
}