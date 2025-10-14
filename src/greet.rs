use std::io::{self, Write};

pub fn greet() {
    let mut name = String::new();

    print!("Enter your name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    println!("Hello, {}!", name);
}
