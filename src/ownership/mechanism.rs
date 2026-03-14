pub fn example_a() {
    let nome = String::from("João");
    show_name(nome);

    // println!("{}", name);

    let idade = 30;
    show_age(idade);

    println!("{}", idade);
}

// The reference is moved to the argument
// that takes ownership of the value
fn show_name(name: String) {
    println!("{}", name);
} // name is discarded (drop is called)

// Receives a copy of the value (does not take ownership)
fn show_age(age: i32) {
    println!("{}", age);
}

pub fn example_b() {
    let name = new_name();
    println!("{}", name);

    let (name, length) = calculate_length(name);
    println!("{} has length {}", name, length);
}

#[allow(clippy::let_and_return)]
fn new_name() -> String {
    let name = String::from("Clara");
    name // ownership of the value is moved to the caller function
}

// Receives ownership of the value
fn calculate_length(name: String) -> (String, usize) {
    let length = name.len();
    (name, length) // Returns ownership of the value to the calling function
}
