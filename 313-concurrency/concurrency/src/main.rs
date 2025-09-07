use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    //   let handle = thread::spawn(move || {
    //       println!("Hello from thread");
    //   });
    //   handle.join().unwrap();
    // thread::sleep(Duration::from_secs(2));
    // Create multiple threads
    //    let mut handles = Vec::new();
    //    for i in 0..5 {
    //        let handle = thread::spawn(move || {
    //            println!("Hello from thread {}", i);
    //        });
    //        handles.push(handle);
    //    }
    //    for handle in handles {
    //        handle.join().unwrap();
    //         }
    // For MPSC
    let (transmitter, reciever) = mpsc::channel();
    let new_transmitter: Sender<String> = transmitter.clone();

    thread::spawn(move || {
        transmitter
            .send(String::from("Hi from transmitter"))
            .unwrap();
    });
    // for multipule thread

    thread::spawn(move || {
        let vec = vec![
            String::from("Main one"),
            String::from("Main two"),
            String::from("Main three"),
            String::from("Main four"),
            String::from("Main five"),
        ];

        for val in vec {
            new_transmitter.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for recieved in reciever {
        println!("{}", recieved);
    }

    let lock = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..8 {
        let counter = Arc::clone(&lock);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }); //lock is given up
        handles.push(handle);
    }
}
