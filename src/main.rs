mod fundamentals;
mod types;
mod utils;

use std::process::exit;

use crate::utils::terminal::{clear_console, show_menu};
fn main() {
    loop {
        let items = [
            "Fundamentals",
            "Types",
            "Controller",
            "Functions",
            "Ownership",
        ];
        let selection = show_menu("Main Menu", &items, true);

        clear_console();

        match selection {
            1 => fundamentals::execute(),
            2 => types::execute(),
            3 => println!("3"),
            4 => println!("4"),
            5 => println!("5"),
            _ => exit(0),
        }
    }
}
