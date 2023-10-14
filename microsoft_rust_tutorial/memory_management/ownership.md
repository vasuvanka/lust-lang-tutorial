# What is ownership?

Rust includes an ownership system to manage memory. At compile time, the ownership system checks a set of rules to ensure that the ownership features allow your program to run without slowing down.

To understand ownership, let's first take a look at Rust's scoping rules and move semantics.

## Scoping rules
In Rust, like most other programming languages, variables are valid only within a certain scope. In Rust, scopes are often denoted by using curly brackets `{}`. Common scopes include function bodies and `if`, `else`, and `match` branches.

Let's say we have a `mascot` variable that's a string, defined within a scope:

```rs
// `mascot` is not valid and cannot be used here, because it's not yet declared.
{
    let mascot = String::from("ferris");   // `mascot` is valid from this point forward.
    // do stuff with `mascot`.
}
// this scope is now over, so `mascot` is no longer valid and cannot be used.
```

If we try to use `mascot` beyond its scope, we'll get an error like this example:

```rs
{
    let mascot = String::from("ferris");
}
println!("{}", mascot);
```

output is

```console
error[E0425]: cannot find value `mascot` in this scope
     --> src/main.rs:5:20
      |
    5 |     println!("{}", mascot);
      |                    ^^^^^^ not found in this scopes
```