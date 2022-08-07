use std::{
    sync::{mpsc, Mutex},
    thread,
    time::Duration,
};

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
    //Using channels to pass messages
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("there"),
            String::from("from "),
            String::from("Github"),
        ];
        for i in vals {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //Second thread
    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you!"),
        ];
        for i in vals {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!(" {}", received);
    }
    //Shared state concurrency
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m= {:?}", m);
}
