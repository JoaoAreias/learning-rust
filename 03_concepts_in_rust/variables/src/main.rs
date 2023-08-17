use std::error;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}!");

    x = 6;
    println!("The value of x is {x}!");

    // Consts must be type annotated
    const PI: f64 = 3.14159265359;

    // Shadowing
    let y = "5";
    let y = 5;

    // Integer types
    let a: u8   = b'A'; // Byte (u8 only)
    let b: i16  = 0xff;
    let c: i32  = 0b000_1111;
    let d: i64  = 0o77;
    let e: i128 = 123456;

    // Tuples
    let tup = ("João", 12);
    let (user, value) = tup;
    let value = tup.1;

    // Arrays
    let error_codes = [400, 404, 403];
    let not_found = error_codes[1];

}


fn my_function() {
    println!("Another function");
}