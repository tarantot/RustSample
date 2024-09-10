// CLOSURES

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

mod sh;
mod pm;
use std::mem;

fn say_hello () {println!("hello!");}

fn closures () {
    let sh = say_hello; // storing function into a variable
    sh(); // calling a function

    let plus_one = |x:i32| -> i32 {x + 1}; // closure 
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let plus_two = |x| {
        let mut z = x;
        z += 2;
    };
    println!("{} + 2 = {}", 3, plus_two(3));

    let borrow_two = &mut two;

    // T: by value
    // T&
    // &mut &

    let plus_three = |x:&mut i 32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}

fn main () {
    closures();
}