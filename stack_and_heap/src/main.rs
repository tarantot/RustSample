// STACK AND HEAP

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

// mod sh;
use std::mem; // standart package for memory data

struct Point 
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes {} bytes.", mem::size_of_val(&p1));
    println!("p2 takes {} bytes.", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("{} and {}", p3.x, p3.y);
}

fn main () {
    stack_and_heap();
}