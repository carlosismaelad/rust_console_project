mod fundamentals;
mod types;
mod utils;
mod conditional_structures;

use std::process::exit;

use crate::utils::terminal::{clear_console, show_menu};
fn main() {
    loop {
        let items = [
            "Fundamentals",
            "Types",
            "Conditional Structures",
            "Functions",
            "Ownership",
        ];
        let selection = show_menu("Main Menu", &items, true);

        clear_console();

        match selection {
            1 => fundamentals::execute(),
            2 => types::execute(),
            3 => conditional_structures::execute(),
            4 => println!("4"),
            5 => println!("5"),
            _ => exit(0),
        }
    }
}
