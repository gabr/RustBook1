extern crate rand;

use std::io;
use std::io::Write; // required or 'flush()' will not be found
use rand::Rng;
//use std::cmp::Ordering; // for the match at the bottom

// --------------------------- TO PRINT TYPE --------------------------------
trait TypeInfo {
    fn type_name() -> String;
    fn type_of(&self) -> String;
}

macro_rules! impl_type_info {
    ($($name:ident$(<$($T:ident),+>)*),*) => {
        $(impl_type_info_single!($name$(<$($T),*>)*);)*
    };
}

macro_rules! mut_if {
    ($name:ident = $value:expr, $($any:expr)+) => (let mut $name = $value;);
    ($name:ident = $value:expr,) => (let $name = $value;);
}

macro_rules! impl_type_info_single {
    ($name:ident$(<$($T:ident),+>)*) => {
        impl$(<$($T: TypeInfo),*>)* TypeInfo for $name$(<$($T),*>)* {
            fn type_name() -> String {
                mut_if!(res = String::from(stringify!($name)), $($($T)*)*);
                $(
                    res.push('<');
                    $(
                        res.push_str(&$T::type_name());
                        res.push(',');
                    )*
                    res.pop();
                    res.push('>');
                )*
                res
            }
            fn type_of(&self) -> String {
                $name$(::<$($T),*>)*::type_name()
            }
        }
    }
}

impl_type_info!(i32, i64, f32, f64, str, String, Vec<T>, Result<T,S>);
// -------------- END -------- TO PRINT TYPE -------------- END -------------

fn main() {
    println!("Guess the number (range: <1; 100>)!");
    let secret = rand::thread_rng().gen_range(1, 101);
    //println!("{}", secret.type_of()); // i32

    loop {
        print!("guess: ");
        io::stdout().flush().expect("Could not flush stdout");
        let mut guess = String::new();

        //match io::stdin().read_line(&mut guess) {
        //    Ok(i) => println!("\nBytes read: {}\nGiven input: {}", i, guess),
        //    Err(e) => println!("Error: {}", e),
        //}

        // shorter version, will result in panic if Err
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        //let guess: i32 = guess.trim().parse()
        //    .expect("! Please type a number");

        //let guess: i32 = guess.trim().parse().unwrap_or(-1);

        // need to explicitly pass type
        match guess.trim().parse::<i32>() {
            Ok(g)  => {
                if g == secret {
                    println!("> OK");
                    break;
                }

                println!("> FAIL - try {} next time",
                    if g > secret { "lower" } else { "higher" });
            },
            Err(_) => println!("! Not a number - try again"),
        }

        //match guess_num.cmp(&secret) {
        //    Ordering::Less    => println!("> FAIL try higher next time"),
        //    Ordering::Greater => println!("> FAIL try lower next time"),
        //    Ordering::Equal   => {
        //        println!("> OK");
        //        break;
        //    },
        //}
    }
}
