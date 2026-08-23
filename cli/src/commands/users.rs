use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use lichess_api::client::LichessApi;
use lichess_api::model::{PerfType, users};
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Clone, ValueEnum)]
pub enum Performance {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Chess960,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    ThreeCheck,
}

impl From<Performance> for PerfType {
    fn from(perf: Performance) -> Self {
        match perf {
            Performance::UltraBullet => PerfType::UltraBullet,
            Performance::Bullet => PerfType::Bullet,
            Performance::Blitz => PerfType::Blitz,
            Performance::Rapid => PerfType::Rapid,
            Performance::Classical => PerfType::Classical,
            Performance::Chess960 => PerfType::Chess960,
            Performance::Crazyhouse => PerfType::Crazyhouse,
            Performance::Antichess => PerfType::Antichess,
            Performance::Atomic => PerfType::Atomic,
            Performance::Horde => PerfType::Horde,
            Performance::KingOfTheHill => PerfType::KingOfTheHill,
            Performance::RacingKings => PerfType::RacingKings,
            Performance::ThreeCheck => PerfType::ThreeCheck,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum UsersCommand {
    /// Get public data of a user
    Get {
        /// Username
        username: String,
        /// Include trophy information
        #[arg(long)]
        trophies: bool,
    },
    /// Get the online, playing and streaming statuses of several users
    Status {
        /// Comma-separated list of usernames (up to 100)
        users: String,
        /// Include current game IDs
        #[arg(long)]
        with_game_ids: bool,
    },
    /// Get rating history of a user
    RatingHistory {
        /// Username
        username: String,
    },
    /// Get performance statistics of a user
    Performance {
        /// Username
        username: String,
        /// Performance type
        #[arg(value_enum)]
        perf: Performance,
    },
    /// Get users by their IDs
    ByIds {
        /// Comma-separated list of user IDs
        ids: String,
    },
    /// Get current live streamers
    LiveStreamers,
    /// Get the crosstable of two users
    Crosstable {
        /// First username
        user1: String,
        /// Second username
        user2: String,
        /// Include match results
        #[arg(long)]
        matchup: bool,
    },
    /// Autocomplete usernames
    Autocomplete {
        /// Search term (at least 3 characters)
        #[arg(value_parser = parse_autocomplete_term)]
        term: String,
        /// Include friend names
        #[arg(long)]
        friend: bool,
    },
    /// Get all top 10 leaderboards
    Top10,
    /// Get one leaderboard
    Leaderboard {
        /// Performance type
        #[arg(value_enum)]
        perf: Performance,
        /// Number of users to fetch (1-200)
        #[arg(default_value = "10")]
        count: u8,
    },
    /// Get activity feed of a user
    Activity {
        /// Username
        username: String,
    },
}

fn parse_autocomplete_term(term: &str) -> std::result::Result<String, String> {
    if term.len() < 3 {
        Err("search term must be at least 3 characters".to_string())
    } else {
        Ok(term.to_string())
    }
}

impl UsersCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            UsersCommand::Get { username, trophies } => {
                let request = users::public::GetRequest::new(&username, trophies);
                let user = lichess
                    .get_public_user_data(request)
                    .await
                    .wrap_err_with(|| format!("failed to fetch public data for '{username}'"))?;
                output::print(&user, json);
                Ok(())
            }
            UsersCommand::Status {
                users,
                with_game_ids,
            } => {
                let user_ids: Vec<String> =
                    users.split(',').map(|s| s.trim().to_string()).collect();
                let request = users::status::GetRequest::new(user_ids, with_game_ids);
                let statuses = lichess
                    .get_status_of_users(request)
                    .await
                    .wrap_err("failed to fetch user statuses")?;
                output::print(&statuses, json);
                Ok(())
            }
            UsersCommand::RatingHistory { username } => {
                let request = users::rating_history::GetRequest::new(&username);
                let history = lichess
                    .get_rating_history(request)
                    .await
                    .wrap_err_with(|| format!("failed to fetch rating history for '{username}'"))?;
                output::print(&history, json);
                Ok(())
            }
            UsersCommand::Performance { username, perf } => {
                let request = users::performance::GetRequest::new(&username, perf.into());
                let perf_stat = lichess
                    .get_user_performance_statistics(request)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch performance statistics for '{username}'")
                    })?;
                output::print(&perf_stat, json);
                Ok(())
            }
            UsersCommand::ByIds { ids } => {
                let user_ids: Vec<String> = ids.split(',').map(|s| s.trim().to_string()).collect();
                let request = users::by_id::PostRequest::new(user_ids);
                let users = lichess
                    .get_users_by_id(request)
                    .await
                    .wrap_err("failed to fetch users by id")?;
                output::print(&users, json);
                Ok(())
            }
            UsersCommand::LiveStreamers => {
                let streamers = lichess
                    .get_live_streamers()
                    .await
                    .wrap_err("failed to fetch live streamers")?;
                output::print(&streamers, json);
                Ok(())
            }
            UsersCommand::Crosstable {
                user1,
                user2,
                matchup,
            } => {
                let request = users::crosstable::GetRequest::new(&user1, &user2, Some(matchup));
                let crosstable = lichess.get_crosstable(request).await.wrap_err_with(|| {
                    format!("failed to fetch crosstable for '{user1}' vs '{user2}'")
                })?;
                output::print(&crosstable, json);
                Ok(())
            }
            UsersCommand::Autocomplete { term, friend } => {
                let request = users::autocomplete::GetRequest::new(&term, Some(friend));
                let suggestions = lichess
                    .autocomplete_users(request)
                    .await
                    .wrap_err_with(|| format!("failed to autocomplete users for '{term}'"))?;
                output::print(&suggestions.result, json);
                Ok(())
            }
            UsersCommand::Top10 => {
                let leaderboards = lichess
                    .get_all_top_10()
                    .await
                    .wrap_err("failed to fetch top 10 leaderboards")?;
                output::print(&leaderboards, json);
                Ok(())
            }
            UsersCommand::Leaderboard { count, perf } => {
                let request = users::leaderboard::GetRequest::new(count, perf.into());
                let leaderboard = lichess
                    .get_one_leaderboard(request)
                    .await
                    .wrap_err("failed to fetch leaderboard")?;
                output::print(&leaderboard, json);
                Ok(())
            }
            UsersCommand::Activity { username } => {
                let request = users::activity::GetRequest::new(&username);
                let activities = lichess
                    .get_user_activity(request)
                    .await
                    .wrap_err_with(|| format!("failed to fetch activity for '{username}'"))?;
                output::print(&activities, json);
                Ok(())
            }
        }
    }
}
