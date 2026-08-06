// A future is a value that may not be ready now but will become ready at some point in the future.

// The `async` keyword marks blocks and functions as pausable and resumable.
// Inside such contexts, `await` suspends execution until a future is ready.
// Each `await` point is a potential suspension point.

// Polling is the act of checking whether a future's value is available.

// use the async and await keywords most of the time. Rust compiles them into equivalent code using the Future trait.

#![allow(dead_code)]

// trpl crate (trpl is short for “The Rust Programming Language”).
//  It re-exports all the types, traits, and functions you’ll need, primarily from the futures and tokio crates.
use std::future::Future;
use trpl::{Either, Html};

// When Rust sees a function marked with async,
//  it compiles it into a non-async function whose body is an async block.
async fn page_title(url: &str) -> Option<String> {
    // futures in Rust are lazy: they don’t do anything until you ask them to with the await keyword.
    let resp = trpl::get(url).await;
    let txt = resp.text().await;

    // await keyword goes after the expression you’re awaiting, not before it.
    // It’s a postfix keyword

    Html::parse(&txt)
        .select_first("title")
        .map(|title| title.inner_html())
}
// same as
fn page_title2(url: &str) -> impl Future<Output = Option<String>> {
    // When Rust sees a block marked with the async keyword,
    //  it compiles it into a unique, anonymous data type that implements the Future trait.
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

// Executing an Async Function with a Runtime
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];

    // The only place we can use the await keyword is in async functions or blocks,
    // and Rust won’t let us mark the special main function as async.
    //
    // The reason main can’t be marked async is that async code needs a runtime:
    //  Rust crate that manages the details of executing asynchronous code.

    // blocks the current thread until this future runs to completion
    trpl::block_on(async {
        match page_title(url).await {
            Some(title) => {
                println!("{}", title.trim());
            }
            None => println!("url not found"),
        }
    });

    // Compiler creates & manages the state machine data structures for async code automatically.

    // Racing Two URLs Against Each Other Concurrently
    async fn page_title3(url: &str) -> (&str, Option<String>) {
        let response_text = trpl::get(url).await.text().await;
        let title = Html::parse(&response_text)
            .select_first("title")
            .map(|title| title.inner_html());
        (url, title)
    }
    println!("\n************\n");
    trpl::block_on(async {
        // save the resulting futures
        // these don’t do anything yet, because futures are lazy and we haven’t yet awaited them
        let t1 = page_title3(&args[1]);
        let t2 = page_title3(&args[2]);

        // The select function returns
        // Left with that future’s output if the first argument wins, and
        // Right with the second future argument’s output if that one wins.
        let (url, title) = match trpl::select(t1, t2).await {
            Either::Left(left) => {
                println!("left");
                left
            }
            Either::Right(right) => {
                println!("right");
                right
            }
        };

        println!("{url} returned first");

        match title {
            Some(title) => println!("Its page title was: '{}'", title.trim()),
            None => println!("It had no title."),
        }
    })
}
