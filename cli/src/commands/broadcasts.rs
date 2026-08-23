use clap::Subcommand;
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::broadcasts::*;
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Subcommand)]
pub enum BroadcastsCommand {
    /// Export all rounds of a broadcast tournament as PGN
    ExportPgn {
        /// Broadcast tournament ID
        broadcast_tournament_id: String,
        /// Include clock comments
        #[arg(long)]
        clocks: Option<bool>,
        /// Include move comments
        #[arg(long)]
        comments: Option<bool>,
    },
    /// Export one round as PGN
    ExportRoundPgn {
        /// Broadcast round ID
        broadcast_round_id: String,
        /// Include clock comments
        #[arg(long)]
        clocks: Option<bool>,
        /// Include move comments
        #[arg(long)]
        comments: Option<bool>,
    },
    /// Stream ongoing broadcast rounds of a group as PGN
    StreamGroupPgn {
        /// Broadcast group ID
        broadcast_group_id: String,
        /// Include clock comments
        #[arg(long)]
        clocks: Option<bool>,
        /// Include move comments
        #[arg(long)]
        comments: Option<bool>,
    },
    /// Stream an ongoing broadcast round as PGN
    StreamRoundPgn {
        /// Broadcast round ID
        broadcast_round_id: String,
        /// Include clock comments
        #[arg(long)]
        clocks: Option<bool>,
        /// Include move comments
        #[arg(long)]
        comments: Option<bool>,
    },
    /// Stream ongoing broadcast rounds of a tournament as PGN
    StreamTournamentPgn {
        /// Broadcast tournament ID
        broadcast_tour_id: String,
        /// Include clock comments
        #[arg(long)]
        clocks: Option<bool>,
        /// Include move comments
        #[arg(long)]
        comments: Option<bool>,
    },
    /// Get a broadcast round
    GetRound {
        /// Broadcast tournament slug
        broadcast_tournament_slug: String,
        /// Broadcast round slug
        broadcast_round_slug: String,
        /// Broadcast round ID
        broadcast_round_id: String,
    },
    /// Get your broadcast rounds
    MyRounds {
        /// Max number of rounds to fetch
        #[arg(long)]
        nb: Option<u32>,
    },
    /// Get broadcasts created by a user
    ByUser {
        /// Username
        username: String,
        /// Page number
        #[arg(long)]
        page: Option<u32>,
        /// Return HTML instead of markdown for the description
        #[arg(long)]
        html: Option<bool>,
    },
    /// Get official broadcasts
    Official {
        /// Max number of broadcasts to fetch
        #[arg(long)]
        nb: Option<u32>,
        /// Return HTML instead of markdown for the description
        #[arg(long)]
        html: Option<bool>,
        /// Only fetch broadcasts that are live
        #[arg(long)]
        live: Option<bool>,
    },
    /// Search broadcasts
    Search {
        /// Page number
        #[arg(long)]
        page: Option<u32>,
        /// Search query
        #[arg(long)]
        q: Option<String>,
    },
    /// Get paginated top broadcast previews
    Top {
        /// Page number
        #[arg(long)]
        page: Option<u32>,
        /// Return HTML instead of markdown for the description
        #[arg(long)]
        html: Option<bool>,
    },
    /// Get a broadcast tournament
    GetTournament {
        /// Broadcast tournament ID
        broadcast_tournament_id: String,
    },
    /// Create a broadcast tournament
    CreateTournament {
        /// Tournament name
        name: String,
        /// Format, e.g. "Swiss"
        #[arg(long)]
        info_format: Option<String>,
        /// Time control, e.g. "Classical"
        #[arg(long)]
        info_tc: Option<String>,
        /// FIDE time control category
        #[arg(long)]
        info_fide_tc: Option<String>,
        /// Time zone, e.g. "Europe/London"
        #[arg(long)]
        info_time_zone: Option<String>,
        /// Location
        #[arg(long)]
        info_location: Option<String>,
        /// Short list of notable players
        #[arg(long)]
        info_players: Option<String>,
        /// Official website URL
        #[arg(long)]
        info_website: Option<String>,
        /// Standings page URL
        #[arg(long)]
        info_standings: Option<String>,
        /// Regulations page URL
        #[arg(long)]
        info_regulations: Option<String>,
        /// Markdown description
        #[arg(long)]
        markdown: Option<String>,
        /// Show player scores
        #[arg(long)]
        show_scores: Option<bool>,
        /// Show player rating diffs
        #[arg(long)]
        show_rating_diffs: Option<bool>,
        /// Show a team table
        #[arg(long)]
        team_table: Option<bool>,
        /// Visibility: public, unlisted, or private
        #[arg(long)]
        visibility: Option<String>,
        /// Player names/ratings/titles, one per line
        #[arg(long)]
        players: Option<String>,
        /// Team names/tags, one per line
        #[arg(long)]
        teams: Option<String>,
        /// Tier, for official broadcasts
        #[arg(long)]
        tier: Option<i32>,
    },
    /// Update your broadcast tournament
    UpdateTournament {
        /// Broadcast tournament ID
        broadcast_tournament_id: String,
        /// Tournament name
        name: String,
        /// Format, e.g. "Swiss"
        #[arg(long)]
        info_format: Option<String>,
        /// Time control, e.g. "Classical"
        #[arg(long)]
        info_tc: Option<String>,
        /// FIDE time control category
        #[arg(long)]
        info_fide_tc: Option<String>,
        /// Time zone, e.g. "Europe/London"
        #[arg(long)]
        info_time_zone: Option<String>,
        /// Location
        #[arg(long)]
        info_location: Option<String>,
        /// Short list of notable players
        #[arg(long)]
        info_players: Option<String>,
        /// Official website URL
        #[arg(long)]
        info_website: Option<String>,
        /// Standings page URL
        #[arg(long)]
        info_standings: Option<String>,
        /// Regulations page URL
        #[arg(long)]
        info_regulations: Option<String>,
        /// Markdown description
        #[arg(long)]
        markdown: Option<String>,
        /// Show player scores
        #[arg(long)]
        show_scores: Option<bool>,
        /// Show player rating diffs
        #[arg(long)]
        show_rating_diffs: Option<bool>,
        /// Show a team table
        #[arg(long)]
        team_table: Option<bool>,
        /// Visibility: public, unlisted, or private
        #[arg(long)]
        visibility: Option<String>,
        /// Player names/ratings/titles, one per line
        #[arg(long)]
        players: Option<String>,
        /// Team names/tags, one per line
        #[arg(long)]
        teams: Option<String>,
        /// Tier, for official broadcasts
        #[arg(long)]
        tier: Option<i32>,
    },
    /// Create a broadcast round
    CreateRound {
        /// Broadcast tournament ID
        broadcast_tournament_id: String,
        /// Round name
        name: String,
        /// URL to sync the PGN from
        #[arg(long)]
        sync_url: Option<String>,
        /// Multiple sync URLs, one per line
        #[arg(long)]
        sync_urls: Option<String>,
        /// Sync from existing broadcast round IDs, one per line
        #[arg(long)]
        sync_ids: Option<String>,
        /// Sync from Lichess usernames currently playing, one per line
        #[arg(long)]
        sync_users: Option<String>,
        /// Only sync this board number
        #[arg(long)]
        only_round: Option<i32>,
        /// Slice the PGN source
        #[arg(long)]
        slices: Option<String>,
        /// Source type override for syncing
        #[arg(long)]
        sync_source: Option<String>,
        /// Start time as a unix timestamp in milliseconds
        #[arg(long)]
        starts_at: Option<i64>,
        /// Start automatically after the previous round completes
        #[arg(long)]
        starts_after_previous: Option<bool>,
        /// Delay in seconds before broadcasting moves
        #[arg(long)]
        delay: Option<i32>,
        /// Round status: new, started, or finished
        #[arg(long)]
        status: Option<String>,
        /// Whether the round is rated
        #[arg(long)]
        rated: Option<bool>,
        /// Time between synchronizations in seconds
        #[arg(long)]
        period: Option<i32>,
    },
    /// Get a player of a broadcast
    GetPlayer {
        /// Broadcast tournament ID
        broadcast_tournament_id: String,
        /// Player ID
        player_id: String,
    },
    /// Get players of a broadcast
    GetPlayers {
        /// Broadcast tournament ID
        broadcast_tournament_id: String,
    },
    /// Get the team leaderboard of a broadcast
    GetTeamStandings {
        /// Broadcast tournament ID
        broadcast_tournament_id: String,
    },
    /// Update a broadcast round
    UpdateRound {
        /// Broadcast round ID
        broadcast_round_id: String,
        /// Apply only the fields that were passed, instead of resetting the rest
        #[arg(long)]
        patch: Option<bool>,
        /// Round name
        name: String,
        /// URL to sync the PGN from
        #[arg(long)]
        sync_url: Option<String>,
        /// Multiple sync URLs, one per line
        #[arg(long)]
        sync_urls: Option<String>,
        /// Sync from existing broadcast round IDs, one per line
        #[arg(long)]
        sync_ids: Option<String>,
        /// Sync from Lichess usernames currently playing, one per line
        #[arg(long)]
        sync_users: Option<String>,
        /// Only sync this board number
        #[arg(long)]
        only_round: Option<i32>,
        /// Slice the PGN source
        #[arg(long)]
        slices: Option<String>,
        /// Source type override for syncing
        #[arg(long)]
        sync_source: Option<String>,
        /// Start time as a unix timestamp in milliseconds
        #[arg(long)]
        starts_at: Option<i64>,
        /// Start automatically after the previous round completes
        #[arg(long)]
        starts_after_previous: Option<bool>,
        /// Delay in seconds before broadcasting moves
        #[arg(long)]
        delay: Option<i32>,
        /// Round status: new, started, or finished
        #[arg(long)]
        status: Option<String>,
        /// Whether the round is rated
        #[arg(long)]
        rated: Option<bool>,
        /// Time between synchronizations in seconds
        #[arg(long)]
        period: Option<i32>,
    },
    /// Push PGN to a broadcast round
    PushPgn {
        /// Broadcast round ID
        broadcast_round_id: String,
        /// PGN text to push
        pgn: String,
    },
    /// Reset a broadcast round
    ResetRound {
        /// Broadcast round ID
        broadcast_round_id: String,
    },
}

