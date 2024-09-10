// SCOPE AND SHADOWING; DESCLARING AND USING CONSTRAINTS //

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

use std::mem; // standart package for memory data

// fn scope_and_shadowing()
// {
//     let a = 123;
// 
//     {
//         let b = 456;
//         println!("inside, b = {}", b);
//         
//         let a = 777;
//         println!("inside, a = {}", a);
//     }
// 
//     let b = 007;
//     
//     println!("outside, \"a\" is equal to = {}", a);
//     println!("outside, b = {}", b); // will return error
// }

//fn main()
//{
    // scope_and_shadowing();
//}



const MEANING_OF_LIFE:u8 = 42; // no fixed address

static mut Z:i32 = 123;

fn main()
{
    unsafe // five actions in unsafe Rust that you can’t in safe Rust
    {
        Z = 777;
        println!("{}", Z);
    }
    println!("{}", MEANING_OF_LIFE);
}