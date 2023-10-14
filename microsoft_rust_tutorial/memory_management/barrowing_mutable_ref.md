# Borrowing and mutable references
Now we get to the real core of Rust's memory management story. Immutable and mutable references differ in one other way that has radical effects on how we build our Rust programs. When we borrow a value of any type T, the following rules apply:

Your code must implement either of the following definitions, but not both at the same time:

- One or more immutable references (&T)
- Exactly one mutable reference (&mut T)

The following code doesn't have the allowed definitions, so the compilation fails:

```rs
fn main() {
    let mut value = String::from("hello");

    let ref1 = &mut value;
    let ref2 = &mut value;

    println!("{}, {}", ref1, ref2);
}
```
output is

```console
error[E0499]: cannot borrow `value` as mutable more than once at a time
     --> src/main.rs:5:16
      |
    4 |     let ref1 = &mut value;
      |                ---------- first mutable borrow occurs here
    5 |     let ref2 = &mut value;
      |                ^^^^^^^^^^ second mutable borrow occurs here
    6 |
    7 |     println!("{}, {}", ref1, ref2);
      |                        ---- first borrow later used here
```