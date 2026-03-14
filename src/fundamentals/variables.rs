pub fn immutable() {
    let x = 5;
    println!("x => {}", x);
}

pub fn mutable() {
    let mut x = 5;
    let y = x;
    println!("x, y => {} {}", x, y);

    x = 10;
    println!("x, y => {} {}", x, y);
}

pub fn constant() {
    const PI: f64 = 3.1415;
    println!("PI => {}", PI);
}

pub fn shadowing() {
    let a = 5;
    println!("a => {}", a);

    //Not allowed
    // a = "Text";

    // Allowed
    let a = "Text"; //A new declaration is allowed within the scope of a function
    println!("a => {}", a);
}
