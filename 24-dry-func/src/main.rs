fn largest(list: &[i32]) -> i32 {
    let mut _largest = list[0];

    for &item in list.iter() {
        if item > _largest {
            _largest = item;
        }
    }

    _largest
}

// with generics 
fn largest2<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut _largest = list[0];

    for &item in list {
        if item > _largest {
            _largest = item;
        }
    }

    _largest
}

fn main() {
    // Generics allow us to replace specific types
    // with a placeholder that represents multiple types to remove code duplication

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");
}
