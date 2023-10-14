# Pattern matching
There's a powerful operator in Rust that's called match. You can use it to control the flow of your program by providing patterns. When match finds a matching pattern, it runs code that you supply with that pattern.

```rs
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
        Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
        None => println!("There is no fruit! :("),
    }
}
```

output is

```console
It's a delicious banana!
It's a delicious coconut!
There is no fruit! :(
```

You can refine your match expression even further to act differently, depending on the values inside a Some variant. For example, you could stress the fact that coconuts are awesome by running the following code:

```rs
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
        Some(&"coconut") => println!("Coconuts are awesome!!!"),
        Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
        None => println!("There is no fruit! :("),
    }
}
```

output is 

```console
It's a delicious banana!
Coconuts are awesome!!!
There is no fruit! :(
```