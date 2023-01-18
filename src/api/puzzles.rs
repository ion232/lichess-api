use bytes::Bytes;

use crate::client::LichessApi;
use crate::models::puzzles::{DailyPuzzleQuery, DailyPuzzle};
use crate::error::Result;

impl<'a, HttpClient> LichessApi<'a, HttpClient> {
    pub(crate) fn get_daily_puzzle_request(&self) -> Result<http::Request<Bytes>> {
        let path_segments = vec!["api", "puzzle", "daily"];
        let request = self
            .request_builder::<DailyPuzzleQuery>(path_segments, None)?
            .method(http::Method::GET)
            .body(bytes::Bytes::new())?;
        Ok(request)
    }
}

impl<'a> LichessApi<'a, reqwest::Client> {
    // pub async fn get_puzzle_activity(&self) -> Result<DailyPuzzle> {
    //     let url= format!("{}/api/puzzle/activity", LICHESS_BASE);
    //     let request = self.default_builder(&url).build()?;
    //     let mut stream = self.send::<DailyPuzzle>(request).await?;
    //     if let Some(result) = stream.next().await {
    //         result
    //     } else {
    //         Err(Error::Response("empty response stream".to_string()))
    //     }
    // }

    pub async fn get_daily_puzzle(&self) -> Result<DailyPuzzle> {
        let request = self.get_daily_puzzle_request()?;
        let mut stream = self.send(request).await?;
        self.expect_one_model(&mut stream).await
    }

    // pub async fn get_puzzle_dashboard(&self, days: u32) -> Result<DailyPuzzle> {
    //     let url= format!("{}/api/puzzle/dashboard/{}", LICHESS_BASE, days);

    // }

    // pub async fn get_puzzle_storm_dashboard(&self) -> Result<DailyPuzzle> {
    //     let url= format!("{}/api/storm/dashboard/{}", LICHESS_BASE, username);
    // }

    // pub async fn get_puzzle(&self) -> Result<DailyPuzzle> {
    //     let url= format!("{}/api/puzzle/{}", LICHESS_BASE, id);
    // }

    // pub async fn make_puzzle_race(&self) -> Result<DailyPuzzle> {
    //     let url= format!("{}/api/racer", LICHESS_BASE);
    // }
}
