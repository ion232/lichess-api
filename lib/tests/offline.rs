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
pub fn timeline() {
    test_response_model::<account::timeline::Timeline>("timeline");
}

#[test]
pub fn challenges() {
    test_response_model::<challenges::ChallengeJson>("challenge_json");
    test_response_model::<challenges::ChallengeOpenJson>("challenge_open_json");
    test_response_model::<challenges::ChallengeDeclinedJson>("challenge_declined_json");
}

#[test]
pub fn teams() {
    test_response_model::<teams::Team>("team");
    test_response_model::<teams::TeamPaginatorJson>("team_paginator");
    test_response_model::<teams::TeamRequestWithUser>("team_request_with_user");
    test_response_model::<teams::TeamUpdates>("team_updates");
    test_response_model::<teams::TeamUpdatesOfTeam>("team_updates_of_team");
    test_response_model::<teams::users::TeamMember>("team_member");
    test_response_model::<ArenaTournament>("arena_tournament");
    test_response_model::<SwissTournament>("swiss_tournament");
}

#[test]
pub fn arena_tournaments() {
    test_response_model::<arena_tournaments::ArenaTournaments>("arena_tournaments");
    test_response_model::<arena_tournaments::ArenaTournamentFull>("arena_tournament_full");
    test_response_model::<arena_tournaments::ArenaResult>("arena_result");
    test_response_model::<arena_tournaments::ArenaTeamStanding>("arena_team_standing");
    test_response_model::<arena_tournaments::ArenaTournamentPlayed>("arena_tournament_played");
}

#[test]
pub fn swiss_tournaments() {
    test_response_model::<swiss_tournaments::SwissResult>("swiss_result");
}

#[test]
pub fn bulk_pairings() {
    test_response_model::<bulk_pairings::BulkPairing>("bulk_pairing");
}

#[test]
pub fn studies() {
    test_response_model::<studies::create::CreateStudyResponse>("study_create");
    test_response_model::<studies::StudyMetadata>("study_metadata");
}

#[test]
pub fn broadcasts() {
    test_response_model::<broadcasts::BroadcastRound>("broadcast_round");
    test_response_model::<broadcasts::BroadcastWithRoundsAndFullGroup>("broadcast_tournament");
    test_response_model::<broadcasts::BroadcastMyRound>("broadcast_my_round");
    test_response_model::<broadcasts::BroadcastRoundNew>("broadcast_round_new");
    test_response_model::<broadcasts::BroadcastWithRounds>("broadcast_with_rounds");
    test_response_model::<broadcasts::BroadcastWithRounds>("broadcast_official");
    test_response_model::<broadcasts::BroadcastByUserPaginator>("broadcast_by_user_paginator");
    test_response_model::<broadcasts::BroadcastTop>("broadcast_top");
    test_response_model::<broadcasts::BroadcastPlayerEntryWithFideAndGames>("broadcast_player");
    test_response_model::<Vec<broadcasts::BroadcastPlayerEntry>>("broadcast_players");
    test_response_model::<Vec<broadcasts::BroadcastTeamLeaderboardEntry>>(
        "broadcast_team_leaderboard",
    );
    test_response_model::<broadcasts::BroadcastSearchPaginator>("broadcast_search_paginator");
    test_response_model::<broadcasts::BroadcastPgnPush>("broadcast_pgn_push");
}

#[cfg(feature = "oauth")]
#[test]
pub fn oauth() {
    test_response_model::<oauth::TestResults>("oauth_test_tokens");
    test_response_model::<oauth::AccessToken>("oauth_access_token");
}

