# Use unwrap and expect
You can try to access the inner value of an Option type directly by using the unwrap method. Be careful, though, because this method will panic if the variant is a None.

```rs
let gift = Some("candy");
assert_eq!(gift.unwrap(), "candy");

let empty_gift: Option<&str> = None;
assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!
```

The expect method does the same as unwrap, but it provides a custom panic message that's provided by its second argument:

```rs
let a = Some("value");
assert_eq!(a.expect("fruits are healthy"), "value");

let b: Option<&str> = None;
b.expect("fruits are healthy"); // panics with `fruits are healthy`
```

output is

```console
thread 'main' panicked at 'fruits are healthy', src/main.rs:6:7
```