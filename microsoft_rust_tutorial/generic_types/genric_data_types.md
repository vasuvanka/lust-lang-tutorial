# What are generic data types?

A generic data type is a type that's defined in terms of other, partially unknown types. We've been using many generic data types since the beginning of this course, for example:

The `Option<T>` enum is generic over the type T, which is the value contained by its Some variant.
The `Result<T, E>` is generic over both its success and failure type, contained by its `Ok` and `Err` variants, respectively.
The vector type `Vec<T>`, the array type `[T; n]`, and the hash map `HashMap<K, V>` are generic over the types they contain.
When we use generic types, we can specify the operation we want without many concerns about the inner types held by the defined type.

```rs
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let boolean = Point { x: true, y: false };
    let integer = Point { x: 1, y: 9 };
    let float = Point { x: 1.7, y: 4.3 };
    let string_slice = Point { x: "high", y: "low" };
}
```

The preceding code defines a `Point<T>` struct. This struct holds any type (`T`) of `x` and `y` values.

We can use multiple generic type parameters. In this case, we show a `Point<T, U>` generic over two types so that `x` and `y` can be values of different types.

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer_and_boolean = Point { x: 5, y: false };
    let float_and_string = Point { x: 1.0, y: "hey" };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let both_integer = Point { x: 10, y: 30 };
    let both_boolean = Point { x: true, y: true };
}
```