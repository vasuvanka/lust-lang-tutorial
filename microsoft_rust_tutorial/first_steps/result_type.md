# Use the Result type to handle errors

Rust provides the `Result<T, E>` enum for returning and propagating errors. By convention, the `Ok(T)` variant represents a success and contains a value, and the variant `Err(E)` represents an error and contains an error value.

The `Result<T, E>` enum is defined as:

```rs
enum Result<T, E> {
    Ok(T):  // A value T was obtained.
    Err(E): // An error of type E was encountered instead.
}
```

In contrast to the `Option` type, which describes the possibility of the absence of a value, the Result type is best suited whenever failures might occur.

## The Result type also has the unwrap and expect methods, which either:

- Return the value inside the `Ok` variant.
- Cause the program to panic, if the variant is an `Err`.

```rs
#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}
```

The output

```console
Ok(3.0)
Err(DivisionByZeroError)
Ok(0.0)
```

The `#[derive(Debug)]` part that precedes the `DivisionByZeroError` struct is a macro that tells the Rust compiler to make the type printable for debugging purposes