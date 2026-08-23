use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::puzzles::{self, *};
use reqwest;

use crate::output;

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
pub enum PuzzlesCommand {
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

impl PuzzlesCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            PuzzlesCommand::Daily => {
                let puzzle = lichess
                    .get_daily_puzzle()
                    .await
                    .wrap_err("failed to fetch daily puzzle")?;
                output::print(&puzzle, json);
                Ok(())
            }
            PuzzlesCommand::Get { id } => {
                let request = puzzles::id::GetRequest::new(&id);
                let puzzle = lichess
                    .get_puzzle(request)
                    .await
                    .wrap_err_with(|| format!("failed to fetch puzzle '{id}'"))?;
                output::print(&puzzle, json);
                Ok(())
            }
            PuzzlesCommand::Activity { max_rounds } => {
                let request = activity::GetRequest::new(max_rounds);
                let mut stream = lichess
                    .get_puzzle_activity(request)
                    .await
                    .wrap_err("failed to fetch puzzle activity")?;
                while let Some(round) = stream.next().await {
                    let round = round.wrap_err("failed to read puzzle activity round")?;
                    output::print(&round, json);
                }
                Ok(())
            }
            PuzzlesCommand::Dashboard { days } => {
                let request = dashboard::GetRequest::new(days.unwrap_or(30));
                let dashboard = lichess
                    .get_puzzle_dashboard(request)
                    .await
                    .wrap_err("failed to fetch puzzle dashboard")?;
                output::print(&dashboard, json);
                Ok(())
            }
            PuzzlesCommand::Storm { username, days } => {
                let request = storm_dashboard::GetRequest::new(&username, days);
                let dashboard = lichess
                    .get_puzzle_storm_dashboard(request)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch storm dashboard for '{username}'")
                    })?;
                output::print(&dashboard, json);
                Ok(())
            }
            PuzzlesCommand::Next { angle, difficulty } => {
                let request = next::GetRequest::new(angle, difficulty.map(|d| d.into()));
                let puzzle = lichess
                    .get_new_puzzle(request)
                    .await
                    .wrap_err("failed to fetch new puzzle")?;
                output::print(&puzzle, json);
                Ok(())
            }
            PuzzlesCommand::Replay { days, theme } => {
                let request = replay::GetRequest::new(days, &theme);
                let replay = lichess
                    .get_puzzles_to_replay(request)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch puzzles to replay for theme '{theme}'")
                    })?;
                output::print(&replay, json);
                Ok(())
            }
        }
    }
}
