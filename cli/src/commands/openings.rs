use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::openings::*;
use lichess_api::model::{Color, VariantKey};
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

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
    FromPosition,
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
            Variant::FromPosition => VariantKey::FromPosition,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OpeningColor {
    White,
    Black,
    Random,
}

impl From<OpeningColor> for Color {
    fn from(color: OpeningColor) -> Self {
        match color {
            OpeningColor::White => Color::White,
            OpeningColor::Black => Color::Black,
            OpeningColor::Random => Color::Random,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum OpeningsCommand {
    /// Query masters games
    Masters {
        /// Starting FEN position
        fen: String,
        /// Comma-separated list of moves to reach the queried position
        play: String,
        /// Only games since this year
        #[arg(long)]
        since: Option<u32>,
        /// Only games until this year
        #[arg(long)]
        until: Option<u32>,
        /// Number of most common moves to look up
        #[arg(long)]
        moves: Option<u32>,
        /// Number of top games to fetch
        #[arg(long)]
        top_games: Option<u32>,
    },
    /// Query rated Lichess games
    Lichess {
        /// Chess variant
        #[arg(long, value_enum, default_value = "standard")]
        variant: Variant,
        /// Starting FEN position
        fen: String,
        /// Comma-separated list of moves to reach the queried position
        play: String,
        /// Comma-separated list of speeds to filter by
        #[arg(long)]
        speeds: Option<String>,
        /// Comma-separated list of rating groups to filter by
        #[arg(long)]
        ratings: Option<String>,
        /// Only games since this month, e.g. "2020-01"
        #[arg(long)]
        since: Option<String>,
        /// Only games until this month, e.g. "2023-12"
        #[arg(long)]
        until: Option<String>,
        /// Number of most common moves to look up
        #[arg(long)]
        moves: Option<u32>,
        /// Number of top games to fetch
        #[arg(long)]
        top_games: Option<u32>,
        /// Number of recent games to fetch
        #[arg(long)]
        recent_games: Option<u32>,
        /// Include the move history by month
        #[arg(long)]
        history: Option<bool>,
    },
    /// Query a specific player's games
    Player {
        /// Username
        player: String,
        /// Starting FEN position
        fen: String,
        /// Which color the player played
        #[arg(value_enum)]
        color: OpeningColor,
        /// Comma-separated list of moves to reach the queried position
        play: String,
        /// Chess variant
        #[arg(long, value_enum, default_value = "standard")]
        variant: Variant,
        /// Comma-separated list of speeds to filter by
        #[arg(long)]
        speeds: Option<String>,
        /// Comma-separated list of game modes to filter by
        #[arg(long)]
        modes: Option<String>,
        /// Only games since this month, e.g. "2020-01"
        #[arg(long)]
        since: Option<String>,
        /// Only games until this month, e.g. "2023-12"
        #[arg(long)]
        until: Option<String>,
        /// Number of most common moves to look up
        #[arg(long)]
        moves: Option<u32>,
        /// Number of recent games to fetch
        #[arg(long)]
        recent_games: Option<u32>,
    },
    /// Fetch a masters game's PGN by ID
    Otb {
        /// Masters game ID
        game_id: String,
    },
}

impl OpeningsCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            OpeningsCommand::Masters {
                fen,
                play,
                since,
                until,
                moves,
                top_games,
            } => {
                let query = masters::GetQuery {
                    fen,
                    play,
                    since,
                    until,
                    moves,
                    top_games,
                };
                let result = lichess
                    .openings_masters(query)
                    .await
                    .wrap_err("failed to query masters opening explorer")?;
                output::print(&result, json);
                Ok(())
            }
            OpeningsCommand::Lichess {
                variant,
                fen,
                play,
                speeds,
                ratings,
                since,
                until,
                moves,
                top_games,
                recent_games,
                history,
            } => {
                let query = lichess::GetQuery {
                    variant: variant.into(),
                    fen,
                    play,
                    speeds,
                    ratings,
                    since,
                    until,
                    moves,
                    top_games,
                    recent_games,
                    history,
                };
                let result = lichess
                    .openings_lichess(query)
                    .await
                    .wrap_err("failed to query lichess opening explorer")?;
                output::print(&result, json);
                Ok(())
            }
            OpeningsCommand::Player {
                player,
                fen,
                color,
                play,
                variant,
                speeds,
                modes,
                since,
                until,
                moves,
                recent_games,
            } => {
                let query = player::GetQuery {
                    player,
                    fen,
                    color: color.into(),
                    play,
                    variant: variant.into(),
                    speeds,
                    modes,
                    since,
                    until,
                    moves,
                    recent_games,
                };
                let result = lichess
                    .openings_player(query)
                    .await
                    .wrap_err("failed to query player opening explorer")?;
                output::print(&result, json);
                Ok(())
            }
            OpeningsCommand::Otb { game_id } => {
                let mut stream = lichess
                    .openings_otb(game_id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to fetch masters game '{game_id}'"))?;
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.wrap_err("failed to read pgn stream")?;
                    println!("{chunk}");
                }
                Ok(())
            }
        }
    }
}
