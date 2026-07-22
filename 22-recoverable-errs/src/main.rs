// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    let open_file = std::fs::File::open("input.txt");

    let _file = match open_file {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => match std::fs::File::create("input.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => {
                panic!("Problem opening the file: {:?}", e);
            }
        },
    };

    // Same as
    // let file = File::open("input.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("input.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    // Loggin err
    let file2 = std::fs::File::open("non.txt").expect("File not found lol");
    dbg!(file2);
}
