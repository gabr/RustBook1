struct Foo {
    i: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Droped {}", self.i);
    }
}

fn main() {
    println!("Start");

    let f7 = Foo { i: 7 };
    let f8 = Foo { i: 8 };

    {
        let f2 = Foo { i: 2 };
    } // the "Droped 2" print will happen here


    println!("End");
} // the "Droped 8" will be first, then 7

