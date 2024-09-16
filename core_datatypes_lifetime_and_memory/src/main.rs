// CORE DATATYPES, LIFETIME, AND MEMORY //

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

use std::mem; // standart package for memory data


// fn main()
// {
//     println!("test");
//     let x = 0;
// }

fn main()
{
    // boolean variable
    let g: bool = !false; // true
    println!("{}, size = {} bytes", g, mem::size_of_val(&g))
}


// OWNERSHIP
// fn main()
// { 
//    let u = 1; // i32 datatype
//    let u2 = u;    
//    println!("u = {}", u); // success

//    let v = vec![1, 2, 3];
//    let v2 = v;
    
//    println!("{:?}", v); /* fail because 
//    once the variable reassigned it becomes unusable */

//    let foo - |v:Vec<i32>| ();
//    foo(v);

//    let u = Box::new(1); // i32
//    let u2 = u;

//    println("u = {}", *u); /* fail because 
//    using hte moved value */

//    /* retake ownership and return the value back */
//    let print_vector = |x:Vec<i32>| -> Vec<i32> {
//        println!("{:?}", x);
//        x
//    };
// }


// BORROWING
fn main ()
{
    let print_vector = |x:&Vec<i32>| {
        println!("x[0] = {}", x[0]);
    };

    let v = vec![3, 2, 1];
    print_vector(&v);
    println!("v[0] = {}", v[0]); // access vector after borrowing

    let mut a = 40;
    let b = &mut a;
    *b += 2; /* borrowing the value of 'a'
    the * allows to acces what is referred to */

    println!("a = {}", a); // error because 'a' is not released

    let mut a1 = 40;
    {
        let b1 = &mut a1;
    *b += 2;
    } 

    println!("a = {}", a); // correct because b1 borrows only within {}

    let mut z = vec![3, 2, 1];
    for i in &z {
        println!("i = {}", i);
        z.push(5); // error
    }
}


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


// LIFETIME
struct Person {
    name: String
}

impl Person {
    fn get_ref_name<'a> (&'aself) -> &'a String {
        &self.name
    }
}

struct Company<'z> { // specifying the lifetyme
    name: String,
    ceo: &'z Person // the same lifetime as for Company
}

fn main () {
    /* &'static  definition of how long the vatiable will live */

    // let boss = Person{name: String::from("Elon Musk")};
    // let tesla = Company{name:String::from("Tesla"), ceo: &boss};

    let mut z: &String; {
        let p = Person{name: String::from("John")}; /* the variabe
        does not live long enough because of lifetime elision */
        z = p.get_ref_name();
    }
}
