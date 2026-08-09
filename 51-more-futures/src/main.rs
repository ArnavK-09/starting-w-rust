// Yielding Control to the Runtime

use std::{thread, time::Duration};

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

// Your async code runs in **chunks**:
//
// - Code between two `await` points runs non-stop, all at once.
// - Only at an `await` does the runtime get a chance to pause you and run someone else.
fn main() {
    trpl::block_on(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
        println!("\n\n");

        // `await` = "runtime, pause me and let others run."
        // - No `await` between work = you hog the runtime.
        // - Use `yield_now()` (not fake sleeps) to politely hand over control.
        // - Cooperative multitasking: every future must be a good neighbor.

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        // `yield_now()` beats `sleep()` — clearer intent,
        // and faster: timers have a minimum granularity (this `sleep` always costs ≥1ms, even for 1ns).

        trpl::select(a, b).await;
        println!("\n\n");
        // don't yield after every line yielding isn't free.
        // Sometimes letting a task block briefly is faster overall.
    })
}
