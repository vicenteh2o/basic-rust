#[allow(unused_variables)]
fn main() {
    let eight_bit:i8 = -128;
    let eight_bit:u8 = 255;
    let sixteen_bit_signed: i16 = 32_500;
    let thirty_two_bit_signed: i32 = -2147483648;
    let thirty_two_bit_unsigned: u32 = 4294967295;

    let some_value  = 20u8;
    
    println!("Some value: {some_value}");

    // usize
    let days: usize = 55;
    // isize
    let years: isize = -15_000;

    // strings
    println!("Dear Emily,\nHow have you been?");
    println!("\tOnce upon a time");
    println!("Juliet said \"I love you Romeo\"");

    let filepath = r"C:\My Documents\new\videos";
    println!("{filepath}");
}
