mod conditionals;
mod loops;

use crate::utils::terminal::{clear_console, show_menu, wait_for_enter};

pub fn execute() {
    loop {
        let items = [
            "Conditionals",
            "For - Range",
            "For - Array",
            "While",
            "Loop",
        ];
        let selecionado = show_menu("Conditional Structures", &items, false);

        clear_console();

        match selecionado {
            1 => conditionals::exemplo(),
            2 => loops::range_example(),
            3 => loops::array_example(),
            4 => loops::while_example(),
            5 => loops::loop_example(),
            _ => break,
        }

        wait_for_enter();
    }
}
