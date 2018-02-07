fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let i: usize = 0;
    //let j: i32 = 0;

    // works
    println!("{}", v[i]);

    // doesn't work
    //println!("{}", v[j]);

    // panic:
    //println!("{}", v[9]);


    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }

    for i in v {
        println!("{}", i);
    }

    // cannot use v any more after passing ovnership to the for loop
    // use to borrow the vector:
    //
    //      for i in &v {
    //          println!("{}", i);
    //      }
}
