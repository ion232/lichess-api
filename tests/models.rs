use lichess_api::error::Result;
use lichess_api::models::puzzles::*;

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
fn daily_puzzle() {
    test_model::<DailyPuzzle>("puzzle");
}
