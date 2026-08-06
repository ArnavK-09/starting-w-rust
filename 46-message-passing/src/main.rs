// Transfer Data Between Threads with Message Passing

// A channel is a general programming concept by which data is sent from one thread to another.

// mpsc => multiple producer, single consumer
use std::time::Duration;
use std::{sync::mpsc, thread};

fn main() {
    // tx → transmit → send (transmitter)
    // rx → receive → get (receiver)
    let (tx, rx) = mpsc::channel(); //  mpsc::channel::<String>();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap()
    });

    // recv => receive
    //   which will block the main thread’s execution and wait until a value is sent down the channel.
    // try_recv, doesn’t block
    let received = rx.recv().unwrap();
    println!("Got {received}");

    // Transferring Ownership Through Channels
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        // println!("val is {val}"); // err: value borrowed here after move
        // Once the value has been sent to another thread,
        //  that thread could modify or drop it before we try to use the value again.
    });
    let received = rx.recv().unwrap();
    println!("Got {received}\n");

    // Sending Multiple Values
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
        }
    });

    for val in rx {
        println!("Recieved {val}");
    }

    // Creating Multiple Producers
    //  create multiple threads that all send values to the same receiver

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for val in rx {
        println!("Recieved Multiple {val}");
    }
}
