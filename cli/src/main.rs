use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::external_engine::{self, *};
use lichess_api::model::puzzles::{self, *};
use rand::Rng;
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

#[derive(Debug, Subcommand)]
enum ExternalEngineCommand {
    /// Lists all external engines that have been registered for the user, and the credentials required to use them.
    List,
    /// Registers a new external engine for the user. It can then be selected and used on the analysis board.
    Create(CreateExternalEngineArgs),
    /// Get properties and credentials of an external engine.
    Get { id: String },
    /// Updates the properties of an external engine.
    Update(UpdateExternalEngineArgs),
    /// Unregisters an external engine.
    Delete { id: String },
    /// Request analysis from an external engine.
    Analyse(AnalyseArgs),
    /// Wait for an analysis requests to any of the external engines that have been registered with the given secret.
    AcquireAnalysis {
        provider_secret: String,
        /// Loop and wait for analysis requests
        #[arg(long, short)]
        wait: bool,
    },
}

#[derive(Debug, Parser)]
struct CreateExternalEngineArgs {
    /// Display name of the engine
    #[arg(long, short)]
    name: String,
    /// Maximum number of available threads
    #[arg(long, short = 't')]
    max_threads: u32,
    /// Maximum available hash table size, in MiB
    #[arg(long, short = 'm')]
    max_hash: u32,
    /// Estimated depth of normal search
    #[arg(long, short)]
    default_depth: u8,
    /// Optional list of supported chess variants
    #[arg(long, short)]
    variants: Option<Vec<String>>,
    /// Arbitrary data that the engine provider can use for identification or bookkeeping
    #[arg(long, short)]
    provider_data: Option<String>,
}

#[derive(Debug, Parser)]
struct UpdateExternalEngineArgs {
    // The external engine id
    id: String,
    /// Display name of the engine
    #[arg(long, short)]
    name: String,
    /// Maximum number of available threads
    #[arg(long, short = 't')]
    max_threads: u32,
    /// Maximum available hash table size, in MiB
    #[arg(long, short = 'm')]
    max_hash: u32,
    /// Estimated depth of normal search
    #[arg(long, short)]
    default_depth: u8,
    /// Optional list of supported chess variants
    #[arg(long, short)]
    variants: Option<Vec<String>>,
    /// Arbitrary data that the engine provider can use for identification or bookkeeping
    #[arg(long, short)]
    provider_data: Option<String>,
}

#[derive(Debug, Parser)]
struct AnalyseArgs {
    /// The external engine id
    id: String,
    /// The client secret for the engine
    #[arg(long, short)]
    client_secret: String,
    /// The session id
    #[arg(long, short)]
    session_id: String,
    /// Number of threads to use
    #[arg(long, short = 't')]
    threads: u32,
    /// Hash table size in MiB
    #[arg(long, short = 'm')]
    hash: u32,
    /// Whether to analyse infinitely
    #[arg(long, short)]
    infinite: bool,
    /// Number of principal variations to return
    #[arg(long, short = 'p')]
    pv: u8,
    /// The variant to analyse
    #[arg(long, short)]
    variant: String,
    /// The initial FEN
    #[arg(long, short)]
    fen: String,
    /// The moves to analyse
    #[arg(long, short = 'a')]
    moves: Vec<String>,
}

impl ExternalEngineCommand {
    async fn run(self, lichess: Lichess) -> Result<()> {
        match self {
            ExternalEngineCommand::List => {
                let engines = lichess.list_external_engines().await?;
                println!("{:#?}", engines);
                Ok(())
            }
            ExternalEngineCommand::Get { id } => {
                let request = external_engine::id::GetRequest::new(&id);
                let engine = lichess.get_external_engine(request).await?;
                println!("{:#?}", engine);
                Ok(())
            }
            ExternalEngineCommand::Create(args) => {
                let provider_secret = generate_provider_secret();
                let engine = CreateExternalEngine {
                    name: args.name,
                    max_threads: args.max_threads,
                    max_hash: args.max_hash,
                    default_depth: args.default_depth,
                    variants: args.variants,
                    provider_data: args.provider_data,
                    provider_secret: provider_secret.clone(),
                };
                let request = create::PostRequest::new(engine);
                let engine = lichess.create_external_engine(request).await?;
                println!("{:#?}", engine);
                println!("provider_secret: {}", provider_secret);
                Ok(())
            }
            ExternalEngineCommand::Update(args) => {
                let provider_secret = generate_provider_secret();
                let engine = UpdateExternalEngine {
                    name: args.name,
                    max_threads: args.max_threads,
                    max_hash: args.max_hash,
                    default_depth: args.default_depth,
                    variants: args.variants,
                    provider_data: args.provider_data,
                    provider_secret: provider_secret.clone(),
                };
                let request = update::PutRequest::new(&args.id, engine);
                let engine = lichess.update_external_engine(request).await?;
                println!("{:#?}", engine);
                println!("provider_secret: {}", provider_secret);
                Ok(())
            }
            ExternalEngineCommand::Delete { id } => {
                let request = delete::DeleteRequest::new(&id);
                let ok = lichess.delete_external_engine(request).await?;
                println!("Deleted engine {}. {}", id, ok);
                Ok(())
            }
            ExternalEngineCommand::Analyse(args) => {
                let analysis_request = analyse::AnalysisRequest {
                    client_secret: args.client_secret,
                    work: analyse::ExternalEngineWork {
                        session_id: args.session_id,
                        threads: args.threads,
                        hash: args.hash,
                        infinite: args.infinite,
                        multi_pv: args.pv,
                        variant: args.variant,
                        initial_fen: args.fen,
                        moves: args.moves,
                    },
                };
                let request = analyse::PostRequest::new(&args.id, analysis_request);
                tracing::debug!("{:#?}", request);
                let mut stream = lichess.analyse_with_external_engine(request).await?;
                while let Some(analysis) = stream.next().await {
                    let analysis = analysis?;
                    println!("{:#?}", analysis);
                }
                Ok(())
            }
            ExternalEngineCommand::AcquireAnalysis {
                provider_secret,
                wait,
            } => {
                let acquire_analysis = acquire_analysis::AcquireAnalysis { provider_secret };
                let request = acquire_analysis::PostRequest::new(acquire_analysis);
                let mut analysis = lichess.acquire_analysis_request(request.clone()).await?;
                while wait && analysis.is_none() {
                    tracing::debug!("No analysis request available");
                    analysis = lichess.acquire_analysis_request(request.clone()).await?;
                }
                println!("{:#?}", analysis);
                Ok(())
            }
        }
    }
}

/// Generate a random provider secret for the engine
/// This is used to authenticate the engine with the lichess server
fn generate_provider_secret() -> String {
    let provider_secret = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>();
    provider_secret
}