#[cfg(feature = "oauth")]
#[test]
pub fn oauth_authorization_url() {
    let url = oauth::authorize::AuthorizationUrl::new("example.com", "http://example.com/", "cc")
        .scope("preference:read challenge:write")
        .state("st")
        .to_url()
        .expect("Unable to build authorization url.");

    assert_eq!(url.scheme(), "https");
    assert_eq!(url.host_str(), Some("lichess.org"));
    assert_eq!(url.path(), "/oauth");

    let params: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();

    assert_eq!(
        params.get("response_type").map(String::as_str),
        Some("code")
    );
    assert_eq!(
        params.get("client_id").map(String::as_str),
        Some("example.com")
    );
    assert_eq!(
        params.get("redirect_uri").map(String::as_str),
        Some("http://example.com/")
    );
    assert_eq!(
        params.get("code_challenge_method").map(String::as_str),
        Some("S256")
    );
    assert_eq!(params.get("code_challenge").map(String::as_str), Some("cc"));
    assert_eq!(
        params.get("scope").map(String::as_str),
        Some("preference:read challenge:write")
    );
    assert_eq!(params.get("state").map(String::as_str), Some("st"));

    // Unset optional parameters are omitted entirely.
    assert_eq!(params.get("username"), None);
}

#[cfg(feature = "oauth")]
#[test]
pub fn oauth_pkce() {
    use lichess_api::model::oauth::Pkce;

    // Test vector from RFC 7636 appendix B.
    assert_eq!(
        Pkce::derive_challenge("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"),
        "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
    );

    let pkce = Pkce::generate();

    // 32 random bytes, base64url encoded without padding.
    assert_eq!(pkce.verifier().len(), 43);
    assert!(!pkce.verifier().contains('='));
    assert_eq!(pkce.challenge(), Pkce::derive_challenge(pkce.verifier()));
    assert_ne!(pkce.verifier(), pkce.challenge());

    // Secrets must not repeat across requests.
    assert_ne!(Pkce::generate().verifier(), pkce.verifier());
}

#[cfg(feature = "oauth")]
#[test]
pub fn oauth_start_generates_secrets() {
    use lichess_api::model::oauth::authorize::AuthorizationUrl;

    let (url, pending) = AuthorizationUrl::generated("example.com", "http://example.com/")
        .scope("preference:read")
        .start()
        .expect("Unable to start authorization.");

    let params: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();

    // The generated state is what the pending authorization will check against,
    // and the challenge must never be the verifier itself.
    assert_eq!(
        params.get("state").map(String::as_str),
        Some(pending.state())
    );
    assert_eq!(
        params.get("code_challenge_method").map(String::as_str),
        Some("S256")
    );
    assert!(params.contains_key("code_challenge"));
}

#[cfg(feature = "oauth")]
#[test]
pub fn oauth_pending_authorization() {
    use lichess_api::error::Error;
    use lichess_api::model::oauth::PendingAuthorization;

    let pending =
        || PendingAuthorization::new("verifier", "st", "example.com", "http://example.com/");
    let redirect = |query: &str| {
        url::Url::parse(&format!("http://example.com/?{}", query)).expect("Unable to parse url.")
    };

    // Happy path: the form carries the verifier the challenge was derived from.
    let form = pending()
        .exchange_form(&redirect("code=abc&state=st"))
        .expect("Unable to build exchange form.");
    let encoded = serde_urlencoded::to_string(&form).expect("Unable to encode form.");

    assert!(encoded.contains("grant_type=authorization_code"));
    assert!(encoded.contains("code=abc"));
    assert!(encoded.contains("code_verifier=verifier"));

    // A mismatched state is rejected before anything else is considered.
    assert!(matches!(
        pending().exchange_form(&redirect("code=abc&state=wrong")),
        Err(Error::OAuthStateMismatch)
    ));

    // A missing state is a mismatch, not an absent check.
    assert!(matches!(
        pending().exchange_form(&redirect("code=abc")),
        Err(Error::OAuthStateMismatch)
    ));

    // Denial is surfaced with its description, not flattened into a string.
    let denied = pending().exchange_form(&redirect(
        "error=access_denied&error_description=user+cancelled&state=st",
    ));
    match denied {
        Err(Error::OAuth {
            error,
            error_description,
        }) => {
            assert_eq!(error, "access_denied");
            assert_eq!(error_description.as_deref(), Some("user cancelled"));
        }
        other => panic!("expected an oauth error, got {:?}", other.map(|_| ())),
    }

    // An error carrying a forged state is still rejected as a mismatch.
    assert!(matches!(
        pending().exchange_form(&redirect("error=access_denied&state=wrong")),
        Err(Error::OAuthStateMismatch)
    ));

    // Neither a code nor an error is malformed.
    assert!(pending().exchange_form(&redirect("state=st")).is_err());
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
