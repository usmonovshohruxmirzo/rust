use std::fmt::Debug;

#[warn(unused_assignments)]

pub fn variables() {
    print!("========= VARIABLES ========= ");

    let x = 10;
    println!("x = {}", x);

    let mut y = 20;
    println!("{}", y);

    y = 100;
    println!("{}", y);

    // shadowing
    let a = 10;
    let a = a + 12;
    let a = a + 30;
    println!("{}", a);

    data_types();

    // Memory & Ownership Behavior

    let s1 = String::from("Hello");
    let s2 = s1; // Ownership moves to 2

    // println!("{}", s1); // s1 no longer valid
    println!("{}", s2);


    // Cloning
    let ss1 = String::from("Hello");
    let ss2 = ss1.clone();
    println!("{}, {}", ss1, ss2);

    const MAX_POINTS: u32 = 100_000;
    static HELLO: &str = "Hello, World";
}

fn data_types() {
    let int: i32 = 10;
    let float: f64 = 7.18;
    let uint: u8 = 255;
    let ch: char = 'R';
    let name: &str = "Alex";
    let is_empty: bool = true;

    let string: String = String::from("Rust");

    println!("Integer (i32): {}", int);
    println!("Float (f64): {}", float);
    println!("Unsigned Integer (u8): {}", uint);
    println!("Character (char): {}", ch);
    println!("String Slice (&str): {}", name);
    println!("Boolean (bool): {}", is_empty);
    println!("Owned String (String): {}", string);
}
