mod commands;
mod output;

use clap::builder::Styles;
use clap::builder::styling::AnsiColor;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use commands::{
    AccountCommand, AnalysisCommand, ArenaTournamentsCommand, BoardCommand, BotCommand,
    BroadcastsCommand, BulkPairingsCommand, ChallengesCommand, ExternalEngineCommand, FideCommand,
    GamesCommand, MessagingCommand, OpeningsCommand, PuzzlesCommand, RelationsCommand,
    SimulsCommand, StudiesCommand, SwissTournamentsCommand, TablebaseCommand, TeamsCommand,
    TvCommand, UsersCommand,
};
use lichess_api::client::LichessApi;
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

    /// Print output as pretty-printed JSON instead of Rust debug format
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    Account {
        #[clap(subcommand)]
        command: AccountCommand,
    },
    Analysis {
        #[clap(subcommand)]
        command: AnalysisCommand,
    },
    ArenaTournaments {
        #[clap(subcommand)]
        command: ArenaTournamentsCommand,
    },
    Board {
        #[clap(subcommand)]
        command: BoardCommand,
    },
    Bot {
        #[clap(subcommand)]
        command: BotCommand,
    },
    Broadcasts {
        #[clap(subcommand)]
        command: BroadcastsCommand,
    },
    BulkPairings {
        #[clap(subcommand)]
        command: BulkPairingsCommand,
    },
    Puzzles {
        #[clap(subcommand)]
        command: PuzzlesCommand,
    },
    Engine {
        #[clap(subcommand)]
        command: ExternalEngineCommand,
    },
    Challenges {
        #[clap(subcommand)]
        command: ChallengesCommand,
    },
    Fide {
        #[clap(subcommand)]
        command: FideCommand,
    },
    Games {
        #[clap(subcommand)]
        command: GamesCommand,
    },
    Messaging {
        #[clap(subcommand)]
        command: MessagingCommand,
    },
    Openings {
        #[clap(subcommand)]
        command: OpeningsCommand,
    },
    Relations {
        #[clap(subcommand)]
        command: RelationsCommand,
    },
    Simuls {
        #[clap(subcommand)]
        command: SimulsCommand,
    },
    Studies {
        #[clap(subcommand)]
        command: StudiesCommand,
    },
    SwissTournaments {
        #[clap(subcommand)]
        command: SwissTournamentsCommand,
    },
    Tablebase {
        #[clap(subcommand)]
        command: TablebaseCommand,
    },
    Teams {
        #[clap(subcommand)]
        command: TeamsCommand,
    },
    Tv {
        #[clap(subcommand)]
        command: TvCommand,
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

fn main() -> Result<()> {
    // clap's derive-generated `Command` graph for this many nested subcommands is deep enough to
    // overflow the default 1 MiB main-thread stack on Windows (Linux/macOS default to 8 MiB), so
    // run everything on a thread with an explicitly larger stack.
    std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(run)
        .expect("failed to spawn main thread")
        .join()
        .expect("main thread panicked")
}

fn run() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
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
    })
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
        let json = args.json;
        match args.command {
            Command::Account { command } => command.run(self.lichess, json).await,
            Command::Analysis { command } => command.run(self.lichess, json).await,
            Command::ArenaTournaments { command } => command.run(self.lichess, json).await,
            Command::Board { command } => command.run(self.lichess, json).await,
            Command::Bot { command } => command.run(self.lichess, json).await,
            Command::Broadcasts { command } => command.run(self.lichess, json).await,
            Command::BulkPairings { command } => command.run(self.lichess, json).await,
            Command::Puzzles { command } => command.run(self.lichess, json).await,
            Command::Engine { command } => command.run(self.lichess, json).await,
            Command::Challenges { command } => command.run(self.lichess, json).await,
            Command::Fide { command } => command.run(self.lichess, json).await,
            Command::Games { command } => command.run(self.lichess, json).await,
            Command::Messaging { command } => command.run(self.lichess, json).await,
            Command::Openings { command } => command.run(self.lichess, json).await,
            Command::Relations { command } => command.run(self.lichess, json).await,
            Command::Simuls { command } => command.run(self.lichess, json).await,
            Command::Studies { command } => command.run(self.lichess, json).await,
            Command::SwissTournaments { command } => command.run(self.lichess, json).await,
            Command::Tablebase { command } => command.run(self.lichess, json).await,
            Command::Teams { command } => command.run(self.lichess, json).await,
            Command::Tv { command } => command.run(self.lichess, json).await,
            Command::Users { command } => command.run(self.lichess, json).await,
        }
    }
}
