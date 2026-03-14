use std::fmt;

pub fn structs_example() {
    struct User {
        name: String,
        email: String,
        active: bool,
        age: u8,
    }

    let user = User {
        name: String::from("João"),
        email: String::from("jjj@abcdmail.com"),
        active: true,
        age: 25,
    };

    println!("user => {} {}", user.name, user.email);
    println!("user => {} {}", user.active, user.age);
}

pub fn enums_example() {
    enum Weekday {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl fmt::Display for Weekday {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Weekday::Monday => write!(f, "Monday"),
                Weekday::Tuesday => write!(f, "Tuesday"),
                Weekday::Wednesday => write!(f, "Wednesday"),
                Weekday::Thursday => write!(f, "Thursday"),
                Weekday::Friday => write!(f, "Friday"),
                Weekday::Saturday => write!(f, "Saturday"),
                Weekday::Sunday => write!(f, "Sunday"),
            }
        }
    }

    println!("day => {}", Weekday::Sunday);
    println!("day => {}", Weekday::Monday);
    println!("day => {}", Weekday::Tuesday);
    println!("day => {}", Weekday::Wednesday);
    println!("day => {}", Weekday::Thursday);
    println!("day => {}", Weekday::Friday);
    println!("day => {}", Weekday::Saturday);
}
