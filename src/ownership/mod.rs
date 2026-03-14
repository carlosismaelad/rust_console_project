mod mechanism;
mod reference;
mod scope;
mod slice;

use crate::utils::terminal::{clear_console, show_menu, wait_for_enter};

pub fn execute() {
    loop {
        let items = [
            "Basic Example",
            "Lifetime",
            "Move",
            "Clone",
            "Copy",
            "Moving Ownership #1",
            "Moving Ownership #2",
            "Immutable Reference",
            "Mutable Reference #1",
            "Mutable Reference #2",
            "Mutable Reference #3",
            "Dangling Reference",
            "Slice",
        ];

        let selection = show_menu("Ownership", &items, false);

        clear_console();

        match selection {
            1 => scope::basic_example(),
            2 => scope::lifetime_example(),
            3 => scope::move_example(),
            4 => scope::clone_example(),
            5 => scope::copy_example(),
            6 => mechanism::example_a(),
            7 => mechanism::example_b(),
            8 => reference::immutable_ref_example(),
            9 => reference::mutable_ref_example_a(),
            10 => reference::mutable_ref_example_b(),
            11 => reference::mutable_ref_example_c(),
            12 => reference::dangling_ref_example(),
            13 => slice::example(),
            _ => break,
        }

        wait_for_enter();
    }
}
