mod first;
mod operators;
mod variables;

use crate::utils::terminal::{clear_console, show_menu, wait_for_enter};

pub fn execute() {
    loop {
        let items = [
            "First Example",
            "Variables - Immutable",
            "Variables - Mutable",
            "Variables - Constant",
            "Variables - Shadowing",
            "Operators - Arithmetic",
            "Operators - Relational",
            "Operators - Logical",
        ];

        let selection = show_menu("Fundamentals", &items, false);

        clear_console();

        match selection {
            1 => first::example(),
            2 => variables::immutable(),
            3 => variables::mutable(),
            4 => variables::constant(),
            5 => variables::shadowing(),
            6 => operators::arithmetic(),
            7 => operators::relational(),
            8 => operators::logical(),
            _ => break,
        }

        wait_for_enter();
    }
}
