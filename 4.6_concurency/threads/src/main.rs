use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn main() {
    thread::spawn(|| {
        println!("Hello, threads world!");
    });



    let handle = thread::spawn(|| {
        "Hello from handle"
    });

    println!("{}", handle.join().unwrap());


    let x = 1;
    // Need to use move closure to not use reference to x.
    // The reference cannot be used because of dangling pointer possibility.
    thread::spawn(move || {
        println!("x = {}", x);
    }).join().unwrap();


    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut threads = Vec::new();

    for i in 0..3 {
        let data = data.clone();
        threads.push(thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += i;
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("{:?}", data);
    {
        assert_eq!(4, data.lock().unwrap()[0]);
    }

    let data2 = Arc::new(Mutex::new(0));

    // `tx` is the "transmitter" or "sender".
    // `rx` is the "receiver".
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data2.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }

    println!("{:?}", data2);

    let panic_handle = thread::spawn(move || {
        panic!("oops!");
    });

    let result = panic_handle.join();

    assert!(result.is_err());
    println!("{}", result.is_err());
}

