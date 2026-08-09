// Streams: Futures in Sequence
//
// stream => producing a sequence of items over time

// The similarity between iterators and streams in Rust
// means we can actually create a stream from any iterator

use trpl;

// right trait in scope to be able to use the next method.
use trpl::StreamExt;
// StreamExt supplies a higher-level set of APIs on top of Stream,
// including the next method as well as other utility methods similar to
// those provided by the Iterator trait

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);

        // convert the iterator into a stream
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(val) = stream.next().await {
            println!("The value was: {val}");
        }
    })
}
