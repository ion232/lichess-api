use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::puzzles::*;
use reqwest;

const HELP_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Blue.on_default().bold())
    .usage(AnsiColor::Blue.on_default().bold())
    .literal(AnsiColor::White.on_default())
    .placeholder(AnsiColor::Green.on_default());

#[derive(Debug, Parser)]
#[command(author, version, about, styles = HELP_STYLES)]
struct Cli {
    /// A personal API token for lichess (https://lichess.org/account/oauth/token)
    #[arg(long, short)]
    api_token: Option<String>,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Puzzle {
        #[clap(subcommand)]
        command: PuzzleCommand,
    },
}

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug)]
struct App {
    lichess: Lichess,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();
    let app = App::new(args.api_token.clone());
    app.run(args).await
}

impl App {
    pub fn new(api_token: Option<String>) -> Self {
        let client = reqwest::ClientBuilder::new().build().unwrap();
        let api = LichessApi::new(client, api_token);
        Self { lichess: api }
    }

    async fn run(self, args: Cli) -> Result<()> {
        match args.command {
            Command::Puzzle { command } => command.run(self.lichess).await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum PuzzleCommand {
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
    async fn run(self, lichess: Lichess) -> Result<()> {
        match self {
            PuzzleCommand::Daily => {
                let request = daily::GetRequest::new();
                let puzzle = lichess.get_daily_puzzle(request).await?;
                println!("{puzzle:#?}");
                Ok(())
            }
            PuzzleCommand::Get { id } => {
                let request = id::GetRequest::new(&id);
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
