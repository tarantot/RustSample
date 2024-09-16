// USING MUTEX FOR THREAD-SAFE MUTABILITY

#[allow(dead_code)]
#[allow(unused_variables)]

use::std::sync::Arc; // Arc is for atomic reference counting
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<MutexString>> /* mutual exclusion
    threads are mutually excluded from modifying the actual variable 
    until one of them is allowed to do so */
}

impl Person {
    fn new (name: Arc<String>, state: Arc<MutexString>>) -> Person {
        Person{name: name, state: state}
    }

    fn greet(&self) {
        println!("Hi, my name is {}!", self.name);

        self.state.clear();
        self.state.push_str("excited!");

        println!("Hi, my name is {} and I am {}.", self.name, state.as_str());
    }
}

fn mutex_demo () {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());

    t.join().unwrap().
}

// fn arc_demo () {
//     let name = Arc::new("John".to_string()); 
//     let person = Person::new(name.clone());

//     let t = thread::spawn(move || {
//         person.greet();
//     });
//     println!("Name - {}", name);
// }

t.join().unwrap();

fn main () {
//     rc_demo();
    mutex_demo();
}