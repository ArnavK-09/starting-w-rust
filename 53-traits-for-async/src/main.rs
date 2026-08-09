// The Future Trait
//
// The Future trait's Output associated type defines its resolution value, like Iterator's Item type.
// Its poll method takes a pinned self reference and a Context reference, returning Poll<Self::Output>.

#![allow(unused)]
#![allow(dead_code)]

use std::pin::Pin;
use std::task::Context;

// The Pending variant indicates that the future still has work to do, so the caller will need to check again later.
// The Ready variant indicates that the Future has finished its work and the T value is available.
pub enum Poll<T> {
    Ready(T),
    Pending,
}
// When you see code that uses await, Rust compiles it under the hood to code that calls poll.
// something kind of (although not exactly): match page_title(url).await  => match page_title(url).poll()
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

// join! requires compile-time known number of futures
// join_all needs Future + Send trait bound on Vec<Fut>

// The Pin Type and the Unpin Trait
//
// pin! macro to pin the values, which means putting them inside the Pin type
// that guarantees the values won’t be moved in memory

// Pin: compiler enforces memory stability for async futures
// Wraps &mut, Box, Rc -- not reference counting, just compiler tool
// Guarantees values stay put to prevent undefined behavior during await

//  This fails - can't move futures in vectors
// let futures: Vec<Box<dyn Future>> = vec![...];

// Works - Pin prevents movement
// let futures: Vec<Pin<&mut dyn Future>> = vec![tx1_fut, rx_fut, tx_fut];
// The Pin<&mut dyn Future> type tells the compiler: "These futures won't move at runtime, so their internal self-references stay valid."

// BEFORE - compiler says "you CAN move this async block"
// async move { x = 1; await; x = 2; }

// AFTER pin! - compiler says "you CANNOT move this"
// pin!(async move { x = 1; await; x = 2; })

// Moving = Relocating in Memory

// Unpin
//
// This type is fine to relocate in memory - no self-references exist

// The Stream Trait
//
// Stream has no definition in the standard library
// very common definition from the futures crate used throughout the ecosystem.

trait Stream {
    // The Stream trait defines an associated type called Item
    type Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

// StreamExt is automatically implemented for every type that implements Stream,
// but these traits are defined separately to enable the community to iterate on
// convenience APIs without affecting the foundational trait.
trait StreamExt: Stream {
    // Like a child trait inheriting from a parent trait
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;

    // other methods...
}

// trait Child: Parent {} means Child can ONLY be implemented for types that already implement Parent
//
// Compile-time safety - can't use Child traits without Parent impl
// Method chaining works - Parent methods always available when Child is used
// Enables blanket impls - automatic extension trait implementations

fn main() {}
