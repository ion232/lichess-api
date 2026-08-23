use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::tv::*;
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Clone, ValueEnum)]
pub enum Channel {
    Bot,
    Blitz,
    RacingKings,
    UltraBullet,
    Bullet,
    Classical,
    ThreeCheck,
    Antichess,
    Computer,
    Horde,
    Rapid,
    Atomic,
    Crazyhouse,
    Chess960,
    KingOfTheHill,
    Best,
}

impl From<Channel> for ChannelName {
    fn from(channel: Channel) -> Self {
        match channel {
            Channel::Bot => ChannelName::Bot,
            Channel::Blitz => ChannelName::Blitz,
            Channel::RacingKings => ChannelName::RacingKings,
            Channel::UltraBullet => ChannelName::UltraBullet,
            Channel::Bullet => ChannelName::Bullet,
            Channel::Classical => ChannelName::Classical,
            Channel::ThreeCheck => ChannelName::ThreeCheck,
            Channel::Antichess => ChannelName::Antichess,
            Channel::Computer => ChannelName::Computer,
            Channel::Horde => ChannelName::Horde,
            Channel::Rapid => ChannelName::Rapid,
            Channel::Atomic => ChannelName::Atomic,
            Channel::Crazyhouse => ChannelName::Crazyhouse,
            Channel::Chess960 => ChannelName::Chess960,
            Channel::KingOfTheHill => ChannelName::KingOfTheHill,
            Channel::Best => ChannelName::Best,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum TvCommand {
    /// Get current TV champions for every channel
    Channels,
    /// Stream the current featured game overall
    StreamCurrent,
    /// Stream the current featured game of a channel
    StreamChannel {
        /// TV channel
        #[arg(value_enum)]
        channel: Channel,
    },
    /// Get ongoing games of a TV channel
    ChannelGames {
        /// TV channel
        #[arg(value_enum)]
        channel: Channel,
        /// Max number of games to fetch
        #[arg(long)]
        number_of_games: Option<u8>,
        /// Include the PGN moves
        #[arg(long)]
        moves: Option<bool>,
        /// Include the PGN moves as a JSON array
        #[arg(long)]
        pgn_in_json: Option<bool>,
        /// Include the PGN tags
        #[arg(long)]
        tags: Option<bool>,
        /// Include clock comments
        #[arg(long)]
        clocks: Option<bool>,
        /// Include the opening name
        #[arg(long)]
        opening: Option<bool>,
    },
}

impl TvCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            TvCommand::Channels => {
                let channels = lichess
                    .tv_channels()
                    .await
                    .wrap_err("failed to fetch tv channels")?;
                output::print(&channels, json);
                Ok(())
            }
            TvCommand::StreamCurrent => {
                let mut stream = lichess
                    .tv_stream_current()
                    .await
                    .wrap_err("failed to stream current tv game")?;
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(event) => output::print(&event, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            TvCommand::StreamChannel { channel } => {
                let request = stream::channel::GetRequest::new(channel.into());
                let mut stream = lichess
                    .tv_stream_channel_current(request)
                    .await
                    .wrap_err("failed to stream tv channel")?;
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(event) => output::print(&event, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            TvCommand::ChannelGames {
                channel,
                number_of_games,
                moves,
                pgn_in_json,
                tags,
                clocks,
                opening,
            } => {
                let query = games::GetQuery {
                    number_of_games,
                    moves,
                    pgn_in_json,
                    tags,
                    clocks,
                    opening,
                };
                let request = games::GetRequest::new(channel.into(), Some(query));
                let mut stream = lichess
                    .tv_channel_games(request)
                    .await
                    .wrap_err("failed to fetch tv channel games")?;
                while let Some(game) = stream.next().await {
                    let game = game.wrap_err("failed to read tv channel game")?;
                    output::print(&game, json);
                }
                Ok(())
            }
        }
    }
}
