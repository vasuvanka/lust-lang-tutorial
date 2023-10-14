# Ownership and dropping
Rust adds a twist to the idea of scopes. Whenever an object goes out of scope, it's "dropped." Dropping a variable releases any resources that are tied to it. For variables of files, the file ends up being closed. For variables that have allocated memory associated with them, the memory is freed.

In Rust, bindings that have things "associated" with them that they'll free when the binding is dropped are said to "own" those things.

In the previous example, the `mascot` variable owns the `String` data associated with it. The String itself owns the heap-allocated memory that holds the characters of that string. At the end of the scope, `mascot` is "dropped", the `String` it owns is dropped, and finally the memory that `String` owns is freed.

```rs
{
    let mascot = String::from("ferris");
    // mascot dropped here. The string data memory will be freed here.
}
```

# Move semantics
Sometimes, though, we don't want the things associated with a variable to be dropped at the end of scope. Instead, we want to transfer ownership of an item from one binding to another.

The simplest example is when declaring a new binding:

```rs
{
    let mascot = String::from("ferris");
    // transfer ownership of mascot to the variable ferris.
    let ferris = mascot;
    // ferris dropped here. The string data memory will be freed here.
}
```

In Rust, "transferring ownership" is known as "moving". In other words, the ownership of the `String` value has been moved from `mascot` to `ferris`.

```rs
{
    let mascot = String::from("ferris");
    let ferris = mascot;
    println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
}
```
Output is

```console
error[E0382]: borrow of moved value: `mascot`
 --> src/main.rs:4:20
  |
2 |     let mascot = String::from("ferris");
  |         ------ move occurs because `mascot` has type `String`, which does not implement the `Copy` trait
3 |     let ferris = mascot;
  |                  ------ value moved here
4 |     println!("{}", mascot);
  |                    ^^^^^^ value borrowed here after move
```

This result is known as a "use after move" compile error.