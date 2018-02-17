fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let black_r = black.0;
    let Point(_, origin_y, origin_z) = origin;
    let (x, y, z) = (1, 2, 3);

    println!("{}", origin_y);
    println!("{}, {}, {}", x, y, z);
}
