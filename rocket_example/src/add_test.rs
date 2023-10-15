fn add(a: i32,b:i32) -> i32 {
    a+b
}


#[test]
fn add_works(){
    assert_eq!(add(1,1), 2)
    assert_eq!(add(1,10), 11)
    assert_eq!(add(11,-2), 9)
}