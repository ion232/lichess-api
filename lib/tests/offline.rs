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
pub fn challenges() {
    test_response_model::<challenges::ChallengeJson>("challenge_json");
    test_response_model::<challenges::ChallengeOpenJson>("challenge_open_json");
    test_response_model::<challenges::ChallengeDeclinedJson>("challenge_declined_json");
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
