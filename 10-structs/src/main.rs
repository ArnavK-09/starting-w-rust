struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct AlwaysEqual;
#[allow(dead_code)]
struct RGB(i32, i32, i32);

fn main() {
    let mut p1: Person = Person {
        name: String::from("Peter"),
        age: 27,
    };
    let p2 = new_person("Jane", 0);
    println!("Hello, world! \n {:#?} {:?}", p1.name, p1.age);
    p1.age += 1;
    let p3 = Person { age: 10, ..p1 };
    println!("Hello, world! \n {:#?} {:?}", p2.name, p2.age);
    println!("Hello, world! \n {:#?} {:?}", p3.name, p3.age);

    let x = AlwaysEqual;
    println!("{:?}", x);

    let y = RGB(0, 255, 0);
    dbg!("{:?}", y.1);
}

fn new_person(name: &str, age: u8) -> Person {
    Person {
        name: String::from(name.to_string()),
        age: age + 1,
    }
}
