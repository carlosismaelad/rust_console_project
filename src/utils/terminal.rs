use rpassword::prompt_password;
use std::io::Write;

pub fn clear_console() {
    print!("{esc}c", esc = 27 as char);
}

pub fn wait_for_enter() {
    if let Err(error) = prompt_password("Press Enter to continue...") {
        eprintln!("Failed to read input: {error}");
    }
}

pub fn show_menu(title: &str, items: &[&str], exit: bool) -> u32 {
    clear_console();
    let complete = String::from("Rust Studies :: ") + title + "\n\n";
    println!("{}", complete);
    println!("{}", String::from("=").repeat(complete.len()));

    show_items(items);

    println!("{}", if exit { "* - Exit" } else { "* - Back" });
    print!("\nChoose an option: ");
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let option: Result<u32, _> = line.trim().parse();

    match option {
        Ok(option) => option,
        _ => 0,
    }
}

pub fn show_items(items: &[&str]) {
    for (index, item) in items.iter().enumerate() {
        println!("{} - {}", index + 1, item);
    }
}
