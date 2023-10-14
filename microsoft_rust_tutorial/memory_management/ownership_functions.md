# Ownership in functions
Let's take a look at an example of a string being passed to a function as an argument. Passing something as an argument to a function moves that thing into the function.

```rs
fn process(input: String) {}

fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Ownership of the string in `s` moved into `process`
    process(s); // Error! ownership already moved.
}
```

The compiler complains that the value `s` was moved.

```console
error[E0382]: use of moved value: `s`
     --> src/main.rs:6:13
      |
    4 |     let s = String::from("Hello, world!");
      |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
    5 |     process(s); // Transfers ownership of `s` to `process`
      |             - value moved here
    6 |     process(s); // Error! ownership already transferred.
      |             ^ value used here after move
```

# Copying instead of moving
In the previous example, you might have noticed the mention of the Copy trait in the (rather informative) compiler error messages. We haven't talked about traits yet, but a value that implements the Copy trait, isn't moved but is copied.

Let's take a look at a value that implements the Copy trait: `u32`. The following code mirrors our broken code, but it compiles without issue.

```rs
fn process(input: u32) {}

fn caller() {
    let n = 1u32;
    process(n); // Ownership of the number in `n` copied into `process`
    process(n); // `n` can be used again because it wasn't moved, it was copied.
}
```

Simple types like numbers are copy types. They implement the `Copy` trait, which means they're copied rather than moved. The same action occurs for most simple types. Copying numbers is inexpensive, so it makes sense for these values to be copied. Copying strings or vectors or other complex types can be expensive, so they don't implement the `Copy` trait and are instead moved.

# Copying types that don't implement `Copy`

One way to work around the errors we saw in the previous example is by explicitly copying types before they're moved: known as cloning in Rust. A call to `.clone` will duplicate the memory and produce a new value. The new value is moved meaning the old value can still be used.

```rs
fn process(s: String) {}

fn main() {
    let s = String::from("Hello, world!");
    process(s.clone()); // Passing another value, cloned from `s`.
    process(s); // s was never moved and so it can still be used.
}
```
This approach can be useful, but it can make your code slower as every call to `clone` makes a full copy of the data.