fn main() {
    println!("Hello, world!");
}

#[test]
#[should_panic(expected = "assertion failed")]
fn it_works() {
    assert!(true);
    assert_eq!(1, 2); // panics
}

#[test]
#[ignore]
fn math_works() {
    // cargo test -- --ignored
    assert_eq!(4, add_two(2));
}

fn add_two(x: i32) -> i32 {
    x + 2
}
