// 1. At any given time, you can have either one mutable reference
// or any number of immutable references.
// 2. References must always be valid.

pub fn immutable_ref_example() {
    let text = String::from(
        "Rust is a modern systems programming language 
        that runs incredibly fast, prevents segmentation faults, 
        and guarantees concurrent safety.",
    );

    let word_count = calculate_word_count(&text);
    println!("The text '{}' has {} words", text, word_count);
}

#[allow(clippy::ptr_arg)]
// Receives a reference to the text (does not take ownership)
// Borrows the text
fn calculate_word_count(text: &String) -> usize {
    text.split_whitespace().count()
    // text is discarded (drop is not called because it does not have ownership)
}

pub fn mutable_ref_example_a() {
    let name = String::from("João");
    last_name(&name);
    println!("Name: {}", name);
}

#[allow(clippy::ptr_arg)]
// Cannot be borrowed as mutable
fn last_name(text: &String) {
    // text.push_str(" da Silva");
    println!("{}", text.len());
}

pub fn mutable_ref_example_b() {
    let mut name = String::from("João");
    mutable_last_name(&mut name);
    println!("Name: {}", name);

    let n1 = &mut name;
    println!("{}", n1);

    let n2 = &mut name;
    println!("{}", n2);
}

// Can be borrowed as mutable
fn mutable_last_name(text: &mut String) {
    text.push_str(" da Silva");
}

pub fn mutable_ref_example_c() {
    let mut name = String::from("Gabriel");

    // immutable - no problem
    let n1 = &name;
    let n2 = &name;

    println!("{} {}", n1, n2);

    // mutable - problem if order is changed
    let n3 = &mut name;

    println!("{}", n3);
}

pub fn dangling_ref_example() {
    // let dangling_ref = generate_dangling();
    // println!("Dangling Reference {}", dangling_ref);

    let no_dangling = no_dangling();
    println!("No Dangling {}", no_dangling);
}

// fn generate_dangling() -> &String {
//     let text = String::from("Dangling Reference");
//     &text // returns a reference that will be discarded
// }

#[allow(clippy::let_and_return)]
fn no_dangling() -> String {
    let text = String::from("No Dangling");
    text // returns a reference that will be moved to the calling function
}
