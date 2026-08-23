use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::VariantKey;
use lichess_api::model::arena_tournaments::*;
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
pub enum ArenaTournamentsCommand {
    /// Get currently created, started, and finished arena tournaments
    Current,
    /// Create a new arena tournament
    Create {
        /// Tournament name
        #[arg(long)]
        name: Option<String>,
        /// Clock time in minutes
        #[arg(long)]
        clock_time: f64,
        /// Clock increment in seconds
        #[arg(long)]
        clock_increment: u32,
        /// Duration of the tournament in minutes
        #[arg(long)]
        minutes: u32,
        /// Minutes to wait before starting the tournament
        #[arg(long)]
        wait_minutes: Option<u32>,
        /// Start date as a unix timestamp in milliseconds
        #[arg(long)]
        start_date: Option<i64>,
        /// Chess variant
        #[arg(long, value_enum)]
        variant: Option<Variant>,
        /// Whether the tournament is rated
        #[arg(long)]
        rated: Option<bool>,
        /// Custom starting position (FEN)
        #[arg(long)]
        position: Option<String>,
        /// Whether berserk is allowed
        #[arg(long)]
        berserkable: Option<bool>,
        /// Whether streaks are allowed
        #[arg(long)]
        streakable: Option<bool>,
        /// Whether a chat is enabled
        #[arg(long)]
        has_chat: Option<bool>,
        /// Tournament description
        #[arg(long)]
        description: Option<String>,
        /// Password to join the tournament
        #[arg(long)]
        password: Option<String>,
        /// Team ID that hosts a team battle
        #[arg(long)]
        team_battle_by_team: Option<String>,
        /// Restrict entry to members of this team
        #[arg(long)]
        conditions_team_member_team_id: Option<String>,
        /// Minimum rating to join
        #[arg(long)]
        conditions_min_rating: Option<u32>,
        /// Maximum rating to join
        #[arg(long)]
        conditions_max_rating: Option<u32>,
        /// Minimum number of rated games required to join
        #[arg(long)]
        conditions_nb_rated_game: Option<u32>,
        /// Comma-separated list of usernames always allowed to join
        #[arg(long)]
        conditions_allow_list: Option<String>,
        /// Whether bots are allowed to join
        #[arg(long)]
        conditions_bots: Option<bool>,
        /// Minimum account age in days required to join
        #[arg(long)]
        conditions_account_age: Option<u32>,
    },
    /// Get info about an arena tournament
    Get {
        /// Tournament ID
        id: String,
        /// Standings page to include
        #[arg(long)]
        page: Option<u32>,
    },
    /// Update an arena tournament
    Update {
        /// Tournament ID
        id: String,
        /// Tournament name
        #[arg(long)]
        name: Option<String>,
        /// Clock time in minutes
        #[arg(long)]
        clock_time: f64,
        /// Clock increment in seconds
        #[arg(long)]
        clock_increment: u32,
        /// Duration of the tournament in minutes
        #[arg(long)]
        minutes: u32,
        /// Minutes to wait before starting the tournament
        #[arg(long)]
        wait_minutes: Option<u32>,
        /// Start date as a unix timestamp in milliseconds
        #[arg(long)]
        start_date: Option<i64>,
        /// Chess variant
        #[arg(long, value_enum)]
        variant: Option<Variant>,
        /// Whether the tournament is rated
        #[arg(long)]
        rated: Option<bool>,
        /// Custom starting position (FEN)
        #[arg(long)]
        position: Option<String>,
        /// Whether berserk is allowed
        #[arg(long)]
        berserkable: Option<bool>,
        /// Whether streaks are allowed
        #[arg(long)]
        streakable: Option<bool>,
        /// Whether a chat is enabled
        #[arg(long)]
        has_chat: Option<bool>,
        /// Tournament description
        #[arg(long)]
        description: Option<String>,
        /// Password to join the tournament
        #[arg(long)]
        password: Option<String>,
        /// Minimum rating to join
        #[arg(long)]
        conditions_min_rating: Option<u32>,
        /// Maximum rating to join
        #[arg(long)]
        conditions_max_rating: Option<u32>,
        /// Minimum number of rated games required to join
        #[arg(long)]
        conditions_nb_rated_game: Option<u32>,
        /// Comma-separated list of usernames always allowed to join
        #[arg(long)]
        conditions_allow_list: Option<String>,
        /// Whether bots are allowed to join
        #[arg(long)]
        conditions_bots: Option<bool>,
        /// Minimum account age in days required to join
        #[arg(long)]
        conditions_account_age: Option<u32>,
    },
    /// Export games of an arena tournament
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
    /// Join an arena tournament
    Join {
        /// Tournament ID
        id: String,
        /// Password to join, if required
        #[arg(long)]
        password: Option<String>,
        /// Team ID to join with, for team battles
        #[arg(long)]
        team: Option<String>,
        /// Whether to be paired as soon as possible
        #[arg(long)]
        pair_me_asap: Option<bool>,
    },
    /// Get the results of an arena tournament
    Results {
        /// Tournament ID
        id: String,
        /// Max number of results to fetch
        #[arg(long)]
        nb: Option<u32>,
        /// Include the score sheet
        #[arg(long)]
        sheet: Option<bool>,
    },
    /// Get the team standing of a team battle
    Teams {
        /// Tournament ID
        id: String,
    },
    /// Terminate an arena tournament
    Terminate {
        /// Tournament ID
        id: String,
    },
    /// Pause or leave an arena tournament
    Withdraw {
        /// Tournament ID
        id: String,
    },
    /// Update a team battle
    UpdateTeamBattle {
        /// Tournament ID
        id: String,
        /// Comma-separated list of team IDs
        #[arg(long)]
        teams: String,
        /// Number of leaders per team
        #[arg(long)]
        nb_leaders: u32,
    },
    /// Get tournaments created by a user
    CreatedByUser {
        /// Username
        username: String,
        /// Max number of tournaments to fetch
        #[arg(long)]
        nb: Option<u32>,
        /// Filter by status (10 = created, 20 = started, 30 = finished)
        #[arg(long)]
        status: Option<u8>,
    },
    /// Get tournaments played by a user
    PlayedByUser {
        /// Username
        username: String,
        /// Max number of tournaments to fetch
        #[arg(long)]
        nb: Option<u32>,
        /// Include performance rating
        #[arg(long)]
        performance: Option<bool>,
    },
}

