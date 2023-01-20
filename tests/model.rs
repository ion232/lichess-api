use lichess_api::model::*;

use serde::Serialize;
use serde::de::DeserializeOwned;

use std::fs;

fn test_model<Model: Serialize + DeserializeOwned>(file_name: &str) {
    let path = format!("./tests/data/{}.json", file_name);

    let file_model_string = fs::read_to_string(path)
        .expect("Unable to read file.");

    let file_model_value: serde_json::Value = serde_json::from_str(&file_model_string)
        .expect("Unable to serialize model into json value.");

    let model: Model = serde_json::from_str(&file_model_string)
        .expect("Unable to deserialize json string to model.");

    let model_value = serde_json::to_value(&model)
        .expect("Unable to serialize model to json value.");
    
    assert_eq!(model_value, file_model_value);
}

#[test]
pub fn puzzle() {
    test_model::<puzzles::PuzzleJson>("puzzle");
}

#[test]
pub fn puzzle_round() {
    test_model::<puzzles::PuzzleRoundJson>("puzzle_round");
}

#[test]
pub fn puzzle_race() {
    test_model::<puzzles::PuzzleRaceJson>("puzzle_race");
}

#[test]
pub fn puzzle_dashboard() {
    test_model::<puzzles::PuzzleDashboardJson>("puzzle_dashboard");
}

#[test]
pub fn storm_dashboard() {
    test_model::<puzzles::StormDashboardJson>("storm_dashboard");
}
