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
            1 => println!("First Example"),
            _ => break,
        }

        wait_for_enter();
    }
}
