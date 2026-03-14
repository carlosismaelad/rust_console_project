mod conditional_structures;
mod functions;
mod fundamentals;
mod ownership;
mod types;
mod utils;

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
            4 => functions::execute(),
            5 => ownership::execute(),
            _ => exit(0),
        }
    }
}
