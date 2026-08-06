// Shared-State Concurrency
//
// Channels are like single ownership: once a value is sent, it can no longer be used.
// Shared-memory concurrency is like multiple ownership: many threads can access the same memory simultaneously.

// Mutex is an abbreviation for mutual exclusion,
//  A mutex allows only one thread to access some data at any given time.
//  To access the data, a thread must first acquire the mutex's lock.
//  The lock is a data structure that tracks who currently has exclusive access.
//  Thus, the mutex guards the data it holds via this locking mechanism.
use std::sync::Mutex;
use std::thread;

// Atomic Reference Counting with Arc<T>
use std::sync::Arc;
// a stands for atomic, meaning it’s an atomically reference-counted type

// Mutexes have a reputation for being difficult to use because you have to remember two rules:
//
// - You must attempt to acquire the lock before using the data.
// - When you’re done with the data that the mutex guards, you must unlock the data so that other threads can acquire the lock

fn main() {
    let m = Mutex::new(5);
    dbg!(&m);
    {
        // This call will block the current thread so that it can’t do any work until it’s our turn to have the lock.
        let mut n = m.lock().unwrap(); // To access the data inside the mutex, we use the lock method to acquire the lock
        // The call to lock would fail if another thread holding the lock panicked.
        //  so we’ve chosen to unwrap and have this thread panic if we’re in that situation.

        // After acquiring the lock, we can treat the return value, named num in this case, as a mutable reference to the data inside.
        *n = *n + 10;

        // The MutexGuard type implements Deref to point at our inner data
        // the type also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope.
    }
    dbg!(&m);

    // Shared Access to Mutex<T>

    // Multiple Ownership with Multiple Threads

    // Arc<T> is a type like Rc<T> that is safe to use in concurrent situations.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    dbg!(&counter);

    // Rc<T> is not safe to share across threads because its reference count operations are not atomic.
    // Each clone increments the count and each drop decrements it, but without atomicity, race conditions can occur.
    // This can lead to incorrect counts, causing memory leaks or premature deallocation.

    // Mutex<T> comes with the risk of creating deadlocks.
    // A deadlock occurs when two or more threads are waiting for each other to release a lock,
    // causing all of them to be stuck indefinitely.
}
