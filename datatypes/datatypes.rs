use std::mem;

fn main() {
    let x = 4;
    println!("x is {}", x);
    let x = 10;
    println!("x is {}", x);
    let mut z = x;
    println!("z is {}", z);
    z = z + 1;
    println!("z is {}", z);

    // integers
    let i: i8 = -1;
    println!("i with i8 type is {}", i);
    let i: i16 = 1;
    println!("i with i16 type is {}", i);
    let i: i32 = 1;
    println!("i with i32 type is {}", i);
    let i: i64 = 1;
    println!("i with i64 type is {}", i);

    // unsigned integers
    let u: u8 = 1;
    println!("u with u8 type is {}", u);
    let u: u16 = 100;
    println!("u with u16 type is {}", u);
    let u: u32 = 1000;
    println!("u with u32 type is {}", u);
    let u: u64 = 10000;
    println!("u with u64 type is {}", u);

    // floats
    let f: f32 = 1.11;
    println!("f with f32 type is {}", f);
    let f: f64 = 1001.11;
    println!("f with f64 type is {}", f);

    // chars
    let c: char = 'a';
    println!("c is {}", c);

    // booleans
    let b: bool = true;
    println!("b is {}", b);
    let b: bool = false;
    println!("b is {}", b);

    // tuples
    let tup: (i16, char, bool) = (1, 'x', false);
    println!("int value in tuple is {}", tup.0);
    println!("char value in tuple is {}", tup.1);
    println!("bool value in tuple is {}", tup.2);
    let mut tup: (i16, char, bool) = (1, 'x', false);
    tup.0 = 20;
    println!("int value in tuple is {}", tup.0);

    // arrays
    let a: [i32; 2] = [1, 2];
    println!("int value in array is {}", a[0]);
    println!("int value in array is {}", a[1]);

    // All elements can be initialized to the same value.
    let list: [i32; 500] = [-1; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", list[0]);
    println!("Second element of the array: {}", list[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", list.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&list));

    // slices
    let s = &list[1..4];
    println!("Number of elements in slice: {}", s.len());

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..s.len() + 1 {
        match s.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!, slice out of bound", i),
        }
    }

    // statement formatting
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
}
