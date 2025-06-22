mod commands;

use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use commands::{ChallengesCommand, ExternalEngineCommand, PuzzleCommand, UsersCommand};
use lichess_api::client::LichessApi;
use reqwest;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

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

    /// Enable verbose logging
    #[arg(long, short)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    Puzzle {
        #[clap(subcommand)]
        command: PuzzleCommand,
    },
    Engine {
        #[clap(subcommand)]
        command: ExternalEngineCommand,
    },
    Challenges {
        #[clap(subcommand)]
        command: ChallengesCommand,
    },
    Users {
        #[clap(subcommand)]
        command: UsersCommand,
    },
}

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug)]
struct App {
    lichess: Lichess,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let level = if args.verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };
    init_tracing(level)?;
    color_eyre::install()?;
    let app = App::new(args.api_token.clone());
    app.run(args).await
}

fn init_tracing(directive: LevelFilter) -> Result<()> {
    let filter = EnvFilter::builder()
        .from_env()?
        .add_directive(directive.into())
        // remove hyper noise
        .add_directive("hyper::proto=info".parse()?);
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();
    Ok(())
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
            Command::Engine { command } => command.run(self.lichess).await,
            Command::Challenges { command } => command.run(self.lichess).await,
            Command::Users { command } => command.run(self.lichess).await,
        }
    }
}
