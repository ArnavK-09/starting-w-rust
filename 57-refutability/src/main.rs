// Refutability
// Whether a Pattern Might Fail to Match

// Refutable: can fail to match (e.g., Some(x) won't match None)
// Irrefutable: matches every value, can't fail (e.g., let x = 5;)

// The if let and while let expressions and the let...else statement accept refutable and irrefutable patterns,
// but the compiler warns against irrefutable patterns because, by definition,
// they’re intended to handle possible failure

fn main() {
    let some_option_value = Some(5);

    // refutable pattern
    // If some_option_value were a None value
    let Some(x) = some_option_value;
    // the let statement can only accept an irrefutable pattern
    // because there is nothing valid the code can do with a None value

    // To fix an error where a refutable pattern is used where an irrefutable one is required,
    // use `let...else` instead of `let`
    let Some(x) = some_option_value else {
        return;
    };

    // match arms must use refutable patterns
}
