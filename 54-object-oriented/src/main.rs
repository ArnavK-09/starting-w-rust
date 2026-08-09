// Object-oriented programs are made up of objects.
// An object packages both data and the procedures that operate on that data.
// The procedures are typically called methods or operations.

// Rust is object oriented
// - Structs and enums have data, and impl blocks provide methods on structs and enums.
// - Even though structs and enums with methods aren’t called objects, they provide the same functionality,
// - according to the Gang of Four’s definition of objects.

// Encapsulation That Hides Implementation Details
//
// - encapsulation, which means that the implementation details of an object aren’t accessible to code using that object.
// - the only way to interact with an object is through its public AP
// - code using the object shouldn’t be able to reach into the object’s internals and change data or behavior directly

// The struct is marked pub so that other code can use it, but the fields within the struct remain private
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// The public methods add, remove, and average are the only ways to access or modify data in an instance of AveragedCollection
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_avg();
    }
    pub fn update_avg(&mut self) {
        let total = selt.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(v) => {
                self.update_avg();
                Some(v)
            }
            None => None,
        }
    }
}

// Inheritance as a Type System and as Code Sharing
//
// Inheritance is a mechanism whereby an object can inherit elements from another object’s definition,
// thus gaining the parent object’s data and behavior without you having to define them again
//
// Rust is not such a language. There is no way to define a struct that inherits the parent struct’s fields and method implementations without using a macro.

// To enable a child type to be used in the same places as the parent type.
// This is also called polymorphism
// which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.

// Rust has no struct inheritance, but uses traits for code sharing (default impl)
// Polymorphism in Rust = generics + trait bounds (not inheritance)
// Better: flexible composition, no forced method exposure, no single-inheritance limits