impl BroadcastsCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            BroadcastsCommand::ExportPgn {
                broadcast_tournament_id,
                clocks,
                comments,
            } => {
                let query = export_pgn::GetQuery {
                    options: PgnStreamQuery { clocks, comments },
                };
                let mut stream = lichess
                    .export_broadcast_pgn(&broadcast_tournament_id, query)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to export pgn of broadcast tournament '{broadcast_tournament_id}'"
                        )
                    })?;
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.wrap_err("failed to read pgn stream")?;
                    println!("{chunk}");
                }
                Ok(())
            }
            BroadcastsCommand::ExportRoundPgn {
                broadcast_round_id,
                clocks,
                comments,
            } => {
                let query = export_round_pgn::GetQuery {
                    options: PgnStreamQuery { clocks, comments },
                };
                let mut stream = lichess
                    .export_broadcast_round_pgn(&broadcast_round_id, query)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to export pgn of broadcast round '{broadcast_round_id}'")
                    })?;
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.wrap_err("failed to read pgn stream")?;
                    println!("{chunk}");
                }
                Ok(())
            }
            BroadcastsCommand::StreamGroupPgn {
                broadcast_group_id,
                clocks,
                comments,
            } => {
                let query = stream_group_pgn::GetQuery {
                    options: PgnStreamQuery { clocks, comments },
                };
                let mut stream = lichess
                    .stream_broadcast_group_pgn(&broadcast_group_id, query)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to stream pgn of broadcast group '{broadcast_group_id}'")
                    })?;
                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(chunk) => println!("{chunk}"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BroadcastsCommand::StreamRoundPgn {
                broadcast_round_id,
                clocks,
                comments,
            } => {
                let query = stream_round_pgn::GetQuery {
                    options: PgnStreamQuery { clocks, comments },
                };
                let mut stream = lichess
                    .stream_broadcast_round_pgn(&broadcast_round_id, query)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to stream pgn of broadcast round '{broadcast_round_id}'")
                    })?;
                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(chunk) => println!("{chunk}"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BroadcastsCommand::StreamTournamentPgn {
                broadcast_tour_id,
                clocks,
                comments,
            } => {
                let query = stream_tournament_pgn::GetQuery {
                    options: PgnStreamQuery { clocks, comments },
                };
                let mut stream = lichess
                    .stream_broadcast_tournament_pgn(&broadcast_tour_id, query)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to stream pgn of broadcast tournament '{broadcast_tour_id}'"
                        )
                    })?;
                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(chunk) => println!("{chunk}"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BroadcastsCommand::GetRound {
                broadcast_tournament_slug,
                broadcast_round_slug,
                broadcast_round_id,
            } => {
                let round = lichess
                    .get_broadcast_round(
                        &broadcast_tournament_slug,
                        &broadcast_round_slug,
                        &broadcast_round_id,
                    )
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch broadcast round '{broadcast_round_id}'")
                    })?;
                output::print(&round, json);
                Ok(())
            }
            BroadcastsCommand::MyRounds { nb } => {
                let query = list_my_rounds::GetQuery { nb };
                let mut stream = lichess
                    .get_my_broadcast_rounds(query)
                    .await
                    .wrap_err("failed to fetch your broadcast rounds")?;
                while let Some(round) = stream.next().await {
                    match round {
                        Ok(round) => output::print(&round, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BroadcastsCommand::ByUser {
                username,
                page,
                html,
            } => {
                let query = list_by_user::GetQuery { page, html };
                let broadcasts = lichess
                    .get_broadcasts_by_user(&username, query)
                    .await
                    .wrap_err_with(|| format!("failed to fetch broadcasts by '{username}'"))?;
                output::print(&broadcasts, json);
                Ok(())
            }
            BroadcastsCommand::Official { nb, html, live } => {
                let query = list_official::GetQuery { nb, html, live };
                let mut stream = lichess
                    .get_official_broadcasts(query)
                    .await
                    .wrap_err("failed to fetch official broadcasts")?;
                while let Some(broadcast) = stream.next().await {
                    match broadcast {
                        Ok(broadcast) => output::print(&broadcast, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            BroadcastsCommand::Search { page, q } => {
                let query = search::GetQuery { page, q };
                let results = lichess
                    .search_broadcasts(query)
                    .await
                    .wrap_err("failed to search broadcasts")?;
                output::print(&results, json);
                Ok(())
            }
            BroadcastsCommand::Top { page, html } => {
                let query = top::GetQuery { page, html };
                let top = lichess
                    .get_top_broadcasts(query)
                    .await
                    .wrap_err("failed to fetch top broadcasts")?;
                output::print(&top, json);
                Ok(())
            }
            BroadcastsCommand::GetTournament {
                broadcast_tournament_id,
            } => {
                let tournament = lichess
                    .get_broadcast_tournament(broadcast_tournament_id.as_str())
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch broadcast tournament '{broadcast_tournament_id}'")
                    })?;
                output::print(&tournament, json);
                Ok(())
            }
            BroadcastsCommand::CreateTournament {
                name,
                info_format,
                info_tc,
                info_fide_tc,
                info_time_zone,
                info_location,
                info_players,
                info_website,
                info_standings,
                info_regulations,
                markdown,
                show_scores,
                show_rating_diffs,
                team_table,
                visibility,
                players,
                teams,
                tier,
            } => {
                let form = create_tournament::CreateBroadcastTournamentForm {
                    name,
                    info_format,
                    info_tc,
                    info_fide_tc,
                    info_time_zone,
                    info_location,
                    info_players,
                    info_website,
                    info_standings,
                    info_regulations,
                    markdown,
                    show_scores,
                    show_rating_diffs,
                    team_table,
                    visibility,
                    players,
                    teams,
                    tier,
                    tiebreaks: None,
                    grouping: None,
                };
                let tournament = lichess
                    .create_broadcast_tournament(form)
                    .await
                    .wrap_err("failed to create broadcast tournament")?;
                output::print(&tournament, json);
                Ok(())
            }
            BroadcastsCommand::UpdateTournament {
                broadcast_tournament_id,
                name,
                info_format,
                info_tc,
                info_fide_tc,
                info_time_zone,
                info_location,
                info_players,
                info_website,
                info_standings,
                info_regulations,
                markdown,
                show_scores,
                show_rating_diffs,
                team_table,
                visibility,
                players,
                teams,
                tier,
            } => {
                let form = create_tournament::CreateBroadcastTournamentForm {
                    name,
                    info_format,
                    info_tc,
                    info_fide_tc,
                    info_time_zone,
                    info_location,
                    info_players,
                    info_website,
                    info_standings,
                    info_regulations,
                    markdown,
                    show_scores,
                    show_rating_diffs,
                    team_table,
                    visibility,
                    players,
                    teams,
                    tier,
                    tiebreaks: None,
                    grouping: None,
                };
                let result = lichess
                    .update_broadcast_tournament(&broadcast_tournament_id, form)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to update broadcast tournament '{broadcast_tournament_id}'")
                    })?;
                println!("Updated tournament '{broadcast_tournament_id}': {}", result);
                Ok(())
            }
            BroadcastsCommand::CreateRound {
                broadcast_tournament_id,
                name,
                sync_url,
                sync_urls,
                sync_ids,
                sync_users,
                only_round,
                slices,
                sync_source,
                starts_at,
                starts_after_previous,
                delay,
                status,
                rated,
                period,
            } => {
                let form = create_round::BroadcastRoundForm {
                    name,
                    sync_url,
                    sync_urls,
                    sync_ids,
                    sync_users,
                    only_round,
                    slices,
                    sync_source,
                    starts_at,
                    starts_after_previous,
                    delay,
                    status,
                    rated,
                    custom_scoring: None,
                    team_custom_scoring: None,
                    period,
                };
                let round = lichess
                    .create_broadcast_round(&broadcast_tournament_id, form)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to create round for broadcast tournament '{broadcast_tournament_id}'"
                        )
                    })?;
                output::print(&round, json);
                Ok(())
            }
            BroadcastsCommand::GetPlayer {
                broadcast_tournament_id,
                player_id,
            } => {
                let player = lichess
                    .get_broadcast_player(&broadcast_tournament_id, &player_id)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to fetch player '{player_id}' of broadcast '{broadcast_tournament_id}'"
                        )
                    })?;
                output::print(&player, json);
                Ok(())
            }
            BroadcastsCommand::GetPlayers {
                broadcast_tournament_id,
            } => {
                let players = lichess
                    .get_broadcast_players(broadcast_tournament_id.as_str())
                    .await
                    .wrap_err_with(|| {
                        format!("failed to fetch players of broadcast '{broadcast_tournament_id}'")
                    })?;
                output::print(&players, json);
                Ok(())
            }
            BroadcastsCommand::GetTeamStandings {
                broadcast_tournament_id,
            } => {
                let standings = lichess
                    .get_broadcast_team_standings(broadcast_tournament_id.as_str())
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to fetch team standings of broadcast '{broadcast_tournament_id}'"
                        )
                    })?;
                output::print(&standings, json);
                Ok(())
            }
            BroadcastsCommand::UpdateRound {
                broadcast_round_id,
                patch,
                name,
                sync_url,
                sync_urls,
                sync_ids,
                sync_users,
                only_round,
                slices,
                sync_source,
                starts_at,
                starts_after_previous,
                delay,
                status,
                rated,
                period,
            } => {
                let query = update_round::PostQuery { patch };
                let form = create_round::BroadcastRoundForm {
                    name,
                    sync_url,
                    sync_urls,
                    sync_ids,
                    sync_users,
                    only_round,
                    slices,
                    sync_source,
                    starts_at,
                    starts_after_previous,
                    delay,
                    status,
                    rated,
                    custom_scoring: None,
                    team_custom_scoring: None,
                    period,
                };
                let round = lichess
                    .update_broadcast_round(&broadcast_round_id, query, form)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to update broadcast round '{broadcast_round_id}'")
                    })?;
                output::print(&round, json);
                Ok(())
            }
            BroadcastsCommand::PushPgn {
                broadcast_round_id,
                pgn,
            } => {
                let result = lichess
                    .push_broadcast_round_pgn(&broadcast_round_id, pgn)
                    .await
                    .wrap_err_with(|| {
                        format!("failed to push pgn to broadcast round '{broadcast_round_id}'")
                    })?;
                output::print(&result, json);
                Ok(())
            }
            BroadcastsCommand::ResetRound { broadcast_round_id } => {
                let result = lichess
                    .reset_broadcast_round(broadcast_round_id.as_str())
                    .await
                    .wrap_err_with(|| {
                        format!("failed to reset broadcast round '{broadcast_round_id}'")
                    })?;
                println!("Reset round '{broadcast_round_id}': {}", result);
                Ok(())
            }
        }
    }
}
