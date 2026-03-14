// 1. All value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be discarded.

pub fn basic_example() {
    {
        let s = String::from("Olá");
        println!("{}", s);
    }
    // println!("{}", s);
}

pub fn lifetime_example() {
    let x;
    {
        let y = String::from("Escopo interno");
        x = &y;
        println!("{} {}", x, y);
        println!("{}", x);
    }
}

pub fn move_example() {
    // Value is allocated on the stack
    let num1 = 10;
    let num2 = num1;
    println!("{} {}", num1, num2);

    // Value is allocated on the heap
    let s1 = String::from("Olá");

    // s1 was moved to s2
    let s2 = s1;

    // println!("{}", s1);
    println!("{}", s2);
}

pub fn clone_example() {
    let s1 = String::from("Olá");

    // Clone needs to be explicitly called
    let s2 = s1.clone();

    println!("{} {}", s1, s2);
}

pub fn copy_example() {
    // Fixed-size values are stored on the stack and are copied
    // Needs to implement the Copy trait
    // i32, f64, bool, char, tuples with Copy types
    let numbers_a = [1, 2, 3, 4, 5];
    let numbers_b = numbers_a;

    println!("{:?} {:?}", numbers_a, numbers_b);
}
