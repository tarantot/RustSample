// CORE DATATYPES //

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

use std::mem; // standart package for memory data


// fn main()
// {
//     println!("test");
//     let x = 0;
// }


// fn main()
// {
//     let a: u8 = 123; // u - unsigned, 8 bits, 0 -- 255
//     println!("a = {}", a); // imutable variable which value cannot be reassigned
// 
//     let mut b: i8 = 0; // i = signed, range -128 -- 127 (including 0) from -2^(n-1) ... 2^(n-1)-1
//     println!("b = {} before", b);
//     b = 42;
//     println!("b = {} after", b);
// 
//     let mut c = 123456789; // i32 = 32 bits = 4 bytes; mutable value that can be reassigned
//     println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c)); // function mem::size_of_val() to see the size of variable
// 
//     c = -1;
//     println!("c = {}", c);
// 
//     // u8, u16, u32, u64, i8, i16, ...
// }


// fn main()
// {
//     // usize, isize
// 
//     let z: isize = 123;
//     let size_of_z:usize = mem::size_of_val(&z);
//     println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8)
// }


// fn main()
// {
//     let d: char = 'x'; // definig a single character
//     println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));
// 
//     // f32 f64 - floating point variable; IEE754
// 
//     let e: f32 = 2.5;
//     println!("{}, size = {} bytes", e, mem::size_of_val(&e));
// 
//     let e1: f64 = 2.5;
//     println!("{}, size = {} bytes", e1, mem::size_of_val(&e1));
// }

fn main()
{
    // boolean variable
    let g: bool = !false; // true
    println!("{}, size = {} bytes", g, mem::size_of_val(&g))
}