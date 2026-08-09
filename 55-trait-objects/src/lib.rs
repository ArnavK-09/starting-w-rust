// Defining a Trait for Common Behavior

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // This vector is of type Box<dyn Draw>, which is a trait object;
    // it’s a stand-in for any type inside a Box that implements the Draw trait.
    pub components: Vec<Box<dyn Draw>>,
    // dyn Draw is Rust's way of saying " a dynamic type that implements the Draw trait ."
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// Implementing the Trait
impl Draw for Button {
    fn draw(&self) {
        println!("from Button")
    }
}
