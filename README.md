# rust-lang-tutorial

To get started install rust first using below command

```console 
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

once installation done, Open a terminal and check installation using 

```console
    rustc --version
```

## Cargo: the Rust build tool and package manager

When you install Rustup you’ll also get the latest stable version of the Rust build tool and package manager, also known as Cargo. Cargo does lots of things:

build your project with 
```console 
    cargo build
```
run your project with cargo run
```console 
    cargo run
```
test your project with cargo test
```console 
    cargo test
```
build documentation for your project with cargo doc
```console 
    cargo doc
```
publish a library to crates.io with cargo publish
```console 
    cargo publish
```
To test that you have Rust and Cargo installed, you can run this in your terminal of choice:

```console 
    cargo --version
```

(Doc link to Hello world)[https://doc.rust-lang.org/rust-by-example/hello.html]

# Project Hello-rust

Create project using 
```console
cargo new hello-rust
```

add dependency using cargo
```console
cargo add ferris-says@0.2
```

run the project using cargo run
```console
cargo run
```

to check project state
```console
cargo check
```


