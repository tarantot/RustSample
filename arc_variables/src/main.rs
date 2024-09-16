// ATOMIC REFERENCE COUNTED (ARC) VARIABLES

// REFERENCE-COUNTED VARIABLES

#[allow(dead_code)]
#[allow(unused_variables)]

use::std::sync::Arc; // Arc is for atomic reference counting
use std::thread;

struct Person {
    name: Arc<String>
}

impl Person {
    fn new (name: Arc<String>) -> Person {
        Person{name: name}
    }

    fn greet(&self) {
        println!("Hi, my name is {}!", self.name);
    }
}

fn arc_demo () {
    let name = Arc::new("John".to_string()); 
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name - {}", name);

    t.join.unwrap();
}

fn main () {
    rc_demo();
}