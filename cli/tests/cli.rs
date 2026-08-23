use assert_cmd::Command;
use predicates::prelude::*;

fn lichess() -> Command {
    Command::cargo_bin("lichess").unwrap()
}

#[test]
fn top_level_help_lists_all_categories() {
    lichess()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("board"))
        .stdout(predicate::str::contains("puzzles"))
        .stdout(predicate::str::contains("engine"))
        .stdout(predicate::str::contains("challenges"))
        .stdout(predicate::str::contains("users"));
}

#[test]
fn no_subcommand_fails_with_usage() {
    lichess().assert().failure();
}

#[test]
fn unknown_subcommand_fails() {
    lichess().arg("not-a-real-command").assert().failure();
}

#[test]
fn subcommand_help_succeeds_for_every_category() {
    for subcommand in ["board", "puzzles", "engine", "challenges", "users"] {
        lichess().args([subcommand, "--help"]).assert().success();
    }
}

#[test]
fn users_performance_rejects_invalid_perf_type() {
    lichess()
        .args(["users", "performance", "some-user", "not-a-real-perf"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn users_leaderboard_rejects_invalid_perf_type() {
    lichess()
        .args(["users", "leaderboard", "not-a-real-perf"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn users_autocomplete_rejects_short_term() {
    lichess()
        .args(["users", "autocomplete", "ab"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("at least 3 characters"));
}

#[test]
fn board_create_seek_rejects_invalid_variant() {
    lichess()
        .args(["board", "create-seek", "--variant", "not-a-real-variant"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn board_write_chat_rejects_invalid_room() {
    lichess()
        .args([
            "board",
            "write-chat",
            "some-game-id",
            "--room",
            "not-a-real-room",
            "hello",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn challenges_create_rejects_invalid_variant() {
    lichess()
        .args([
            "challenges",
            "create",
            "some-user",
            "--variant",
            "not-a-real-variant",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn challenges_decline_rejects_invalid_reason() {
    lichess()
        .args([
            "challenges",
            "decline",
            "some-challenge-id",
            "--reason",
            "not-a-real-reason",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn puzzles_next_rejects_invalid_difficulty() {
    lichess()
        .args(["puzzles", "next", "--difficulty", "not-a-real-difficulty"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn json_flag_is_accepted_globally() {
    // --json is a global flag; it should parse successfully even placed after the subcommand,
    // without requiring network access (help exits before any request is made).
    lichess()
        .args(["--json", "users", "--help"])
        .assert()
        .success();
}
