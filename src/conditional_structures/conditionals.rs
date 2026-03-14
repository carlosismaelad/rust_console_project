// Annotation used to silence the warning that 
// instructs to use a match instead of nested if statements:
// `if` can be rewritten with `match` (clippy::comparison_chain)
#[allow(clippy::comparison_chain)]

pub fn exemplo() {
    let x = 10;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
    }
}