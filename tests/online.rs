use futures::StreamExt;
use lichess_api::client::*;
use reqwest;
use tokio;

#[tokio::test(flavor = "current_thread")]
pub async fn daily_puzzle() {
    let api = make_api();
    let response = api.get_daily_puzzle().await.unwrap();
    println!("{:?}", response);
    assert!(!response.puzzle.id.is_empty());
}

#[tokio::test(flavor = "current_thread")]
pub async fn simuls() {
    let api = make_api();
    let response = api.get_current_simuls().await.unwrap();
    println!("{:?}", response);
    assert!(!response.started.first().unwrap().id.is_empty());
}

#[tokio::test(flavor = "current_thread")]
pub async fn fide_player() {
    let api = make_api();
    let response = api.get_fide_player(1503014).await.unwrap();
    println!("{:?}", response);
    assert!(response.id == 1503014);
}

#[tokio::test(flavor = "current_thread")]
pub async fn fide_search() {
    let api = make_api();
    let response = api.search_fide_player("Magnus Carlsen").await.unwrap();
    assert!(response.len() > 0);
    for player in response {
        println!("{:?}", player);
    }
}

fn make_api() -> LichessApi<reqwest::Client> {
    let http: reqwest::Client = reqwest::Client::new();
    LichessApi::new(http, None)
}
