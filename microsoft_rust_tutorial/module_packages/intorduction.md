# Understand concepts behind code organization

Before we start, it's important to explain the concepts behind code organization in Rust programs:

## A package:
- Contains functionality within one or more crates.
- Includes information about how to build those crates. The information is in the Cargo.toml file.
## A crate:
- Is a compilation unit, which is the smallest amount of code that the Rust compiler can operate on.
- Once compiled, produces either an executable or a library.
- Contains an implicit, unnamed top-level module.
## A module:
- Is a (possibly nested) unit of code organization inside a crate.
- Can have recursive definitions that span additional modules.


# Crates
Rust's compilation model centers on artifacts called crates that can be compiled into a binary or into a library.

Every project that you create with the `cargo new` command is a crate itself. All third-party Rust code that you can use as dependencies in your project is also, each, a single crate.

# Modules
Rust provides a powerful module system that can be used to hierarchically split code into logical units that also ease readability and reuse.

A module is a collection of items:

- Constants
- Type aliases
- Functions
- Structs
- Enums
- Traits
- impl blocks
- Other modules


Modules also control item privacy. Item privacy identifies an item as either public or private. Public means that the item can be used by outside code. Private means the item is an internal implementation detail and not available for outside use.

An example of a module:

```rs
mod math {
    type Complex = (f64, f64);
    pub fn sin(f: f64) -> f64 { /* ... */ }
    pub fn cos(f: f64) -> f64 { /* ... */ }
    pub fn tan(f: f64) -> f64 { /* ... */ }
}

println!("{}", math::cos(45.0));
```

If a source file has `mod` declarations in it, the contents of the module files would be inserted in places where mod declarations in the source file are found, before running the compiler over it. In other words, modules don't get compiled individually, only crates get compiled.