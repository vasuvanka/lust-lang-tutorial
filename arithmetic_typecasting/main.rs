use std::convert::TryInto;
use std::io;

fn main() {
    println!("arithmetic & type casting ops");

    let x: u8 = 101;
    let y: u8 = 255; // note: the literal `257` does not fit into the type `u8` whose range is `0..=255`

    println!("x={},y={}", x, y);

    let a: i8 = x as i8;
    println!("a={}", a);

    let a: i8 = x.try_into().unwrap();
    println!("a={}", a);

    let f: f32 = 1.2;
    println!("f={}", f);

    let a: u8 = f as u8;
    println!("a={}", a);

    let float_value: f32 = 42.7;
    let rounded_int_value: i8 = float_value.round() as i8;

    println!("Original f32 value: {}", float_value);
    println!("Rounded i8 value: {}", rounded_int_value);

    let x = 1i8;
    let y = 2i8;

    let z = x + y;
    println!("z = {}", z);

    let x = 1i8;
    let y = 2u8;

    let z = x as u8 + y;
    println!("z = {}", z);

    let x = 1i8;
    let y = 2u8;

    let z = x as f32 / y as f32;
    println!("z = {}", z);

    let x = 100.1;
    let y = 2;

    let z = x / y as f32;
    println!("z = {}", z);

    let x = (i32::MAX as i64) + 1;
    let y = 10_00 as i32;

    println!("x is {}", x);
    let z = x as i32 + y; // here x value overflowed so will get incorrect results
    println!("z = {}", z);

    println!("Pls provide two numbers >> ");
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).expect("read line failed");
    io::stdin().read_line(&mut b).expect("read line failed");

    let x: i64 = a.trim().parse().unwrap();
    let y: i64 = b.trim().parse().unwrap();

    println!("a={} b={}", a, b);
    println!("sum = {}", x + y);

    let flag = x > y;
    let other_flag = flag && x > 10;
    println!("sum = {}", flag);
    println!("other_flag = {}", other_flag);
}
