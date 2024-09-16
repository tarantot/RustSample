// TRAITS

use std::mem; 
use stf::fmt::Debug;

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name (&self) -> &'static str;
    
    fn talk(&self) {
        println!("{} cannot talk.", self.name());
    }
}

struct Human {
    name: &'static str; // the character ' shows the lifetime
}

#[derive(Debug)]
struct Circle {
    radius:f64,
}

#[derive(Debug)]
struct Square {
    side:f64,
}

impl Animal for Human {
    fn create(name:&'static str) -> Human {
        Human{name: name}
    }

    fn name (&self) -> &'static str {
        self.name
    }
    fn talk (&self) {
        println!("{} says hello!", self.name());
    }
}

impl Animal for Cat {
    fn create(name:&'static str) -> Cat {
        Cat{name: name}
    }

    fn name (&self) -> &'static str {
        self.name
    }
    fn talk (&self) {
        println!("{} says hello!", self.name());
    }
}

trait Summable<T> {
    fn sun (&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self {result += *x;}
        return result;
    }
}

fn traits () {
    // let h = Human{name: "John"};
    // let h = Human::create("John");
    let h:Human = Animal::create("John");
    h.talk();

    let c = Cat{name: "Garfield"};
    c.talk();

    let a = vec![1,2,3];
    println!("sum of 'a' is {}", a.sum());
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Sircle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// fn print_info(shape: impl Shape + Debug) { // variations
// fn print_info<T: Shape + Debug>(shape:T) { 
fn print_info<T>(shape: T) where T: Shape + Debug {
    println!("{:&}", shape);
    println!("The area is {}.", shape.area());
}

fn main () {
    traits();
    let c = Circle{radius: 2.0};
    print_info(c);
}