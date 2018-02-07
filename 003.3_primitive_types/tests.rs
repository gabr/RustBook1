fn main() {
    let a: [i32; 10] = [0; 10];
    println!("{}", a.len());

    // tuple
    let (x, y, z) = (1, 2, 3);
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    // (0,); // single element tuple
    // (0);  // zero in parentheses
    let tuple = ('a', 2, "CCC");
    println!("{}", tuple.2);

    fn foo(x: i32) -> i32 { x+x }
    let x: fn(i32) -> i32 = foo;
    let y = |x: i32| x+x;
    println!("{}", x(2));
    println!("{}", y(2));
}
