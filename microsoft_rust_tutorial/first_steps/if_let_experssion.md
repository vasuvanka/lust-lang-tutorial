# The if let expression
Rust offers a convenient way to test whether a value conforms with a single pattern.

In the following example, the input to match is an Option<u8> value. The match expression should only run code if that input value is 7.

```rs
let a_number: Option<u8> = Some(7);
match a_number {
    Some(7) => println!("That's my lucky number!"),
    _ => {},
}
```

In this case, we'd like to ignore the None variant and all Some<u8> values that don't match Some(7). Wildcard patterns are useful for this type of situation. You can add the _ (underscore) wildcard pattern after all other patterns to match anything else, and it's used to satisfy the compiler demands for exhausting match arms.

To condense this code, you can use an if let expression:

```rs
let a_number: Option<u8> = Some(7);
if let Some(7) = a_number {
    println!("That's my lucky number!");
}
```