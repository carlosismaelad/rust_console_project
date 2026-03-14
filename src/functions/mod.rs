mod fns;
mod lambda;

use crate::utils::terminal::{clear_console, show_menu, wait_for_enter};

pub fn execute() {
    loop {
        let items = ["Basics", "Map", "Filter"];

        let selection = show_menu("Functions", &items, false);

        clear_console();

        match selection {
            1 => fns::example(),
            2 => lambda::map_example(),
            3 => lambda::filter_example(),
            _ => break,
        }

        wait_for_enter();
    }
}
