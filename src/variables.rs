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
