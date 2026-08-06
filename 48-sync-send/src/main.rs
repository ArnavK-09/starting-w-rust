// Send = safe to MOVE to another thread (e.g. i32, String, Arc)
// Sync = safe to SHARE with multiple threads (e.g. i32, Mutex, Arc)

// Rc<T>      -> NOT Send/Sync (ref count not thread-safe) -> use Arc<T>
// RefCell<T> -> NOT Sync (runtime borrow check is single-threaded) -> use Mutex<T>
// Arc<T>     -> Send + Sync (thread-safe Rc)

// Auto traits: if all parts are Send/Sync, the type is too.
// Manual impl requires `unsafe` — avoid it.

fn main() {}
