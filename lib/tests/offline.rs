use lichess_api::model::*;

use serde::Serialize;
use serde::de::DeserializeOwned;

use std::fs;

#[test]
pub fn ok() {
    test_response_model::<Ok>("ok");
}

#[test]
pub fn response() {
    test_response_model::<Response<()>>("not_found");
    test_response_model::<Response<()>>("error");
}

#[test]
pub fn board_event_stream() {
    test_response_model::<board::stream::events::Event>("game_start_event");
    test_response_model::<board::stream::events::Event>("game_start_ai_event");
    test_response_model::<board::stream::events::Event>("game_finish_event");
    test_response_model::<board::stream::events::Event>("game_finish_ai_event");
    test_response_model::<board::stream::events::Event>("challenge_event1");
    test_response_model::<board::stream::events::Event>("challenge_event2");
    test_response_model::<board::stream::events::Event>("challenge_declined_event1");
    test_response_model::<board::stream::events::Event>("challenge_declined_event2");
    test_response_model::<board::stream::events::Event>("challenge_canceled_event1");
    test_response_model::<board::stream::events::Event>("challenge_canceled_event2");
}

#[test]
pub fn board_game_stream() {
    test_response_model::<board::stream::game::Event>("game_full_event");
    test_response_model::<board::stream::game::Event>("game_state_event");
    test_response_model::<board::stream::game::Event>("game_state_resign");
    test_response_model::<board::stream::game::Event>("chat_line_event");
    test_response_model::<board::stream::game::Event>("opponent_gone_event");
}

#[test]
pub fn board_chat() {
    test_response_model::<Vec<board::chat::ChatLine>>("game_chat");
}

#[test]
pub fn puzzle_and_game() {
    test_response_model::<puzzles::PuzzleAndGame>("puzzle_and_game");
}

#[test]
pub fn puzzle_activity() {
    test_response_model::<puzzles::activity::PuzzleActivity>("puzzle_activity");
}

#[test]
pub fn puzzle_race() {
    test_response_model::<puzzles::race::PuzzleRacer>("puzzle_racer");
}

#[test]
pub fn puzzle_dashboard() {
    test_response_model::<puzzles::dashboard::PuzzleDashboard>("puzzle_dashboard");
}

#[test]
pub fn puzzle_replay() {
    test_response_model::<puzzles::replay::Replay>("puzzle_replay");
}

#[test]
pub fn storm_dashboard() {
    test_response_model::<puzzles::storm_dashboard::StormDashboard>("storm_dashboard");
}

#[test]
pub fn simuls() {
    test_response_model::<simuls::current::Simuls>("current_simuls");
}

#[test]
pub fn tv() {
    test_response_model::<tv::Channels>("tv_channels");
    test_response_model::<tv::stream::Event>("tv_stream_featured");
    test_response_model::<tv::stream::Event>("tv_stream_featured_untitled");
    test_response_model::<tv::stream::Event>("tv_stream_fen");
}

#[test]
pub fn users() {
    test_response_model::<users::Top10s>("players");
    test_response_model::<Vec<users::rating_history::RatingEntry>>("rating_history");
    test_response_model::<users::rating_history::RatingHistory>("rating_history");
    test_response_model::<users::performance::PerfStat>("user_performance");
    test_response_model::<Vec<users::StreamingUser>>("streamers");
    test_response_model::<Vec<users::UserNote>>("notes");
    test_response_model::<Vec<users::activity::UserActivity>>("user_activity");
    test_response_model::<Vec<users::activity::UserActivity>>("user_activities");
    test_response_model::<users::UserExtended>("user_extended");
    test_response_model::<Vec<users::status::User>>("user_statuses");
    test_response_model::<Vec<users::StreamingUser>>("live_streamers");
    test_response_model::<users::autocomplete::Autocompletions>("user_autocompletions");
}

#[test]
pub fn timeline() {
    test_response_model::<account::timeline::Timeline>("timeline");
}

