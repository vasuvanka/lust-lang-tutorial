# Learn about fatal errors with panic!

## Panicking is the simplest error handling mechanism in Rust.

You can use the panic! macro to panic the current thread. The macro prints an error message, frees resources, and then exits the program.

```rs
fn main() {
    panic!("Farewell!");
}
```

output of the program

```console
thread 'main' panicked at 'Farewell!', src/main.rs:2:5
```

Rust panics on some operations such as a division by zero or an attempt to access an index that isn't present in an array, a vector, or a hash map, as shown in the following code:

```rs
fn main() {
    let v = vec![0, 1, 2, 3];
    println!("{}", v[6]); // this will cause a panic!
}
```