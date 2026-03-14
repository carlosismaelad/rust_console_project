mod basics;
mod custom;
mod sequencies;

use crate::utils::terminal::{clear_console, show_menu, wait_for_enter};

pub fn execute() {
    loop {
        let items = ["Basic", "Sequencies", "Custom - Structs", "Custom - Enums"];

        let selection = show_menu("Types", &items, false);

        clear_console();

        match selection {
            1 => basics::execute(),
            2 => sequencies::execute(),
            3 => custom::structs_example(),
            4 => custom::enums_example(),
            _ => break,
        }

        wait_for_enter();
    }
}
