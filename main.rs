mod cat;
mod echo;
mod pwd;
mod touch;
mod rmf;
mod datetime;
mod guessing_game;
mod calc;

use std::env;

fn main() {
    let command = env::args().nth(1);

    match command.as_deref() {
        Some("cat") => cat::run(),
        Some("echo") => echo::run(),
        Some("pwd") => pwd::run(),
        Some("touch") => touch::run(),
        Some("rmf") => rmf::run(),
		Some("datetime") => datetime::run(),
		Some("guessing_game") => guessing_game::run(),
		Some("calc") => calc::run(),
        _ => {
            eprintln!("Error: Command not found or no command specified.");
			
            std::process::exit(1);
        }
    }
}
