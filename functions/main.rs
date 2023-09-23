fn main() {
    println!("functions");
    test();
    add(1, 10);
    let num = {
        let x = 11;
        x + 1
    };
    println!("num is {}", num);

    let result = add_with_return(10, 11);
    println!("result is {}", result);
}

fn test() {
    println!("called by main");
}

fn add(x: i64, y: i64) {
    println!("sum is : {}", x + y);
}

fn add_with_return(x: i32, y: i32) -> i32 {
    x + y
}
