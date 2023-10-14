# Use the Option type to deal with absence

The Rust standard library provides an Option<T> enum to be used when the absence of a value is a possibility. Option<T> is widely used in Rust code. It's useful for working with values that might exist or that might be empty.

In many other languages absence of a value would be modeled using null or nil, but Rust doesn't use null outside of code that interoperates with other languages. Rust is explicit about when a value is optional. While in many languages, a function that takes a String might actually take either a String or null, in Rust, that same function can only take an actual String. If you want to model an optional string in Rust, you need to explicitly wrap it in an Option type: Option<String>.

## Option<T> manifests itself as one of two variants:

```rs
enum Option<T> {
    None,     // The value doesn't exist
    Some(T),  // The value exists
}
```

we mentioned that trying to access a vector's non-existent index would cause the program to panic, but you could avoid that by using the Vec::get method, which returns an Option type instead of panicking. If the value exists at a specified index, it's wrapped in the Option::Some(value) variant. If the index is out of bounds, it would return an Option::None value instead.

```rs
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

// pick the first item:
let first = fruits.get(0);
println!("{:?}", first);

// pick the third item:
let third = fruits.get(2);
println!("{:?}", third);

// pick the 99th item, which is non-existent:
let non_existent = fruits.get(99);
println!("{:?}", non_existent);
```

output of the program:

```console
Some("banana")
Some("coconut")
None
```