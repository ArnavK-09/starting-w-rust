// Using Trait Objects to Abstract over Shared Behavior

#![allow(unused_imports)]
#![allow(dead_code)]

use trait_objects::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("from SelectBox")
    }
}

fn main() {
    let screen = Screen {
        // A Vec cannot store different sized items together! That's why we need Box .
        components: vec![
            // Box moves data from stack to heap and creates a stable pointer to it.
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            // Box<dyn Draw> is always 8 bytes (pointer size) regardless of what type it contains.
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// Static Dispatch (Generics)
// Compiler knows exact type at compile time, generates separate code for each type
fn serve_static<C: Draw>(component: C) {
    component.draw();
}

// Dynamic Dispatch (Trait Objects)
// Compiler doesn't know exact type, uses vtable lookup at runtime
fn serve_dynamic(component: &dyn Draw) {
    component.draw();
}
