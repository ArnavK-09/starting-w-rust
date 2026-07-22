fn main() {
    // Common unerecoverable errs
    let v = vec![1, 2, 3];
    v[99];

    // The call to panic! causes the error message contained in the last two lines
    panic!("Unrecoverable error");
}
