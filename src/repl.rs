use std::io::{self, Write};
use crate::explorer::Explorer;

pub fn start(mut explorer: Explorer) {
    loop {
        print!("ironman> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input. Please try again.");
            continue;
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            continue;
        }

        match trimmed {
            "exit" => {
                println!("Exiting Ironman Explorer. Goodbye!");
                break;
            }

            "pwd" => {
                println!("{}", explorer.current_dir.display());
            }

            _ => {
                println!("Unknown command: {}", trimmed);
            }
        }
    }
}