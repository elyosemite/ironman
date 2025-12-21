use std::io::{self, Write};
use crate::explorer::Explorer;

pub fn start(explorer: Explorer) {
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

            "clean" | "cls" => {
                clear_screen()
            }

            _ => {
                println!("Unknown command: {}", trimmed);
            }
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}