use std::{thread, time::Duration};

fn main() {
    //Creating a new thread with spawn
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    //When placed here, the entire spawn thread will run
    handle.join().unwrap();
    //Using the move closure
    let v = vec![1, 3, 5];
    let handle_me = thread::spawn(move || {
        println!("here's a vector {:?}", v);
    });
    handle_me.join().unwrap();
}
