use lichess_api::client::LichessApi;
use lichess_api::error::Result;
use reqwest;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::ClientBuilder::new().build().unwrap();

    let api = LichessApi::new(&client, None);
    let daily_puzzle = api.get_daily_puzzle().await?;
    let pgn = daily_puzzle.game.pgn;

    println!("{}", pgn);

    Ok(())
}
