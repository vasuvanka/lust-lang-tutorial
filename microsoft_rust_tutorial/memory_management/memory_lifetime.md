# Validate references by using lifetimes

Languages like C and C++ often have a problem where a pointer points to an item that's already been freed. This problem is known as a "dangling pointer". Fortunately, Rust eliminates this problem. It guarantees that all references always refer to valid items. But, how does it do it?

Rust's answer to this question is lifetimes. They allow Rust to ensure memory safety without the performance costs of garbage collection.

```rs
fn main() {
    let x;
    {
        let y = 42;
        x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
    }
    println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
}
```

output

```console
error[E0597]: `y` does not live long enough
     --> src/main.rs:6:17
      |
    6 |             x = &y;
      |                 ^^ borrowed value does not live long enough
    7 |         }
      |         - `y` dropped here while still borrowed
    8 |         println!("x: {}", x);
      |                           - borrow later used here
```

Here's the same code snippet with drawings around each variable lifetime. We gave each lifetime a name:

`'a` is the lifetime annotation for our value `x`.
`'b` is the lifetime annotation for our value `y`.

```rs
fn main() {
    let x;                // ---------+-- 'a
    {                     //          |
        let y = 42;       // -+-- 'b  |
        x = &y;           //  |       |
    }                     // -+       |
    println!("x: {}", x); //          |
}
```

Here we can see that the inner 'b lifetime block is shorter than the outer 'a block.

The Rust compiler can verify if the borrows are valid by using the borrow checker. The borrow checker compares the two lifetimes at compile time. In this scenario, x has a lifetime of 'a but it refers to a value with a lifetime of 'b. The reference subject (y at lifetime 'b) is a shorter time than the reference (x at lifetime 'a) so the program doesn't compile.

# Annotating lifetimes in functions
As with types, lifetime durations are inferred by the Rust compiler.

```rs
fn main() {
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);
}

fn longest_word(x: &String, y: &String) -> &String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

output is

```console
error[E0106]: missing lifetime specifier
     --> src/main.rs:9:38
      |
    9 | fn longest_word(x: &String, y: &String) -> &String {
      |                    ----        ----        ^ expected named lifetime parameter
      |
      = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
    help: consider introducing a named lifetime parameter
      |
    9 | fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
      |                ^^^^    ^^^^^^^        ^^^^^^^        ^^^
```

The help text says Rust can't tell whether the reference that's being returned refers to x or y. Neither can we. So, to help identify what the reference is, annotate the return type with a generic parameter to represent the lifetime.

It's possible that lifetimes could be different whenever the function is called. We don't know the concrete lifetimes of the references that will be passed to our longest_word function, and we can't determine if the reference that will be returned will always be a valid one.

The borrow checker can't determine if the reference will be a valid one either. It doesn't know how the input parameters' lifetime relates to the return value's lifetime. This ambiguity is why we need to annotate the lifetimes manually.

Luckily, the compiler gave us a hint on how to fix this error. We can add generic lifetime parameters to our function signature. These parameters define the relationship between the references so the borrow checker can complete its analysis:

```rs
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Make sure to declare generic lifetime parameters inside angle brackets, and add the declaration between the parameter list and the function name.

# Annotating lifetimes in types

Whenever a struct or enum holds a reference in one of its fields, we must annotate that type definition with the lifetime of each reference that it carries along with it.

For example, consider the following example code. We have a `text` string (which owns its contents) and a `Highlight` tuple struct. The struct has one field that holds a string slice. The slice is a borrowed value from another part of our program.

```rs
#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);
}
```

output is 

```console
Highlight("quick brown fox")
Highlight("lazy dog")
```

## lifetime example

```rs
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}

```