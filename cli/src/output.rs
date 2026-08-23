use serde::Serialize;

/// Print a value either as pretty-printed JSON or as Rust `Debug` output,
/// depending on the global `--json` flag.
pub fn print<T: Serialize + std::fmt::Debug>(value: &T, json: bool) {
    if json {
        match serde_json::to_string_pretty(value) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("failed to serialize output as json: {e}"),
        }
    } else {
        println!("{value:#?}");
    }
}
