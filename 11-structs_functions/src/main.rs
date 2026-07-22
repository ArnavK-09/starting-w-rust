#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// multiple funcs in signle impl of single func single impl
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn is_rectangle(&self) -> bool {
        true
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // no duplicate funcs |  fn is_rectangle(&self) -> bool {
    fn is_square(&self) -> bool {
        false
    }
}

fn main() {
    let r1 = Rectangle {
        width: 300,
        height: 550,
    };

    dbg!(
        r1.area(),
        r1.is_rectangle(),
        r1.is_square(),
        Rectangle::square(10)
    );
}
