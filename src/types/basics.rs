pub fn execute() {
    let active: bool = true; // false
    println!("Boolean {}", active);

    let character: char = 'a';
    println!("Character {}", character);

    let name: &str = "Carlos Drummond De Andrade";
    println!("string {}", name);

    let mut name: String = String::from("José");
    name.push_str(" da Silva");
    println!("String {}", name);

    // i8, i16, i32, i64, i128, isize
    // u8, u16, u32, u64, u128, usize
    let quantity: i32 = 10;
    println!("Integer {}", quantity);

    // f32, f64
    let price: f64 = 10.99;
    println!("Floating point {}", price);
}
