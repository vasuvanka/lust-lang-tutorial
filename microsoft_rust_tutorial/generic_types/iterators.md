# Use iterators

We've already covered how we can iterate over collection types by using the loop. This time, we'll do a more in-depth review on how Rust handles the concept of iteration itself.

In Rust, all iterators implement a trait named `Iterator` that's defined in the standard library and is used to implement iterators over collections such as ranges, arrays, vectors, and hash maps.

The core of that trait looks like this:

```rs
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

An `Iterator` has a method, `next`, which when called returns an `Option<Item>`. The next method will return `Some(Item)` as long as there are elements. After they've all been exhausted, it will return `None` to indicate that iteration is finished.