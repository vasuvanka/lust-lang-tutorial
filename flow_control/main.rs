use std::io;

fn main() {
    println!("Enter your fav food");
    let mut b = String::new();

    io::stdin().read_line(&mut b).expect("failed to read");
    if b == "biryani" {
        println!("{} is great choice", b);
    } else if b == "burger" {
        println!("{} is good option", b)
    } else {
        println!("{} is bad option", b)
    }
}
