fn main() {
    println!("hello world!")

    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 5);
}
