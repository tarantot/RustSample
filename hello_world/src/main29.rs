// REFERENCE-COUNTED VARIABLES

#[allow(dead_code)]
#[allow(unused_variables)]

use::std::rc::Rc; /* for using reference-count variable
keeps a count to the number of locations in the code
where the variable actually being referenced */

struct Person {
    name: Rc<String>
}

impl Person {
    fn new (name: Rc<String>) -> Person {
        Person{name: name}
    }

    fn greet(&self) {
        println!("Hi, my name is {}!", self.name);
    }
}

fn rc_demo () {
    // let name = "John".to_string();
    // let person = Person::new(name); // moving the variable so it becomes unavailable

    // let name = Rc::new("John".to_string()); // taking the variable and sharing it around
    // let person = Person::new(name.clone());
    // person.greet();
    // println!("Name - {}", name);

    let name = Rc::new("John".to_string()); // taking the variable and sharing it around
    println!("Name = {} and has {} strong pointers.", name, Rc::strong_count(&name)); // 1 - just created
    {
        let person = Person::new(name.clone());
        println!("Name = {} and has {} strong pointers.", name, Rc::strong_count(&name)); // 2 - cloned
        person.greet();
    }
    println!("Name = {} and has {} strong pointers.", name, Rc::strong_count(&name)); // again 1
}

fn main () {
    rc_demo();
}