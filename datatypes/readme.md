# Variables

[Link to Variables](https://doc.rust-lang.org/rust-by-example/primitives.html)

## Primitives
 - Rust provides access to a wide variety of primitives. A sample includes:

### Scalar Types
- Signed integers: **i8, i16, i32, i64, i128 and isize** (pointer size)
- Unsigned integers: **u8, u16, u32, u64, u128 and usize** (pointer size)
- Floating point: **f32, f64**
- char Unicode scalar values like **'a', 'α' and '∞'** (4 bytes each)
- bool either **true or false**
- The unit type (), whose only possible value is an empty tuple: ()
- Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

### Compound Types
- Arrays like [1, 2, 3]
- Tuples like (1, true)

Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to i32 and floats to f64. 
**Note that Rust can also infer types from context**.