#[test]
pub fn challenges() {
    test_response_model::<challenges::ChallengeJson>("challenge_json");
    test_response_model::<challenges::ChallengeOpenJson>("challenge_open_json");
    test_response_model::<challenges::ChallengeDeclinedJson>("challenge_declined_json");
}

#[test]
pub fn teams() {
    test_response_model::<teams::Team>("team");
    test_response_model::<teams::TeamPaginatorJson>("team_paginator");
    test_response_model::<teams::TeamRequestWithUser>("team_request_with_user");
    test_response_model::<teams::TeamUpdates>("team_updates");
    test_response_model::<teams::TeamUpdatesOfTeam>("team_updates_of_team");
    test_response_model::<teams::users::TeamMember>("team_member");
    test_response_model::<ArenaTournament>("arena_tournament");
    test_response_model::<SwissTournament>("swiss_tournament");
}

#[test]
pub fn arena_tournaments() {
    test_response_model::<arena_tournaments::ArenaTournaments>("arena_tournaments");
    test_response_model::<arena_tournaments::ArenaTournamentFull>("arena_tournament_full");
    test_response_model::<arena_tournaments::ArenaResult>("arena_result");
    test_response_model::<arena_tournaments::ArenaTeamStanding>("arena_team_standing");
    test_response_model::<arena_tournaments::ArenaTournamentPlayed>("arena_tournament_played");
}

#[test]
pub fn swiss_tournaments() {
    test_response_model::<swiss_tournaments::SwissResult>("swiss_result");
}

#[test]
pub fn bulk_pairings() {
    test_response_model::<bulk_pairings::BulkPairing>("bulk_pairing");
}

#[test]
pub fn studies() {
    test_response_model::<studies::create::CreateStudyResponse>("study_create");
    test_response_model::<studies::StudyMetadata>("study_metadata");
}

#[test]
pub fn broadcasts() {
    test_response_model::<broadcasts::BroadcastRound>("broadcast_round");
    test_response_model::<broadcasts::BroadcastWithRoundsAndFullGroup>("broadcast_tournament");
    test_response_model::<broadcasts::BroadcastMyRound>("broadcast_my_round");
    test_response_model::<broadcasts::BroadcastRoundNew>("broadcast_round_new");
    test_response_model::<broadcasts::BroadcastWithRounds>("broadcast_with_rounds");
    test_response_model::<broadcasts::BroadcastWithRounds>("broadcast_official");
    test_response_model::<broadcasts::BroadcastByUserPaginator>("broadcast_by_user_paginator");
    test_response_model::<broadcasts::BroadcastTop>("broadcast_top");
    test_response_model::<broadcasts::BroadcastPlayerEntryWithFideAndGames>("broadcast_player");
    test_response_model::<Vec<broadcasts::BroadcastPlayerEntry>>("broadcast_players");
    test_response_model::<Vec<broadcasts::BroadcastTeamLeaderboardEntry>>(
        "broadcast_team_leaderboard",
    );
    test_response_model::<broadcasts::BroadcastSearchPaginator>("broadcast_search_paginator");
    test_response_model::<broadcasts::BroadcastPgnPush>("broadcast_pgn_push");
}

fn test_response_model<Model: Serialize + DeserializeOwned>(file_name: &str) {
    let path = format!("./tests/data/response/{}.json", file_name);
    test_model::<Model>(path);
}

fn test_model<Model: Serialize + DeserializeOwned>(path: String) {
    let model_string = fs::read_to_string(&path).expect("Unable to read file.");
    let model_json: serde_json::Value = serde_json::from_str(&model_string)
        .expect("Unable to deserialize model string into json value.");
    let model: Model = serde_json::from_str(&model_string)
        .expect("Unable to deserialize model string into model.");
    let reserialized_model_json: serde_json::Value =
        serde_json::to_value(&model).expect("Unable to serialize model into json value.");

    assert_eq!(model_json, reserialized_model_json);
}
