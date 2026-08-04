// The features that run these independent parts are called threads
use std::thread;
use std::time::Duration;

fn main() {
    let t = thread::spawn(|| {
        // JoinHandle<T>. A JoinHandle<T> is an owned value that,
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Note that when the main thread of a Rust program completes, all spawned threads are shut down,
    //  whether or not they have finished running.

    // t.join().unwrap(); // blocks main so prioritize t to complete and then run for

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Waiting for All Threads to Finish
    t.join().unwrap(); // Main thread: wait until this spawned thread finishes

    // `join()` waits at the exact line where you call it
    //
    // - before the loop → spawned thread finishes first
    // - after the loop → both loops can run together, then main waits

    // Using move Closures with Threads
    //
    // We’ll often use the move keyword with closures passed to thread::spawn
    // because the closure will then take ownership of the values it uses from the environment,
    // thus transferring ownership of those values from one thread to another

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
        v // returns ownership
    });
    // drop(v); // err: value used here after move
    // println!("Here's a vector: {v:?}"); // err: value used here after move

    let v = handle.join().unwrap();
    println!("Here's a vector2: {v:?}");
    drop(v);
}
