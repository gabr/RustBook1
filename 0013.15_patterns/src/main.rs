fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 2, y: 3 };

    match p {
        Point { x: 2, y } => println!("{}", y),
        _ => println!("???"),
    }
}
