use lichess_api::client::*;
use lichess_api::model::puzzles::daily;
use reqwest;
use tokio;

#[tokio::test(flavor = "current_thread")]
pub async fn daily_puzzle() {
    let api = make_api(None);
    let request = daily::GetRequest::new();
    let response = api.get_daily_puzzle(request).await.unwrap();
    println!("{:?}", response);
    assert!(!response.puzzle.id.is_empty());
}

fn make_api(auth_token: Option<String>) -> LichessApi<reqwest::Client> {
    let http: reqwest::Client = reqwest::Client::new();
    LichessApi::new(http, auth_token)
}
