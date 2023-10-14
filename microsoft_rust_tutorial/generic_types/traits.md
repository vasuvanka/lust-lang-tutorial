# Define shared behavior with traits

A trait is a common interface that a group of types can implement. The Rust standard library has many useful traits, such as:

- `io::Read` for values that can read bytes from a source.
- `io::Write` for values that can write out bytes.
- `Debug` for values that can be printed in the console using the "{:?}" format specifier.
- `Clone` for values that can be explicitly duplicated in memory.
- `ToString` for values that can be converted to a `String`.
- `Default` for types that have a sensible default value, like zero for numbers, empty for vectors, and “” for `String`.
- `Iterator` for types that can produce a sequence of values.

Each trait definition is a collection of methods defined for an unknown type, usually representing a capability or behavior that its implementors can do.

To represent the concept of "having a two-dimensional area," we can define the following trait:

```rs
trait Area {
    fn area(&self) -> f64;
}
```
Here, we declare a trait by using the trait keyword and then the trait's name, which is Area in this case.

Now, let's create some new types that will implement our Area trait:

```rs
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

When a type implements a given trait, it's promising to uphold its contract. After implementing the trait, we can call the methods on instances of Circle and Rectangle in the same way we call regular methods, like this:

```rs
let circle = Circle { radius: 5.0 };
let rectangle = Rectangle {
    width: 10.0,
    height: 20.0,
};

println!("Circle area: {}", circle.area());
println!("Rectangle area: {}", rectangle.area());

```

# Use the derive trait

You might have noticed that our custom types are a little difficult to use in practice. This simple `Point` struct can't be compared to other `Point` instances or displayed in the terminal. Because of this difficulty, we might want to use the derive attribute to allow new items to automatically be generated for the struct.

## Downside of generic types
Take a look at the following code example:

```rs
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 { // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1); // can't print using the '{}' format specifier!
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!

}
```
Output is

```console
error[E0277]: `Point` doesn't implement `std::fmt::Display`
      --> src/main.rs:10:20
       |
    10 |     println!("{}", p1);
       |                    ^^ `Point` cannot be formatted with the default formatter
       |
       = help: the trait `std::fmt::Display` is not implemented for `Point`
       = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
       = note: required by `std::fmt::Display::fmt`
       = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

    error[E0277]: `Point` doesn't implement `Debug`
      --> src/main.rs:11:22
       |
    11 |     println!("{:?}", p1);
       |                      ^^ `Point` cannot be formatted using `{:?}`
       |
       = help: the trait `Debug` is not implemented for `Point`
       = note: add `#[derive(Debug)]` or manually implement `Debug`
       = note: required by `std::fmt::Debug::fmt`
       = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

    error[E0369]: binary operation `==` cannot be applied to type `Point`
      --> src/main.rs:13:11
       |
    13 |     if p1 == p2 {
       |        -- ^^ -- Point
       |        |
       |        Point
       |
       = note: an implementation of `std::cmp::PartialEq` might be missing for `Point`

    error: aborting due to 3 previous errors#+end_example
```

This code fails to compile because our `Point` type doesn't implement the following traits:

- The `Debug` trait, which allows a type to be formatted by using the {:?} format specifier, is used in a programmer-facing, debugging context.
- The `Display` trait, which allows a type to be formatted by using the {} format specifier, is similar to `Debug`. But `Display` is better suited for user-facing output.
- The `PartialEq` trait, which allows implementors to be compared for equality.

# Use derive
Luckily, the `Debug` and `PartialEq` traits can be automatically implemented for us by the Rust compiler by using the `#[derive(Trait)]` attribute, if each of its fields implements the trait:

```rs
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

Now the output of the above code is

```console
not equal!
Point { x: 1, y: 2 }
```

Nevertheless, we can implement the `Display` trait for our type by ourselves:

```rs
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```
now the output is

```console
not equal!
(1, 2)
Point { x: 1, y: 2 }
```