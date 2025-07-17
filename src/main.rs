use std::ops::Range;

#[allow(unused_variables)]
fn main() {
    let value: i32 = -15;
    println!("{}", value.abs());
    println!("{}", value.pow(2));

    let empty_space = "  My Content  ";
    println!("{}", empty_space.trim());

    let eight_bit: i8 = -128;
    let eight_bit: u8 = 255;
    let sixteen_bit_signed: i16 = 32_500;
    let thirty_two_bit_signed: i32 = -2147483648;
    let thirty_two_bit_unsigned: u32 = 4294967295;

    let some_value = 20u8;

    println!("Some value: {some_value}");

    // usize
    let days: usize = 55;
    // isize
    let years: isize = -15_000;

    // float
    let pi: f32 = 3.1415901020304050607;
    println!("Current value: {}", pi.ceil());
    println!("Current value: {pi:.2}");

    // casting types
    let miles_away = 50.5;
    let miles_away_i8 = miles_away as i8;
    println!("{miles_away_i8}");

    // strings
    println!("Dear Emily,\nHow have you been?");
    println!("\tOnce upon a time");
    println!("Juliet said \"I love you Romeo\"");

    let filepath = r"C:\My Documents\new\videos";
    println!("{filepath}");

    // math operations
    let addition = 5 + 4;
    let subtraction = 5 - 4;
    let multiplication = 3 * 4;

    let floor_division = 5 / 3;
    println!("{floor_division}");

    let decimal_division = 5.0 / 3.0;
    println!("{decimal_division}");

    let remainder = 7 % 2;
    println!("Remainder: {remainder}");

    let mut year = 2025;
    year += 1;
    println!("The new year is: {year}");

    // boolean
    let active = true;
    let is_young = year < 2024;
    println!("{}", !is_young);

    // equality and inequality operators
    println!("Are those numbers equals ? {}", 26 == 25);
    println!("Are these types NOT equals ? {}", 13 != 13.2 as i32);

    // AND logic with && and OR with ||
    let purchased_ticket = true;
    let plane_on_time = true;
    let making_event = purchased_ticket && plane_on_time;
    let making_event_other_day = purchased_ticket || plane_on_time;
    println!("It is '{}' that I will arrive as expected", making_event);
    println!(
        "It is '{}' that I will arrive on another day",
        making_event_other_day
    );

    let subscription = true;
    let admin = false;
    println!("Can this user see my site? {}", subscription || admin);

    // character type
    let first_initial = 'B';
    let emoji: char = '👍';

    println!(
        "{} {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );

    // array
    let numbers: [i32; 6] = [4, 8, 15, 16, 23, 42];
    let apples = ["Granny Smith", "McIntosh", "Red Delicius"];
    println!("Length: {}", apples.len());

    let currency_rates: [f64; 0] = [];

    let mut seasons: [&str; 4] = ["primavera", "verano", "otonho", "invierno"];
    let new_season = seasons[2].to_uppercase();
    seasons[2] = new_season.as_str();
    println!("Seasons: {}", seasons[2]);

    // Display implementation
    let greetings = ["Alo", "Hola", "Hello"];

    println!("{}", 5);
    println!("{}", true);
    println!("{}", 3.14);
    println!("{greetings:?}"); // implement `std::fmt::Display
    println!("{greetings:#?}"); // # implement prettier

    // Macro
    dbg!(2 + 2);
    dbg!(greetings);

    // tuple
    let employee = ("Vicen", 40, "Developer");
    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;
    let (name, age, department) = employee;

    println!("Name: {name}, age: {age} and department: {department}");
    println!("{employee:?}");

    // range
    let month_days = 1..31;
    println!("{month_days:?}");

    let month_days = 1..=31;
    println!("{month_days:?}");

    for number in month_days {
        print!("{number}");
    }

    let letters = 'a'..='d';
    println!("");
    for letter in letters {
        print!("{letter}");
    }

    // generics
     println!("");
    let weekly_days:Range<i32> = 1..8;
    print!("{weekly_days:?}");
     println!("");
     let weekly_names:Range<char> = 'l'..'d';
     print!("{weekly_names:?}");
}
