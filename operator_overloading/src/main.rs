// OPERATOR OVERLOADING

#![allow(unused_mut)]
#![allow(unused_variables)]

use std::ops::{Add, AddAssign, Neg};
use stf::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> {re, im}
    }
}

// impl Add for Complex<i32> {
//     type Output = Complex<i32>;
// 
//     // a+b
//     fn add(self, rhs: Self /* Complex <i32> */) -> Self::Output {
//         // unimplemented!()
//           Complex {
//              re: self.re = rhs.re,
//              im: self.im = rhs.im
//           }
//     }
// }

impl <T> Add for Complex<T> where T: Add<Output = T> {
    type Output = Complex<T>;

    fn add(self, rhs: Self /* Complex <i32> */) -> Self::Output {
        Complex {
            re: self.re = rhs.re,
            im: self.im = rhs.im
        }
     }
}

impl <T> AddAssign for Complex<T> where T: AddAssign<T> {
    fn add_assign (&mut self, rhs: Self) {
        self.re = rhs.re;
        self.im = rhs.im;
    }
}

impl <T> Neg for Complex<T> where T: Neg<Output=T> {
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: - self.re,
            im: -self.im
        }
    }
}

impl <T> PartialEq for Complex<T> where T: PartialEq {
    fn eq (&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

// partual equality
// full equality: x = x
// NaN - not a number eg 0/0 or inf/inf
// NaN == NaN -> false; violated by full equality

fn main () {
    let mut a = Complex::new(1, 2);
    let mut b = Complex::new(3, 4);

    // println!("{:?}", a+b);
    
    a += b;
    println!("{:?}", -a);

    println!("{:?}", a==a);
}