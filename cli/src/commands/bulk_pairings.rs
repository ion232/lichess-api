use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::bulk_pairings::*;
use lichess_api::model::{Days, VariantKey};
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Clone, ValueEnum)]
pub enum CorrespondenceDays {
    One,
    Two,
    Three,
    Five,
    Seven,
    Ten,
    Fourteen,
}

impl From<CorrespondenceDays> for Days {
    fn from(days: CorrespondenceDays) -> Self {
        match days {
            CorrespondenceDays::One => Days::One,
            CorrespondenceDays::Two => Days::Two,
            CorrespondenceDays::Three => Days::Three,
            CorrespondenceDays::Five => Days::Five,
            CorrespondenceDays::Seven => Days::Seven,
            CorrespondenceDays::Ten => Days::Ten,
            CorrespondenceDays::Fourteen => Days::Fourteen,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Variant {
    Standard,
    Chess960,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    ThreeCheck,
}

impl From<Variant> for VariantKey {
    fn from(variant: Variant) -> Self {
        match variant {
            Variant::Standard => VariantKey::Standard,
            Variant::Chess960 => VariantKey::Chess960,
            Variant::Crazyhouse => VariantKey::Crazyhouse,
            Variant::Antichess => VariantKey::Antichess,
            Variant::Atomic => VariantKey::Atomic,
            Variant::Horde => VariantKey::Horde,
            Variant::KingOfTheHill => VariantKey::KingOfTheHill,
            Variant::RacingKings => VariantKey::RacingKings,
            Variant::ThreeCheck => VariantKey::ThreeCheck,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum BulkPairingsCommand {
    /// View your bulk pairings
    List,
    /// Create a bulk pairing
    Create {
        /// Comma-separated list of paired player tokens, e.g. "token1:token2,token3:token4"
        #[arg(long)]
        players: String,
        /// Clock limit in seconds
        #[arg(long)]
        clock_limit: Option<u32>,
        /// Clock increment in seconds
        #[arg(long)]
        clock_increment: Option<u32>,
        /// Days per turn for correspondence games
        #[arg(long, value_enum)]
        days: Option<CorrespondenceDays>,
        /// Unix timestamp (seconds) at which to schedule the pairings
        #[arg(long)]
        pair_at: Option<i64>,
        /// Unix timestamp (seconds) at which to start the clocks
        #[arg(long)]
        start_clocks_at: Option<i64>,
        /// Whether the games are rated
        #[arg(long)]
        rated: bool,
        /// Chess variant
        #[arg(long, value_enum, default_value = "standard")]
        variant: Variant,
        /// Custom starting position (FEN)
        #[arg(long)]
        fen: Option<String>,
        /// Message sent to each player, templated
        #[arg(long)]
        message: Option<String>,
        /// Extra game rules
        #[arg(long)]
        rules: Option<String>,
    },
    /// Show a bulk pairing
    Get {
        /// Bulk pairing ID
        id: String,
    },
    /// Cancel a bulk pairing
    Cancel {
        /// Bulk pairing ID
        id: String,
    },
    /// Export games of a bulk pairing
    ExportGames {
        /// Bulk pairing ID
        id: String,
        /// Include the PGN moves
        #[arg(long)]
        moves: bool,
        /// Include clock comments in the PGN moves
        #[arg(long)]
        clocks: bool,
        /// Include analysis evaluation comments in the PGN moves
        #[arg(long)]
        evals: bool,
        /// Include the opening name
        #[arg(long)]
        opening: bool,
    },
    /// Manually start the clocks of a bulk pairing
    StartClocks {
        /// Bulk pairing ID
        id: String,
    },
}

impl BulkPairingsCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            BulkPairingsCommand::List => {
                let pairings = lichess
                    .get_bulk_pairings()
                    .await
                    .wrap_err("failed to fetch bulk pairings")?;
                output::print(&pairings, json);
                Ok(())
            }
            BulkPairingsCommand::Create {
                players,
                clock_limit,
                clock_increment,
                days,
                pair_at,
                start_clocks_at,
                rated,
                variant,
                fen,
                message,
                rules,
            } => {
                let form = create::CreateBulkPairingForm {
                    players,
                    clock_limit,
                    clock_increment,
                    days: days.map(|d| d.into()),
                    pair_at,
                    start_clocks_at,
                    rated: Some(rated),
                    variant: Some(variant.into()),
                    fen,
                    message,
                    rules,
                };
                let pairing = lichess
                    .create_bulk_pairing(form)
                    .await
                    .wrap_err("failed to create bulk pairing")?;
                output::print(&pairing, json);
                Ok(())
            }
            BulkPairingsCommand::Get { id } => {
                let request = show::GetRequest::new(&id);
                let pairing = lichess
                    .get_bulk_pairing(request)
                    .await
                    .wrap_err_with(|| format!("failed to fetch bulk pairing '{id}'"))?;
                output::print(&pairing, json);
                Ok(())
            }
            BulkPairingsCommand::Cancel { id } => {
                let request = remove::DeleteRequest::new(&id);
                let result = lichess
                    .cancel_bulk_pairing(request)
                    .await
                    .wrap_err_with(|| format!("failed to cancel bulk pairing '{id}'"))?;
                println!("Bulk pairing cancelled: {}", result);
                Ok(())
            }
            BulkPairingsCommand::ExportGames {
                id,
                moves,
                clocks,
                evals,
                opening,
            } => {
                let query = games::GetQuery {
                    moves: Some(moves),
                    pgn_in_json: None,
                    tags: None,
                    clocks: Some(clocks),
                    evals: Some(evals),
                    accuracy: None,
                    opening: Some(opening),
                    division: None,
                    literate: None,
                };
                let mut stream = lichess
                    .export_bulk_pairing_games(&id, query)
                    .await
                    .wrap_err_with(|| format!("failed to export games for bulk pairing '{id}'"))?;
                while let Some(game) = stream.next().await {
                    let game = game.wrap_err("failed to read exported game")?;
                    output::print(&game, json);
                }
                Ok(())
            }
            BulkPairingsCommand::StartClocks { id } => {
                let request = start_clocks::PostRequest::new(&id);
                let result = lichess
                    .start_bulk_pairing_clocks(request)
                    .await
                    .wrap_err_with(|| format!("failed to start clocks for bulk pairing '{id}'"))?;
                println!("Clocks started: {}", result);
                Ok(())
            }
        }
    }
}
