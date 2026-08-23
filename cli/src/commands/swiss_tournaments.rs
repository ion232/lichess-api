use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::VariantKey;
use lichess_api::model::swiss_tournaments::*;
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
pub enum SwissTournamentsCommand {
    /// Create a new swiss tournament for a team
    Create {
        /// Team ID that hosts the tournament
        team_id: String,
        /// Tournament name
        #[arg(long)]
        name: Option<String>,
        /// Clock limit in seconds
        #[arg(long)]
        clock_limit: u32,
        /// Clock increment in seconds
        #[arg(long)]
        clock_increment: u32,
        /// Number of rounds to play
        #[arg(long)]
        nb_rounds: u32,
        /// Start date as a unix timestamp in milliseconds
        #[arg(long)]
        starts_at: Option<i64>,
        /// Interval between rounds in seconds
        #[arg(long)]
        round_interval: Option<i32>,
        /// Chess variant
        #[arg(long, value_enum)]
        variant: Option<Variant>,
        /// Custom starting position (FEN)
        #[arg(long)]
        position: Option<String>,
        /// Tournament description
        #[arg(long)]
        description: Option<String>,
        /// Whether the tournament is rated
        #[arg(long)]
        rated: Option<bool>,
        /// Password to join the tournament
        #[arg(long)]
        password: Option<String>,
        /// Comma-separated list of usernames who should not play each other
        #[arg(long)]
        forbidden_pairings: Option<String>,
        /// Manual pairings for the next round
        #[arg(long)]
        manual_pairings: Option<String>,
        /// Who can read/write the chat (0 = nobody, 10 = players, 20 = everybody)
        #[arg(long)]
        chat_for: Option<u8>,
        /// Minimum rating to join
        #[arg(long)]
        conditions_min_rating: Option<u32>,
        /// Maximum rating to join
        #[arg(long)]
        conditions_max_rating: Option<u32>,
        /// Minimum number of rated games required to join
        #[arg(long)]
        conditions_nb_rated_game: Option<u32>,
        /// Whether players must play all their games
        #[arg(long)]
        conditions_play_your_games: Option<bool>,
        /// Comma-separated list of usernames always allowed to join
        #[arg(long)]
        conditions_allow_list: Option<String>,
    },
    /// Get info about a swiss tournament
    Get {
        /// Tournament ID
        id: String,
    },
    /// Update a swiss tournament
    Update {
        /// Tournament ID
        id: String,
        /// Tournament name
        #[arg(long)]
        name: Option<String>,
        /// Clock limit in seconds
        #[arg(long)]
        clock_limit: u32,
        /// Clock increment in seconds
        #[arg(long)]
        clock_increment: u32,
        /// Number of rounds to play
        #[arg(long)]
        nb_rounds: u32,
        /// Start date as a unix timestamp in milliseconds
        #[arg(long)]
        starts_at: Option<i64>,
        /// Interval between rounds in seconds
        #[arg(long)]
        round_interval: Option<i32>,
        /// Chess variant
        #[arg(long, value_enum)]
        variant: Option<Variant>,
        /// Custom starting position (FEN)
        #[arg(long)]
        position: Option<String>,
        /// Tournament description
        #[arg(long)]
        description: Option<String>,
        /// Whether the tournament is rated
        #[arg(long)]
        rated: Option<bool>,
        /// Password to join the tournament
        #[arg(long)]
        password: Option<String>,
        /// Comma-separated list of usernames who should not play each other
        #[arg(long)]
        forbidden_pairings: Option<String>,
        /// Manual pairings for the next round
        #[arg(long)]
        manual_pairings: Option<String>,
        /// Who can read/write the chat (0 = nobody, 10 = players, 20 = everybody)
        #[arg(long)]
        chat_for: Option<u8>,
        /// Minimum rating to join
        #[arg(long)]
        conditions_min_rating: Option<u32>,
        /// Maximum rating to join
        #[arg(long)]
        conditions_max_rating: Option<u32>,
        /// Minimum number of rated games required to join
        #[arg(long)]
        conditions_nb_rated_game: Option<u32>,
        /// Whether players must play all their games
        #[arg(long)]
        conditions_play_your_games: Option<bool>,
        /// Comma-separated list of usernames always allowed to join
        #[arg(long)]
        conditions_allow_list: Option<String>,
    },
    /// Export games of a swiss tournament
    ExportGames {
        /// Tournament ID
        id: String,
        /// Only games of this player
        #[arg(long)]
        player: Option<String>,
        /// Include the PGN moves
        #[arg(long)]
        moves: Option<bool>,
        /// Include the PGN moves as a JSON array
        #[arg(long)]
        pgn_in_json: Option<bool>,
        /// Include the PGN tags
        #[arg(long)]
        tags: Option<bool>,
        /// Include clock comments
        #[arg(long)]
        clocks: Option<bool>,
        /// Include analysis evaluations
        #[arg(long)]
        evals: Option<bool>,
        /// Include weighted error values
        #[arg(long)]
        accuracy: Option<bool>,
        /// Include the opening name
        #[arg(long)]
        opening: Option<bool>,
        /// Include the division of the game into opening/middlegame/endgame
        #[arg(long)]
        division: Option<bool>,
    },
    /// Join a swiss tournament
    Join {
        /// Tournament ID
        id: String,
        /// Password to join, if required
        #[arg(long)]
        password: Option<String>,
    },
    /// Get the results of a swiss tournament
    Results {
        /// Tournament ID
        id: String,
        /// Max number of results to fetch
        #[arg(long)]
        nb: Option<u32>,
    },
    /// Manually schedule the next round
    ScheduleNextRound {
        /// Tournament ID
        id: String,
        /// Date to schedule the round for, as a unix timestamp in milliseconds
        #[arg(long)]
        date: i64,
    },
    /// Terminate a swiss tournament
    Terminate {
        /// Tournament ID
        id: String,
    },
    /// Pause or leave a swiss tournament
    Withdraw {
        /// Tournament ID
        id: String,
    },
    /// Export a swiss tournament in the Tournament Report File format
    Trf {
        /// Tournament ID
        id: String,
    },
}

