use std::time::Duration;
use trpl;

fn main() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 0..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 0..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // If you want it to run all the way to the task’s completion
        handle.await.unwrap();
        // otherwise, stops as soon as the for loop in the body of the main async block finishes,
        // because the task spawned by spawn_task is shut down when the main function ends.

        // The bigger difference is that we didn’t need to spawn another operating system thread to do this.
        println!("\n\n");

        // When you give it two futures, it produces a single new future whose output is a tuple containing the output
        // of each future you passed in once they both complete.
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        // wait for both fut1 and fut2 to finish.
        trpl::join(fut1, fut2).await;

        // unlike threads
        // hat is because the trpl::join function is fair,
        // meaning it checks each future equally often, alternating between them,
        // and never lets one race ahead if the other is ready.

        // Sending Data Between Two Tasks Using Message Passing

        // an async version of the multiple-producer
        let (tx, mut rx) = trpl::channel();
        // uses mutable rather than an immutable receiver rx

        let val = String::from("hello");

        // The synchronous Receiver::recv method in std::mpsc::channel blocks until it receives a message.
        // The trpl::Receiver::recv method does not, because it is async.
        //
        // Instead of blocking, it hands control back to the runtime until either a message is received or
        // the send side of the channel closes.
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("\n\nreceived:- {} \n\n", received);

        // series of messages
        let (tx, mut rx) = trpl::channel();
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        // Rust doesn’t yet have a way to use a for loop with an asynchronously produced series of items,
        // so introduce, the while let conditional loop
        // The loop will continue executing as long as the pattern it specifies continues to match the value.
        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // `drop(tx)` must come before the `while let` loop, not after it.
        // The loop only ends because the channel closed — so close it first.
        drop(tx);

        // all messages are received together, single async block linear
        while let Some(value) = rx.recv().await {
            println!("multi received:- {} ", value)
        }
        println!("\n\n");

        // to receive concurrency, each needs own async block
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        // Moving Ownership Into an Async Block
        let tx_fut = async move {
            let vals = vec![
                String::from("hi0"),
                String::from("from0"),
                String::from("the0"),
                String::from("future0"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        // `move` gives tx to this task, so it's dropped when the task ends → channel closes → recv() returns None → loop ends
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi1"),
                String::from("from1"),
                String::from("the1"),
                String::from("future1"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received conc:- '{value}'");
            }
        };
        trpl::join!(tx1_fut, tx_fut, rx_fut);
    })
}
