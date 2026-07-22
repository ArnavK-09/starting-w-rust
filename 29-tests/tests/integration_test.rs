// Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope
use tests::add;

// We don’t need to annotate any code in tests dir with #[cfg(test)]

// Note that the mod common; declaration is the same as the module declaration
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    let result = add(2, 2);
    assert_eq!(result, 4);
}
