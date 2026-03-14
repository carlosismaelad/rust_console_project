// Slices allow you to reference a contiguous sequence
// of elements in a collection, rather than the entire collection.
// A slice is a kind of reference, so it does not have ownership.

pub fn example() {
    let text = String::from("Rust is a modern programming language");

    let word = first_word_starting_with('l', &text);
    println!("The word is '{}'", word);
}

// First word starting with the given letter
fn first_word_starting_with(letter: char, slice: &str) -> &str {
    let bytes = slice.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == letter as u8 {
            return slice[i..].split_whitespace().next().unwrap();
        }
    }
    slice
}
