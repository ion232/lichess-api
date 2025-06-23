use clap::Subcommand;
use color_eyre::Result;
use lichess_api::client::LichessApi;
use lichess_api::model::{PerfType, users};
use reqwest;

type Lichess = LichessApi<reqwest::Client>;

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
        /// Performance type (e.g., bullet, blitz, rapid, classical, etc.)
        perf: String,
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
        term: String,
        /// Include friend names
        #[arg(long)]
        friend: bool,
    },
    /// Get all top 10 leaderboards
    Top10,
    /// Get one leaderboard
    Leaderboard {
        /// Number of users to fetch (1-200)
        #[arg(default_value = "10")]
        nb: u8,
        /// Variant (e.g., bullet, blitz, rapid, classical, etc.)
        perftype: String,
    },
}

impl UsersCommand {
    pub async fn run(self, lichess: Lichess) -> Result<()> {
        match self {
            UsersCommand::Get { username, trophies } => {
                let request = users::public::GetRequest::new(&username, trophies);
                let user = lichess.get_public_user_data(request).await?;
                println!("{:#?}", user);
                Ok(())
            }
            UsersCommand::Status {
                users,
                with_game_ids,
            } => {
                let user_ids: Vec<String> =
                    users.split(',').map(|s| s.trim().to_string()).collect();
                let request = users::status::GetRequest::new(user_ids, with_game_ids);
                let statuses = lichess.get_status_of_users(request).await?;
                for status in statuses {
                    println!("{:#?}", status);
                }
                Ok(())
            }
            UsersCommand::RatingHistory { username } => {
                let request = users::rating_history::GetRequest::new(&username);
                let history = lichess.get_rating_history(request).await?;
                println!("{:#?}", history);
                Ok(())
            }
            UsersCommand::Performance { username, perf } => {
                let perf_type = match perf.as_str() {
                    "ultraBullet" => PerfType::UltraBullet,
                    "bullet" => PerfType::Bullet,
                    "blitz" => PerfType::Blitz,
                    "rapid" => PerfType::Rapid,
                    "classical" => PerfType::Classical,
                    "chess960" => PerfType::Chess960,
                    "crazyhouse" => PerfType::Crazyhouse,
                    "antichess" => PerfType::Antichess,
                    "atomic" => PerfType::Atomic,
                    "horde" => PerfType::Horde,
                    "kingOfTheHill" => PerfType::KingOfTheHill,
                    "racingKings" => PerfType::RacingKings,
                    "threeCheck" => PerfType::ThreeCheck,
                    _ => {
                        println!("Invalid performance type: {}", perf);
                        return Ok(());
                    }
                };
                let request = users::performance::GetRequest::new(&username, perf_type);
                let performance = lichess.get_user_performance_statistics(request).await?;
                println!("{:#?}", performance);
                Ok(())
            }
            UsersCommand::ByIds { ids } => {
                let user_ids: Vec<String> = ids.split(',').map(|s| s.trim().to_string()).collect();
                let request = users::by_id::PostRequest::new(user_ids);
                let users = lichess.get_users_by_id(request).await?;
                for user in users {
                    println!("{:#?}", user);
                }
                Ok(())
            }
            UsersCommand::LiveStreamers => {
                let streamers = lichess.get_live_streamers().await?;
                for streamer in streamers {
                    println!("{:#?}", streamer);
                }
                Ok(())
            }
            UsersCommand::Crosstable {
                user1,
                user2,
                matchup,
            } => {
                let request = users::crosstable::GetRequest::new(&user1, &user2, Some(matchup));
                let crosstable = lichess.get_crosstable(request).await?;
                println!("{:#?}", crosstable);
                Ok(())
            }
            UsersCommand::Autocomplete { term, friend } => {
                if term.len() < 3 {
                    println!("Search term must be at least 3 characters");
                    return Ok(());
                }
                let request = users::autocomplete::GetRequest::new(&term, Some(friend));
                let suggestions = lichess.autocomplete_users(request).await?;
                for user in suggestions {
                    println!("{} ({})", user.name, user.id);
                }
                Ok(())
            }
            UsersCommand::Top10 => {
                let leaderboards = lichess.get_all_top_10().await?;
                println!("{:#?}", leaderboards);
                Ok(())
            }
            UsersCommand::Leaderboard { nb, perftype } => {
                let perf_type = match perftype.as_str() {
                    "ultraBullet" => PerfType::UltraBullet,
                    "bullet" => PerfType::Bullet,
                    "blitz" => PerfType::Blitz,
                    "rapid" => PerfType::Rapid,
                    "classical" => PerfType::Classical,
                    "chess960" => PerfType::Chess960,
                    "crazyhouse" => PerfType::Crazyhouse,
                    "antichess" => PerfType::Antichess,
                    "atomic" => PerfType::Atomic,
                    "horde" => PerfType::Horde,
                    "kingOfTheHill" => PerfType::KingOfTheHill,
                    "racingKings" => PerfType::RacingKings,
                    "threeCheck" => PerfType::ThreeCheck,
                    _ => {
                        println!("Invalid performance type: {}", perftype);
                        return Ok(());
                    }
                };
                let request = users::leaderboard::GetRequest::new(nb, perf_type);
                let leaderboard = lichess.get_one_leaderboard(request).await?;
                println!("{:#?}", leaderboard);
                Ok(())
            }
        }
    }
}