impl ArenaTournamentsCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            ArenaTournamentsCommand::Current => {
                let tournaments = lichess
                    .get_current_arena_tournaments()
                    .await
                    .wrap_err("failed to fetch current arena tournaments")?;
                output::print(&tournaments, json);
                Ok(())
            }
            ArenaTournamentsCommand::Create {
                name,
                clock_time,
                clock_increment,
                minutes,
                wait_minutes,
                start_date,
                variant,
                rated,
                position,
                berserkable,
                streakable,
                has_chat,
                description,
                password,
                team_battle_by_team,
                conditions_team_member_team_id,
                conditions_min_rating,
                conditions_max_rating,
                conditions_nb_rated_game,
                conditions_allow_list,
                conditions_bots,
                conditions_account_age,
            } => {
                let form = create::CreateArenaTournamentForm {
                    name,
                    clock_time,
                    clock_increment,
                    minutes,
                    wait_minutes,
                    start_date,
                    variant: variant.map(|v| v.into()),
                    rated,
                    position,
                    berserkable,
                    streakable,
                    has_chat,
                    description,
                    password,
                    team_battle_by_team,
                    conditions_team_member_team_id,
                    conditions_min_rating,
                    conditions_max_rating,
                    conditions_nb_rated_game,
                    conditions_allow_list,
                    conditions_bots,
                    conditions_account_age,
                };
                let tournament = lichess
                    .create_arena_tournament(form)
                    .await
                    .wrap_err("failed to create arena tournament")?;
                output::print(&tournament, json);
                Ok(())
            }
            ArenaTournamentsCommand::Get { id, page } => {
                let query = show::GetQuery { page };
                let tournament = lichess
                    .get_arena_tournament(&id, query)
                    .await
                    .wrap_err_with(|| format!("failed to fetch arena tournament '{id}'"))?;
                output::print(&tournament, json);
                Ok(())
            }
            ArenaTournamentsCommand::Update {
                id,
                name,
                clock_time,
                clock_increment,
                minutes,
                wait_minutes,
                start_date,
                variant,
                rated,
                position,
                berserkable,
                streakable,
                has_chat,
                description,
                password,
                conditions_min_rating,
                conditions_max_rating,
                conditions_nb_rated_game,
                conditions_allow_list,
                conditions_bots,
                conditions_account_age,
            } => {
                let form = update::UpdateArenaTournamentForm {
                    name,
                    clock_time,
                    clock_increment,
                    minutes,
                    wait_minutes,
                    start_date,
                    variant: variant.map(|v| v.into()),
                    rated,
                    position,
                    berserkable,
                    streakable,
                    has_chat,
                    description,
                    password,
                    conditions_min_rating,
                    conditions_max_rating,
                    conditions_nb_rated_game,
                    conditions_allow_list,
                    conditions_bots,
                    conditions_account_age,
                };
                let tournament = lichess
                    .update_arena_tournament(&id, form)
                    .await
                    .wrap_err_with(|| format!("failed to update arena tournament '{id}'"))?;
                output::print(&tournament, json);
                Ok(())
            }
            ArenaTournamentsCommand::ExportGames {
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
                    .export_arena_tournament_games(&id, query)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to export games for arena tournament '{id}'")
                    })?;
                while let Some(game) = stream.next().await {
                    match game {
                        Ok(game) => output::print(&game, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            ArenaTournamentsCommand::Join {
                id,
                password,
                team,
                pair_me_asap,
            } => {
                let form = join::JoinArenaTournamentForm {
                    password,
                    team,
                    pair_me_asap,
                };
                let result = lichess
                    .join_arena_tournament(&id, form)
                    .await
                    .wrap_err_with(|| format!("failed to join arena tournament '{id}'"))?;
                println!("Joined tournament: {}", result);
                Ok(())
            }
            ArenaTournamentsCommand::Results { id, nb, sheet } => {
                let query = results::GetQuery { nb, sheet };
                let mut stream = lichess
                    .get_arena_tournament_results(&id, query)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch results for arena tournament '{id}'")
                    })?;
                while let Some(result) = stream.next().await {
                    match result {
                        Ok(result) => output::print(&result, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            ArenaTournamentsCommand::Teams { id } => {
                let standing = lichess
                    .get_arena_tournament_team_standing(id.as_str())
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch team standing for arena tournament '{id}'")
                    })?;
                output::print(&standing, json);
                Ok(())
            }
            ArenaTournamentsCommand::Terminate { id } => {
                let result = lichess
                    .terminate_arena_tournament(id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to terminate arena tournament '{id}'"))?;
                println!("Tournament terminated: {}", result);
                Ok(())
            }
            ArenaTournamentsCommand::Withdraw { id } => {
                let result = lichess
                    .withdraw_from_arena_tournament(id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to withdraw from arena tournament '{id}'"))?;
                println!("Withdrawn from tournament: {}", result);
                Ok(())
            }
            ArenaTournamentsCommand::UpdateTeamBattle {
                id,
                teams,
                nb_leaders,
            } => {
                let form = team_battle::TeamBattleForm { teams, nb_leaders };
                let tournament = lichess
                    .update_arena_team_battle(&id, form)
                    .await
                    .wrap_err_with(|| format!("failed to update team battle '{id}'"))?;
                output::print(&tournament, json);
                Ok(())
            }
            ArenaTournamentsCommand::CreatedByUser {
                username,
                nb,
                status,
            } => {
                let query = created_by_user::GetQuery { nb, status };
                let mut stream = lichess
                    .get_arena_tournaments_created_by_user(&username, query)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch tournaments created by '{username}'")
                    })?;
                while let Some(tournament) = stream.next().await {
                    match tournament {
                        Ok(tournament) => output::print(&tournament, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            ArenaTournamentsCommand::PlayedByUser {
                username,
                nb,
                performance,
            } => {
                let query = played_by_user::GetQuery { nb, performance };
                let mut stream = lichess
                    .get_arena_tournaments_played_by_user(&username, query)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch tournaments played by '{username}'")
                    })?;
                while let Some(tournament) = stream.next().await {
                    match tournament {
                        Ok(tournament) => output::print(&tournament, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
        }
    }
}
