use clap::{Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use futures::StreamExt;
use lichess_api::client::LichessApi;
use lichess_api::model::games::*;
use lichess_api::model::{Color, PerfType};
use reqwest;

use crate::output;

type Lichess = LichessApi<reqwest::Client>;

#[derive(Debug, Clone, ValueEnum)]
pub enum Sort {
    DateAsc,
    DateDesc,
}

impl From<Sort> for export::by_user::Sort {
    fn from(sort: Sort) -> Self {
        match sort {
            Sort::DateAsc => export::by_user::Sort::DateAsc,
            Sort::DateDesc => export::by_user::Sort::DateDesc,
        }
    }
}

impl From<Sort> for export::bookmarks::Sort {
    fn from(sort: Sort) -> Self {
        match sort {
            Sort::DateAsc => export::bookmarks::Sort::DateAsc,
            Sort::DateDesc => export::bookmarks::Sort::DateDesc,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum GameColor {
    White,
    Black,
    Random,
}

impl From<GameColor> for Color {
    fn from(color: GameColor) -> Self {
        match color {
            GameColor::White => Color::White,
            GameColor::Black => Color::Black,
            GameColor::Random => Color::Random,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Performance {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Correspondence,
    Chess960,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    ThreeCheck,
}

impl From<Performance> for PerfType {
    fn from(perf: Performance) -> Self {
        match perf {
            Performance::UltraBullet => PerfType::UltraBullet,
            Performance::Bullet => PerfType::Bullet,
            Performance::Blitz => PerfType::Blitz,
            Performance::Rapid => PerfType::Rapid,
            Performance::Classical => PerfType::Classical,
            Performance::Correspondence => PerfType::Correspondence,
            Performance::Chess960 => PerfType::Chess960,
            Performance::Crazyhouse => PerfType::Crazyhouse,
            Performance::Antichess => PerfType::Antichess,
            Performance::Atomic => PerfType::Atomic,
            Performance::Horde => PerfType::Horde,
            Performance::KingOfTheHill => PerfType::KingOfTheHill,
            Performance::RacingKings => PerfType::RacingKings,
            Performance::ThreeCheck => PerfType::ThreeCheck,
        }
    }
}

struct BaseArgs {
    moves: Option<bool>,
    pgn_in_json: Option<bool>,
    tags: Option<bool>,
    clocks: Option<bool>,
    evals: Option<bool>,
    accuracy: Option<bool>,
    opening: Option<bool>,
    literate: Option<bool>,
    players: Option<String>,
}

impl From<BaseArgs> for export::Base {
    fn from(args: BaseArgs) -> Self {
        let default = export::Base::default();
        export::Base {
            moves: args.moves.unwrap_or(default.moves),
            pgn_in_json: args.pgn_in_json.unwrap_or(default.pgn_in_json),
            tags: args.tags.unwrap_or(default.tags),
            clocks: args.clocks.unwrap_or(default.clocks),
            evals: args.evals.unwrap_or(default.evals),
            accuracy: args.accuracy.unwrap_or(default.accuracy),
            opening: args.opening.unwrap_or(default.opening),
            literate: args.literate.unwrap_or(default.literate),
            players: args.players,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum GamesCommand {
    /// Export one game as JSON
    ExportOne {
        /// Game ID
        game_id: String,
        /// Include the PGN moves
        #[arg(long)]
        moves: Option<bool>,
        /// Include the PGN moves as a JSON array
        #[arg(long)]
        pgn_in_json: Option<bool>,
        /// Include the PGN tags
        #[arg(long)]
        tags: Option<bool>,
        /// Include clock comments in the PGN moves
        #[arg(long)]
        clocks: Option<bool>,
        /// Include analysis evaluation comments in the PGN moves
        #[arg(long)]
        evals: Option<bool>,
        /// Include the accuracy percent of each player, when available
        #[arg(long)]
        accuracy: Option<bool>,
        /// Include the opening name
        #[arg(long)]
        opening: Option<bool>,
        /// Include a textual description of the game
        #[arg(long)]
        literate: Option<bool>,
        /// URL of a text file containing real names and ratings for each player
        #[arg(long)]
        players: Option<String>,
    },
    /// Export the ongoing game of a user, if any
    ExportOngoing {
        /// Username
        username: String,
        /// Include the PGN moves
        #[arg(long)]
        moves: Option<bool>,
        /// Include the PGN moves as a JSON array
        #[arg(long)]
        pgn_in_json: Option<bool>,
        /// Include the PGN tags
        #[arg(long)]
        tags: Option<bool>,
        /// Include clock comments in the PGN moves
        #[arg(long)]
        clocks: Option<bool>,
        /// Include analysis evaluation comments in the PGN moves
        #[arg(long)]
        evals: Option<bool>,
        /// Include the accuracy percent of each player, when available
        #[arg(long)]
        accuracy: Option<bool>,
        /// Include the opening name
        #[arg(long)]
        opening: Option<bool>,
        /// Include a textual description of the game
        #[arg(long)]
        literate: Option<bool>,
        /// URL of a text file containing real names and ratings for each player
        #[arg(long)]
        players: Option<String>,
    },
    /// Export all games of a user
    ExportByUser {
        /// Username
        username: String,
        /// Only export games since this timestamp (ms)
        #[arg(long)]
        since: Option<u64>,
        /// Only export games until this timestamp (ms)
        #[arg(long)]
        until: Option<u64>,
        /// Max number of games to export
        #[arg(long, default_value_t = 0)]
        max: u64,
        /// Only export games played against this opponent
        #[arg(long)]
        vs: Option<String>,
        /// Only export rated (true) or casual (false) games
        #[arg(long)]
        rated: Option<bool>,
        /// Only export games of this performance type
        #[arg(long, value_enum)]
        perf_type: Option<Performance>,
        /// Only export games where the user played this color
        #[arg(long, value_enum)]
        color: Option<GameColor>,
        /// Only export analysed games
        #[arg(long)]
        analysed: Option<bool>,
        /// Only export ongoing games
        #[arg(long)]
        ongoing: Option<bool>,
        /// Only export finished games
        #[arg(long)]
        finished: Option<bool>,
        /// Include the last position's FEN
        #[arg(long)]
        last_fen: Option<bool>,
        /// Sort order
        #[arg(long, value_enum)]
        sort: Option<Sort>,
        /// Include the PGN moves
        #[arg(long)]
        moves: Option<bool>,
        /// Include the PGN moves as a JSON array
        #[arg(long)]
        pgn_in_json: Option<bool>,
        /// Include the PGN tags
        #[arg(long)]
        tags: Option<bool>,
        /// Include clock comments in the PGN moves
        #[arg(long)]
        clocks: Option<bool>,
        /// Include analysis evaluation comments in the PGN moves
        #[arg(long)]
        evals: Option<bool>,
        /// Include the accuracy percent of each player, when available
        #[arg(long)]
        accuracy: Option<bool>,
        /// Include the opening name
        #[arg(long)]
        opening: Option<bool>,
        /// Include a textual description of the game
        #[arg(long)]
        literate: Option<bool>,
        /// URL of a text file containing real names and ratings for each player
        #[arg(long)]
        players: Option<String>,
    },
    /// Export games by their IDs
    ExportByIds {
        /// Comma-separated list of game IDs (up to 300)
        ids: String,
        /// Include the PGN moves
        #[arg(long)]
        moves: Option<bool>,
        /// Include the PGN moves as a JSON array
        #[arg(long)]
        pgn_in_json: Option<bool>,
        /// Include the PGN tags
        #[arg(long)]
        tags: Option<bool>,
        /// Include clock comments in the PGN moves
        #[arg(long)]
        clocks: Option<bool>,
        /// Include analysis evaluation comments in the PGN moves
        #[arg(long)]
        evals: Option<bool>,
        /// Include the accuracy percent of each player, when available
        #[arg(long)]
        accuracy: Option<bool>,
        /// Include the opening name
        #[arg(long)]
        opening: Option<bool>,
        /// Include a textual description of the game
        #[arg(long)]
        literate: Option<bool>,
        /// URL of a text file containing real names and ratings for each player
        #[arg(long)]
        players: Option<String>,
    },
    /// Stream games as they start and finish for a list of users
    StreamByUsers {
        /// Comma-separated list of usernames
        usernames: String,
        /// Also stream the current games of these users, if any
        #[arg(long)]
        with_current_games: bool,
    },
    /// Stream games as they start and finish for a list of game IDs
    StreamByIds {
        /// Stream ID, chosen by you
        stream_id: String,
        /// Comma-separated list of game IDs (up to 500)
        ids: String,
    },
    /// Add game IDs to an existing stream
    AddIds {
        /// Stream ID, as passed to `stream-by-ids`
        stream_id: String,
        /// Comma-separated list of game IDs to add
        ids: String,
    },
    /// Get your ongoing games
    Ongoing {
        /// Max number of games to fetch
        #[arg(default_value = "9")]
        max_games: u8,
    },
    /// Stream the moves of a game
    StreamMoves {
        /// Game ID
        game_id: String,
    },
    /// Import a game from PGN
    Import {
        /// PGN text of the game
        pgn: String,
    },
    /// Export your bookmarked games
    ExportBookmarks {
        /// Only export games since this timestamp (ms)
        #[arg(long)]
        since: Option<u64>,
        /// Only export games until this timestamp (ms)
        #[arg(long)]
        until: Option<u64>,
        /// Max number of games to export
        #[arg(long)]
        max: Option<u64>,
        /// Include the last position's FEN
        #[arg(long)]
        last_fen: Option<bool>,
        /// Sort order
        #[arg(long, value_enum)]
        sort: Option<Sort>,
        /// Include the PGN moves
        #[arg(long)]
        moves: Option<bool>,
        /// Include the PGN moves as a JSON array
        #[arg(long)]
        pgn_in_json: Option<bool>,
        /// Include the PGN tags
        #[arg(long)]
        tags: Option<bool>,
        /// Include clock comments in the PGN moves
        #[arg(long)]
        clocks: Option<bool>,
        /// Include analysis evaluation comments in the PGN moves
        #[arg(long)]
        evals: Option<bool>,
        /// Include the accuracy percent of each player, when available
        #[arg(long)]
        accuracy: Option<bool>,
        /// Include the opening name
        #[arg(long)]
        opening: Option<bool>,
        /// Include a textual description of the game
        #[arg(long)]
        literate: Option<bool>,
        /// URL of a text file containing real names and ratings for each player
        #[arg(long)]
        players: Option<String>,
    },
    /// Export your imported games
    ExportImports,
    /// Get the spectator chat of a game
    Chat {
        /// Game ID
        game_id: String,
    },
    /// Bookmark a game
    Bookmark {
        /// Game ID
        game_id: String,
        /// Unbookmark instead of bookmark
        #[arg(long)]
        remove: bool,
    },
}

impl GamesCommand {
    pub async fn run(self, lichess: Lichess, json: bool) -> Result<()> {
        match self {
            GamesCommand::ExportOne {
                game_id,
                moves,
                pgn_in_json,
                tags,
                clocks,
                evals,
                accuracy,
                opening,
                literate,
                players,
            } => {
                let query = export::one::GetQuery {
                    base: BaseArgs {
                        moves,
                        pgn_in_json,
                        tags,
                        clocks,
                        evals,
                        accuracy,
                        opening,
                        literate,
                        players,
                    }
                    .into(),
                };
                let game = lichess
                    .export_one_game(export::one::GetRequest::new(&game_id, query))
                    .await
                    .wrap_err_with(|| format!("failed to export game '{game_id}'"))?;
                output::print(&game, json);
                Ok(())
            }
            GamesCommand::ExportOngoing {
                username,
                moves,
                pgn_in_json,
                tags,
                clocks,
                evals,
                accuracy,
                opening,
                literate,
                players,
            } => {
                let query = export::ongoing::GetQuery {
                    base: BaseArgs {
                        moves,
                        pgn_in_json,
                        tags,
                        clocks,
                        evals,
                        accuracy,
                        opening,
                        literate,
                        players,
                    }
                    .into(),
                };
                let game = lichess
                    .export_ongoing_game(export::ongoing::GetRequest::new(&username, query))
                    .await
                    .wrap_err_with(|| format!("failed to export ongoing game for '{username}'"))?;
                output::print(&game, json);
                Ok(())
            }
            GamesCommand::ExportByUser {
                username,
                since,
                until,
                max,
                vs,
                rated,
                perf_type,
                color,
                analysed,
                ongoing,
                finished,
                last_fen,
                sort,
                moves,
                pgn_in_json,
                tags,
                clocks,
                evals,
                accuracy,
                opening,
                literate,
                players,
            } => {
                let query = export::by_user::GetQuery {
                    base: BaseArgs {
                        moves,
                        pgn_in_json,
                        tags,
                        clocks,
                        evals,
                        accuracy,
                        opening,
                        literate,
                        players,
                    }
                    .into(),
                    since,
                    until,
                    max,
                    vs,
                    rated,
                    perf_type: perf_type.map(Into::into),
                    color: color.map(Into::into),
                    analysed,
                    ongoing,
                    finished,
                    last_fen,
                    sort: sort.map(Into::into),
                };
                let mut stream = lichess
                    .export_games_of_user(export::by_user::GetRequest::new(&username, query))
                    .await
                    .wrap_err_with(|| format!("failed to export games of user '{username}'"))?;
                while let Some(game) = stream.next().await {
                    let game = game.wrap_err("failed to read exported game")?;
                    output::print(&game, json);
                }
                Ok(())
            }
            GamesCommand::ExportByIds {
                ids,
                moves,
                pgn_in_json,
                tags,
                clocks,
                evals,
                accuracy,
                opening,
                literate,
                players,
            } => {
                let game_ids: Vec<String> = ids.split(',').map(|s| s.trim().to_string()).collect();
                let query = export::by_ids::PostQuery {
                    base: BaseArgs {
                        moves,
                        pgn_in_json,
                        tags,
                        clocks,
                        evals,
                        accuracy,
                        opening,
                        literate,
                        players,
                    }
                    .into(),
                };
                let mut stream = lichess
                    .export_games_by_ids(export::by_ids::PostRequest::new(game_ids, query))
                    .await
                    .wrap_err("failed to export games by ids")?;
                while let Some(game) = stream.next().await {
                    let game = game.wrap_err("failed to read exported game")?;
                    output::print(&game, json);
                }
                Ok(())
            }
            GamesCommand::StreamByUsers {
                usernames,
                with_current_games,
            } => {
                let user_ids: Vec<String> =
                    usernames.split(',').map(|s| s.trim().to_string()).collect();
                let mut stream = lichess
                    .stream_games_of_users(stream::by_users::PostRequest::new(
                        user_ids,
                        with_current_games,
                    ))
                    .await
                    .wrap_err("failed to stream games of users")?;
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(event) => output::print(&event, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            GamesCommand::StreamByIds { stream_id, ids } => {
                let game_ids: Vec<String> = ids.split(',').map(|s| s.trim().to_string()).collect();
                let mut stream = lichess
                    .stream_games_by_ids(stream::by_ids::PostRequest::new(&stream_id, game_ids))
                    .await
                    .wrap_err_with(|| format!("failed to stream games for stream '{stream_id}'"))?;
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(event) => output::print(&event, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            GamesCommand::AddIds { stream_id, ids } => {
                let game_ids: Vec<String> = ids.split(',').map(|s| s.trim().to_string()).collect();
                let result = lichess
                    .add_game_ids_to_stream(stream::add_ids::PostRequest::new(&stream_id, game_ids))
                    .await
                    .wrap_err_with(|| format!("failed to add game ids to stream '{stream_id}'"))?;
                println!("Game ids added to stream '{stream_id}': {}", result);
                Ok(())
            }
            GamesCommand::Ongoing { max_games } => {
                let games = lichess
                    .get_my_ongoing_games(ongoing::GetRequest::new(max_games))
                    .await
                    .wrap_err("failed to fetch ongoing games")?;
                output::print(&games, json);
                Ok(())
            }
            GamesCommand::StreamMoves { game_id } => {
                let mut stream = lichess
                    .stream_game_moves(game_id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to stream moves of game '{game_id}'"))?;
                while let Some(event) = stream.next().await {
                    match event {
                        Ok(event) => output::print(&event, json),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(())
            }
            GamesCommand::Import { pgn } => {
                let import = lichess
                    .import_game(pgn)
                    .await
                    .wrap_err("failed to import game")?;
                output::print(&import, json);
                Ok(())
            }
            GamesCommand::ExportBookmarks {
                since,
                until,
                max,
                last_fen,
                sort,
                moves,
                pgn_in_json,
                tags,
                clocks,
                evals,
                accuracy,
                opening,
                literate,
                players,
            } => {
                let query = export::bookmarks::GetQuery {
                    base: BaseArgs {
                        moves,
                        pgn_in_json,
                        tags,
                        clocks,
                        evals,
                        accuracy,
                        opening,
                        literate,
                        players,
                    }
                    .into(),
                    since,
                    until,
                    max,
                    last_fen,
                    sort: sort.map(Into::into),
                };
                let mut stream = lichess
                    .export_bookmarked_games(export::bookmarks::GetRequest::new(query))
                    .await
                    .wrap_err("failed to export bookmarked games")?;
                while let Some(game) = stream.next().await {
                    let game = game.wrap_err("failed to read exported game")?;
                    output::print(&game, json);
                }
                Ok(())
            }
            GamesCommand::ExportImports => {
                let mut stream = lichess
                    .export_imported_games()
                    .await
                    .wrap_err("failed to export imported games")?;
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.wrap_err("failed to read imported games stream")?;
                    println!("{chunk}");
                }
                Ok(())
            }
            GamesCommand::Chat { game_id } => {
                let mut stream = lichess
                    .get_game_chat(game_id.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to fetch chat for game '{game_id}'"))?;
                while let Some(line) = stream.next().await {
                    let line = line.wrap_err("failed to read chat line")?;
                    println!("{}: {}", line.user, line.text);
                }
                Ok(())
            }
            GamesCommand::Bookmark { game_id, remove } => {
                let query = bookmark::PostQuery {
                    v: remove.then_some(false),
                };
                lichess
                    .bookmark_game(bookmark::PostRequest::new(&game_id, query))
                    .await
                    .wrap_err_with(|| format!("failed to bookmark game '{game_id}'"))?;
                println!("Bookmark toggled for game '{game_id}'");
                Ok(())
            }
        }
    }
}
