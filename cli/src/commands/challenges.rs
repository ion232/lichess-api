use clap::{Parser, Subcommand};
use color_eyre::Result;
use lichess_api::client::LichessApi;
use lichess_api::model::VariantKey;
use lichess_api::model::challenges::*;
use reqwest;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Subcommand)]
pub enum ChallengesCommand {
    /// List your challenges
    List,
    /// Create a challenge
    Create(CreateChallengeArgs),
    /// Accept a challenge
    Accept {
        /// Challenge ID
        challenge_id: String,
    },
    /// Decline a challenge
    Decline {
        /// Challenge ID
        challenge_id: String,
        /// Reason for declining
        #[arg(long, value_enum)]
        reason: Option<DeclineReason>,
    },
    /// Cancel a challenge you sent
    Cancel {
        /// Challenge ID
        challenge_id: String,
        /// Opponent token (if applicable)
        #[arg(long)]
        opponent_token: Option<String>,
    },
}

#[derive(Debug, Parser)]
pub struct CreateChallengeArgs {
    /// Username to challenge
    username: String,
    /// Whether the game is rated
    #[arg(long)]
    rated: bool,
    /// Clock limit in seconds
    #[arg(long)]
    clock_limit: Option<u32>,
    /// Clock increment in seconds
    #[arg(long)]
    clock_increment: Option<u32>,
    /// Days per turn for correspondence games
    #[arg(long)]
    days: Option<u32>,
    /// Chess variant
    #[arg(long, default_value = "standard")]
    variant: String,
    /// Custom starting position (FEN)
    #[arg(long)]
    fen: Option<String>,
    /// Message to the opponent
    #[arg(long)]
    message: Option<String>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum DeclineReason {
    Generic,
    Later,
    TooFast,
    TooSlow,
    TimeControl,
    Rated,
    Casual,
    Standard,
    Variant,
    NoBot,
    OnlyBot,
}

impl From<DeclineReason> for decline::Reason {
    fn from(reason: DeclineReason) -> Self {
        match reason {
            DeclineReason::Generic => decline::Reason::Generic,
            DeclineReason::Later => decline::Reason::Later,
            DeclineReason::TooFast => decline::Reason::TooFast,
            DeclineReason::TooSlow => decline::Reason::TooSlow,
            DeclineReason::TimeControl => decline::Reason::TimeControl,
            DeclineReason::Rated => decline::Reason::Rated,
            DeclineReason::Casual => decline::Reason::Casual,
            DeclineReason::Standard => decline::Reason::Standard,
            DeclineReason::Variant => decline::Reason::Variant,
            DeclineReason::NoBot => decline::Reason::NoBot,
            DeclineReason::OnlyBot => decline::Reason::OnlyBot,
        }
    }
}

impl ChallengesCommand {
    pub async fn run(self, lichess: Lichess) -> Result<()> {
        match self {
            ChallengesCommand::List => {
                let challenges = lichess.list_challenges().await?;
                println!("Incoming challenges:");
                for challenge in &challenges.r#in {
                    println!("  {} - {}", challenge.base.id, challenge.base.url);
                }
                println!("Outgoing challenges:");
                for challenge in &challenges.out {
                    println!("  {} - {}", challenge.base.id, challenge.base.url);
                }
                Ok(())
            }
            ChallengesCommand::Create(args) => {
                let variant_key = match args.variant.as_str() {
                    "standard" => VariantKey::Standard,
                    "chess960" => VariantKey::Chess960,
                    "crazyhouse" => VariantKey::Crazyhouse,
                    "antichess" => VariantKey::Antichess,
                    "atomic" => VariantKey::Atomic,
                    "horde" => VariantKey::Horde,
                    "kingOfTheHill" => VariantKey::KingOfTheHill,
                    "racingKings" => VariantKey::RacingKings,
                    "threeCheck" => VariantKey::ThreeCheck,
                    _ => {
                        println!("Invalid variant: {}", args.variant);
                        return Ok(());
                    }
                };

                let challenge = CreateChallenge {
                    base: ChallengeBase {
                        clock_limit: args.clock_limit,
                        clock_increment: args.clock_increment,
                        days: args.days.map(|d| d.into()),
                        variant: variant_key,
                        fen: args.fen,
                    },
                    rated: args.rated,
                    keep_alive_stream: false,
                    accept_by_token: None,
                    message: args.message,
                    rules: String::new(),
                };

                let request = create::PostRequest::new(&args.username, challenge);
                let result = lichess.create_challenge(request).await?;
                println!("Challenge created: {:#?}", result);
                Ok(())
            }
            ChallengesCommand::Accept { challenge_id } => {
                let request = accept::PostRequest::new(&challenge_id);
                let result = lichess.accept_challenge(request).await?;
                println!("Challenge accepted: {}", result);
                Ok(())
            }
            ChallengesCommand::Decline {
                challenge_id,
                reason,
            } => {
                let decline_reason = reason.unwrap_or(DeclineReason::Generic);
                let request = decline::PostRequest::new(challenge_id, decline_reason.into());
                let result = lichess.decline_challenge(request).await?;
                println!("Challenge declined: {}", result);
                Ok(())
            }
            ChallengesCommand::Cancel {
                challenge_id,
                opponent_token,
            } => {
                let request = cancel::PostRequest::new(challenge_id, opponent_token);
                let result = lichess.cancel_challenge(request).await?;
                println!("Challenge cancelled: {}", result);
                Ok(())
            }
        }
    }
}
