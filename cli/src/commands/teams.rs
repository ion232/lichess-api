use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::teams::*;
use lichess_api::model::{ArenaStatusName, SwissStatus};
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Clone, ValueEnum)]
pub enum TeamArenaStatus {
    Created,
    Started,
    Finished,
}

impl From<TeamArenaStatus> for ArenaStatusName {
    fn from(status: TeamArenaStatus) -> Self {
        match status {
            TeamArenaStatus::Created => ArenaStatusName::Created,
            TeamArenaStatus::Started => ArenaStatusName::Started,
            TeamArenaStatus::Finished => ArenaStatusName::Finished,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TeamSwissStatus {
    Created,
    Started,
    Finished,
}

impl From<TeamSwissStatus> for SwissStatus {
    fn from(status: TeamSwissStatus) -> Self {
        match status {
            TeamSwissStatus::Created => SwissStatus::Created,
            TeamSwissStatus::Started => SwissStatus::Started,
            TeamSwissStatus::Finished => SwissStatus::Finished,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum TeamsCommand {
    /// Get popular teams
    Popular {
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },
    /// Search teams
    Search {
        /// Search text
        #[arg(long)]
        text: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },
    /// Get teams of a player
    Of {
        /// Username
        username: String,
    },
    /// Get a single team
    Get {
        /// Team ID
        team_id: String,
    },
    /// Get members of a team
    Members {
        /// Team ID
        team_id: String,
        /// Return the full list of members, not paginated
        #[arg(long)]
        full: bool,
    },
    /// Get team Arena tournaments
    Arena {
        /// Team ID
        team_id: String,
        /// Max number of tournaments to fetch
        #[arg(long)]
        max: Option<u32>,
        /// Filter by tournament status
        #[arg(long, value_enum)]
        status: Option<TeamArenaStatus>,
        /// Filter by tournament creator
        #[arg(long)]
        created_by: Option<String>,
        /// Filter by tournament name
        #[arg(long)]
        name: Option<String>,
    },
    /// Get team Swiss tournaments
    Swiss {
        /// Team ID
        team_id: String,
        /// Max number of tournaments to fetch
        #[arg(long)]
        max: Option<u32>,
        /// Filter by tournament status
        #[arg(long, value_enum)]
        status: Option<TeamSwissStatus>,
        /// Filter by tournament creator
        #[arg(long)]
        created_by: Option<String>,
        /// Filter by tournament name
        #[arg(long)]
        name: Option<String>,
    },
    /// Get join requests for a team you lead
    JoinRequests {
        /// Team ID
        team_id: String,
        /// Include declined requests
        #[arg(long)]
        declined: bool,
    },
    /// Accept a join request
    AcceptRequest {
        /// Team ID
        team_id: String,
        /// User ID
        user_id: String,
    },
    /// Decline a join request
    DeclineRequest {
        /// Team ID
        team_id: String,
        /// User ID
        user_id: String,
    },
    /// Kick a user from your team
    Kick {
        /// Team ID
        team_id: String,
        /// User ID
        user_id: String,
    },
    /// Join a team
    Join {
        /// Team ID
        team_id: String,
        /// Message to the team leader
        #[arg(long)]
        message: Option<String>,
        /// Password, if the team requires one
        #[arg(long)]
        password: Option<String>,
    },
    /// Leave a team
    Quit {
        /// Team ID
        team_id: String,
    },
    /// Send a message to all members of a team you lead
    SendUpdate {
        /// Team ID
        team_id: String,
        /// Message text
        message: String,
    },
    /// Get updates from your teams
    Updates {
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },
    /// Get updates from one of your teams
    UpdatesOfTeam {
        /// Team ID
        team_id: String,
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },
}

impl TeamsCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            TeamsCommand::Popular { page } => {
                let request = all::GetRequest::new(all::GetQuery { page });
                let teams = lichess
                    .get_popular_teams(request)
                    .await
                    .wrap_err("failed to fetch popular teams")?;
                output::print(&teams, json);
                Ok(())
            }
            TeamsCommand::Search { text, page } => {
                let request = search::GetRequest::new(search::GetQuery { text, page });
                let teams = lichess
                    .search_teams(request)
                    .await
                    .wrap_err("failed to search teams")?;
                output::print(&teams, json);
                Ok(())
            }
            TeamsCommand::Of { username } => {
                let request = of_username::GetRequest::new(&username);
                let teams = lichess
                    .get_teams_of_player(request)
                    .await
                    .wrap_err_with(|| format!("failed to fetch teams of player '{username}'"))?;
                output::print(&teams, json);
                Ok(())
            }
            TeamsCommand::Get { team_id } => {
                let request = show::GetRequest::new(&team_id);
                let team = lichess
                    .get_team(request)
                    .await
                    .wrap_err_with(|| format!("failed to fetch team '{team_id}'"))?;
                output::print(&team, json);
                Ok(())
            }
            TeamsCommand::Members { team_id, full } => {
                let request =
                    users::GetRequest::new(&team_id, users::GetQuery { full: Some(full) });
                let mut stream = lichess
                    .get_team_members(request)
                    .await
                    .wrap_err_with(|| format!("failed to fetch members of team '{team_id}'"))?;
                while let Some(member) = stream.next().await {
                    let member = member.wrap_err("failed to read team member")?;
                    output::print(&member, json);
                }
                Ok(())
            }
            TeamsCommand::Arena {
                team_id,
                max,
                status,
                created_by,
                name,
            } => {
                let request = arena::GetRequest::new(
                    &team_id,
                    arena::GetQuery {
                        max,
                        status: status.map(|s| s.into()),
                        created_by,
                        name,
                    },
                );
                let mut stream = lichess
                    .get_team_arena_tournaments(request)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch arena tournaments for team '{team_id}'")
                    })?;
                while let Some(tournament) = stream.next().await {
                    let tournament = tournament.wrap_err("failed to read arena tournament")?;
                    output::print(&tournament, json);
                }
                Ok(())
            }
            TeamsCommand::Swiss {
                team_id,
                max,
                status,
                created_by,
                name,
            } => {
                let request = swiss::GetRequest::new(
                    &team_id,
                    swiss::GetQuery {
                        max,
                        status: status.map(|s| s.into()),
                        created_by,
                        name,
                    },
                );
                let mut stream = lichess
                    .get_team_swiss_tournaments(request)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch swiss tournaments for team '{team_id}'")
                    })?;
                while let Some(tournament) = stream.next().await {
                    let tournament = tournament.wrap_err("failed to read swiss tournament")?;
                    output::print(&tournament, json);
                }
                Ok(())
            }
            TeamsCommand::JoinRequests { team_id, declined } => {
                let request = requests::GetRequest::new(
                    &team_id,
                    requests::GetQuery {
                        declined: Some(declined),
                    },
                );
                let requests = lichess
                    .get_team_join_requests(request)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch join requests for team '{team_id}'")
                    })?;
                output::print(&requests, json);
                Ok(())
            }
            TeamsCommand::AcceptRequest { team_id, user_id } => {
                let request = request_accept::PostRequest::new(&team_id, &user_id);
                let result = lichess
                    .accept_team_join_request(request)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to accept join request from '{user_id}' for team '{team_id}'"
                        )
                    })?;
                println!("Join request accepted: {}", result);
                Ok(())
            }
            TeamsCommand::DeclineRequest { team_id, user_id } => {
                let request = request_decline::PostRequest::new(&team_id, &user_id);
                let result = lichess
                    .decline_team_join_request(request)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to decline join request from '{user_id}' for team '{team_id}'"
                        )
                    })?;
                println!("Join request declined: {}", result);
                Ok(())
            }
            TeamsCommand::Kick { team_id, user_id } => {
                let request = kick::PostRequest::new(&team_id, &user_id);
                let result = lichess.kick_team_member(request).await.wrap_err_with(|| {
                    format!("failed to kick '{user_id}' from team '{team_id}'")
                })?;
                println!("Member kicked: {}", result);
                Ok(())
            }
            TeamsCommand::Join {
                team_id,
                message,
                password,
            } => {
                let request =
                    join::PostRequest::new(&team_id, join::JoinForm { message, password });
                let result = lichess
                    .join_team(request)
                    .await
                    .wrap_err_with(|| format!("failed to join team '{team_id}'"))?;
                println!("Joined team '{}': {}", team_id, result);
                Ok(())
            }
            TeamsCommand::Quit { team_id } => {
                let request = quit::PostRequest::new(&team_id);
                let result = lichess
                    .quit_team(request)
                    .await
                    .wrap_err_with(|| format!("failed to quit team '{team_id}'"))?;
                println!("Left team '{}': {}", team_id, result);
                Ok(())
            }
            TeamsCommand::SendUpdate { team_id, message } => {
                let request = pm_all::PostRequest::new(&team_id, message);
                let result = lichess
                    .send_team_update(request)
                    .await
                    .wrap_err_with(|| format!("failed to send update to team '{team_id}'"))?;
                println!("Update sent: {}", result);
                Ok(())
            }
            TeamsCommand::Updates { page } => {
                let request = updates::GetRequest::new(updates::GetQuery { page });
                let updates = lichess
                    .get_team_updates(request)
                    .await
                    .wrap_err("failed to fetch team updates")?;
                output::print(&updates, json);
                Ok(())
            }
            TeamsCommand::UpdatesOfTeam { team_id, page } => {
                let request =
                    updates_of_team::GetRequest::new(&team_id, updates_of_team::GetQuery { page });
                let updates = lichess
                    .get_team_updates_of_team(request)
                    .await
                    .wrap_err_with(|| format!("failed to fetch updates for team '{team_id}'"))?;
                output::print(&updates, json);
                Ok(())
            }
        }
    }
}
