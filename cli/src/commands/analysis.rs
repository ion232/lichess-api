use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use lichess_api::client::LichessApi;
use lichess_api::model::VariantKey;
use lichess_api::model::analysis::*;
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

#[derive(Debug, Subcommand)]
pub enum AnalysisCommand {
    /// Get the cloud evaluation of a position
    CloudEval {
        /// FEN of the position
        fen: String,
        /// Number of principal variations
        #[arg(long)]
        multi_pv: Option<u32>,
        /// Chess variant
        #[arg(long, value_enum)]
        variant: Option<Variant>,
    },
}

impl AnalysisCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            AnalysisCommand::CloudEval {
                fen,
                multi_pv,
                variant,
            } => {
                let query = cloud::GetQuery {
                    fen: fen.clone(),
                    variation_count: multi_pv,
                    variant: variant.map(Into::into),
                };
                let evaluation = lichess
                    .get_cloud_evaluation(query)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to fetch cloud evaluation for fen '{fen}' (position may not be evaluated)"
                        )
                    })?;
                output::print(&evaluation, json);
                Ok(())
            }
        }
    }
}
