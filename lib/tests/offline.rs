use lichess_api::model::*;

use serde::de::DeserializeOwned;
use serde::Serialize;

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
    test_response_model::<board::stream::events::Event>("challenge");
    test_response_model::<board::stream::events::Event>("challenge_canceled");
    test_response_model::<board::stream::events::Event>("challenge_declined");
    test_response_model::<board::stream::events::Event>("game_start");
    test_response_model::<board::stream::events::Event>("game_finish");
}

#[test]
pub fn board_game_stream() {
    test_response_model::<board::stream::game::Event>("game_full_human");
    test_response_model::<board::stream::game::Event>("game_state");
    test_response_model::<board::stream::game::Event>("game_state_resign");
    test_response_model::<board::stream::game::Event>("chat_line");
    test_response_model::<board::stream::game::Event>("chat_line_spectator");
    test_response_model::<board::stream::game::Event>("opponent_gone_false");
    test_response_model::<board::stream::game::Event>("opponent_gone_true");
}

#[test]
pub fn challenge_ai() {
    test_response_model::<games::stream::moves::Move>("challenge_ai");
    test_response_model::<board::stream::game::Event>("game_full_ai");
}

#[test]
pub fn challenge_anonymous() {
    test_response_model::<board::stream::events::Event>("challenge_anonymous");
    test_response_model::<board::stream::game::Event>("game_full_anonymous");
}

#[test]
pub fn games_export() {
    test_response_model::<games::GameJson>("game_json");
}

#[test]
pub fn puzzle() {
    test_response_model::<puzzles::PuzzleAndGame>("puzzle");
}

#[test]
pub fn puzzle_round() {
    test_response_model::<puzzles::activity::PuzzleRoundJson>("puzzle_round");
}

#[test]
pub fn puzzle_race() {
    test_response_model::<puzzles::race::PuzzleRaceJson>("puzzle_race");
}

#[test]
pub fn puzzle_dashboard() {
    test_response_model::<puzzles::dashboard::PuzzleDashboardJson>("puzzle_dashboard");
}

#[test]
pub fn storm_dashboard() {
    test_response_model::<puzzles::storm_dashboard::StormDashboardJson>("storm_dashboard");
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
    test_response_model::<users::Leaderboards>("players");
    test_response_model::<Vec<users::rating_history::RatingEntry>>("rating-history");
    test_response_model::<users::rating_history::RatingHistory>("rating-history");
    test_response_model::<users::performance::Performance>("performance");
    test_response_model::<Vec<users::activity::Activity>>("activities");
    test_response_model::<Vec<users::StreamingUser>>("streamers");
    test_response_model::<Vec<users::Note>>("notes");
}

fn test_response_model<Model: Serialize + DeserializeOwned>(file_name: &str) {
    let path = format!("./tests/data/response/{}.json", file_name);
    test_model::<Model>(path);
}

fn test_model<Model: Serialize + DeserializeOwned>(path: String) {
    let model_string = fs::read_to_string(path).expect("Unable to read file.");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_string).expect("Unable to serialize model into json value.");
    let model: Model =
        serde_json::from_str(&model_string).expect("Unable to deserialize json string to model.");
    let reserialized_model_json: serde_json::Value =
        serde_json::to_value(&model).expect("Unable to serialize model to json value.");

    assert_eq!(model_json, reserialized_model_json);
}
