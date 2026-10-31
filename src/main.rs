use std::process::ExitCode;

use moonbyte::config::Config;

fn main() -> ExitCode {
    match Config::from_environment() {
        Ok(_config) => {
            println!(
                "Moonbyte configuration validated. Discord and SQLite runtime adapters are not connected yet."
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Moonbyte startup configuration error: {error}");
            ExitCode::from(2)
        }
    }
}
