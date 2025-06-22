use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::puzzles::{self, *};
use reqwest;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Clone, ValueEnum)]
pub enum Difficulty {
    Easiest,
    Easier,
    Normal,
    Harder,
    Hardest,
}

impl From<Difficulty> for next::Difficulty {
    fn from(d: Difficulty) -> Self {
        match d {
            Difficulty::Easiest => next::Difficulty::Easiest,
            Difficulty::Easier => next::Difficulty::Easier,
            Difficulty::Normal => next::Difficulty::Normal,
            Difficulty::Harder => next::Difficulty::Harder,
            Difficulty::Hardest => next::Difficulty::Hardest,
        }
    }
}

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
    /// Get a new random puzzle
    Next {
        /// Filter puzzles by theme/angle
        #[arg(long)]
        angle: Option<String>,
        /// Puzzle difficulty relative to your rating
        #[arg(long, value_enum)]
        difficulty: Option<Difficulty>,
    },
    /// Get puzzles to replay for a specific theme
    Replay {
        /// Number of days to look back (e.g., 30)
        days: u32,
        /// Theme to filter puzzles (e.g., "mix", "endgame")
        theme: String,
    },
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
            PuzzleCommand::Next { angle, difficulty } => {
                let request = next::GetRequest::new(angle, difficulty.map(|d| d.into()));
                let puzzle = lichess.get_new_puzzle(request).await?;
                println!("{puzzle:#?}");
                Ok(())
            }
            PuzzleCommand::Replay { days, theme } => {
                let request = replay::GetRequest::new(days, &theme);
                let replay = lichess.get_puzzles_to_replay(request).await?;
                println!("{replay:#?}");
                Ok(())
            }
        }
    }
}