impl SwissTournamentsCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            SwissTournamentsCommand::Create {
                team_id,
                name,
                clock_limit,
                clock_increment,
                nb_rounds,
                starts_at,
                round_interval,
                variant,
                position,
                description,
                rated,
                password,
                forbidden_pairings,
                manual_pairings,
                chat_for,
                conditions_min_rating,
                conditions_max_rating,
                conditions_nb_rated_game,
                conditions_play_your_games,
                conditions_allow_list,
            } => {
                let form = create::CreateSwissTournamentForm {
                    name,
                    clock_limit,
                    clock_increment,
                    nb_rounds,
                    starts_at,
                    round_interval,
                    variant: variant.map(|v| v.into()),
                    position,
                    description,
                    rated,
                    password,
                    forbidden_pairings,
                    manual_pairings,
                    chat_for,
                    conditions_min_rating,
                    conditions_max_rating,
                    conditions_nb_rated_game,
                    conditions_play_your_games,
                    conditions_allow_list,
                };
                let tournament = lichess
                    .create_swiss_tournament(&team_id, form)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to create swiss tournament for team '{team_id}'")
                    })?;
                output::print(&tournament, json);
                Ok(())
            }
            SwissTournamentsCommand::Get { id } => {
                let tournament = lichess
                    .get_swiss_tournament(id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to fetch swiss tournament '{id}'"))?;
                output::print(&tournament, json);
                Ok(())
            }
            SwissTournamentsCommand::Update {
                id,
                name,
                clock_limit,
                clock_increment,
                nb_rounds,
                starts_at,
                round_interval,
                variant,
                position,
                description,
                rated,
                password,
                forbidden_pairings,
                manual_pairings,
                chat_for,
                conditions_min_rating,
                conditions_max_rating,
                conditions_nb_rated_game,
                conditions_play_your_games,
                conditions_allow_list,
            } => {
                let form = update::UpdateSwissTournamentForm {
                    name,
                    clock_limit,
                    clock_increment,
                    nb_rounds,
                    starts_at,
                    round_interval,
                    variant: variant.map(|v| v.into()),
                    position,
                    description,
                    rated,
                    password,
                    forbidden_pairings,
                    manual_pairings,
                    chat_for,
                    conditions_min_rating,
                    conditions_max_rating,
                    conditions_nb_rated_game,
                    conditions_play_your_games,
                    conditions_allow_list,
                };
                let tournament = lichess
                    .update_swiss_tournament(&id, form)
                    .await
                    .wrap_err_with(|| format!("failed to update swiss tournament '{id}'"))?;
                output::print(&tournament, json);
                Ok(())
            }
            SwissTournamentsCommand::ExportGames {
                id,
                player,
                moves,
                pgn_in_json,
                tags,
                clocks,
                evals,
                accuracy,
                opening,
                division,
            } => {
                let query = games::GetQuery {
                    player,
                    moves,
                    pgn_in_json,
                    tags,
                    clocks,
                    evals,
                    accuracy,
                    opening,
                    division,
                };
                let mut stream = lichess
                    .export_swiss_tournament_games(&id, query)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to export games for swiss tournament '{id}'")
                    })?;
                while let Some(game) = stream.next().await {
                    match game {
                        Ok(game) => output::print(&game, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            SwissTournamentsCommand::Join { id, password } => {
                let form = join::JoinSwissTournamentForm { password };
                let result = lichess
                    .join_swiss_tournament(&id, form)
                    .await
                    .wrap_err_with(|| format!("failed to join swiss tournament '{id}'"))?;
                println!("Joined tournament: {}", result);
                Ok(())
            }
            SwissTournamentsCommand::Results { id, nb } => {
                let query = results::GetQuery { nb };
                let mut stream = lichess
                    .get_swiss_tournament_results(&id, query)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch results for swiss tournament '{id}'")
                    })?;
                while let Some(result) = stream.next().await {
                    match result {
                        Ok(result) => output::print(&result, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            SwissTournamentsCommand::ScheduleNextRound { id, date } => {
                let form = schedule_next_round::ScheduleNextRoundForm { date };
                lichess
                    .schedule_next_swiss_round(&id, form)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to schedule next round for swiss tournament '{id}'")
                    })?;
                println!("Next round scheduled for tournament '{}'", id);
                Ok(())
            }
            SwissTournamentsCommand::Terminate { id } => {
                let result = lichess
                    .terminate_swiss_tournament(id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to terminate swiss tournament '{id}'"))?;
                println!("Tournament terminated: {}", result);
                Ok(())
            }
            SwissTournamentsCommand::Withdraw { id } => {
                let result = lichess
                    .withdraw_from_swiss_tournament(id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to withdraw from swiss tournament '{id}'"))?;
                println!("Withdrawn from tournament: {}", result);
                Ok(())
            }
            SwissTournamentsCommand::Trf { id } => {
                let trf = lichess
                    .get_swiss_tournament_trf(id.as_str())
                    .await
                    .wrap_err_with(|| {
                        format!("failed to export trf for swiss tournament '{id}'")
                    })?;
                println!("{trf}");
                Ok(())
            }
        }
    }
}
