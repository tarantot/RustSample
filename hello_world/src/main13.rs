// ARRAYS AND SLICES

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

use std::mem; 

fn array () {
    let mut a:[i32;5] = [1,2,3,4,5];

    println!("a has {} elements, irst is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a); // print entire array content

    // println!("doesn\'t match!", if a != [1,2,3,4,5] else {"match!"});

    let b = [1; 10]; // b.len() == 10 1-ones
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    prinltn!("b took up {} bytes", mem::size_of_val(&b));

    let mxt: [[f32;3]; 2] =   // dimensional array
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    println!("{:?}", mxt);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn use_slice(slice: &[i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
}

fn slices () {
    let mut data = [1,2,3,4,5];

    use_slice(&data[1..4]);
}

fn main () {
    array();
    slices();
}