// TUPLES

fn sum_and_product (x:i32, y:i32) -> (i32, i32) {
    (x + y, x * y)
}

fn tuples () {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_rpoduct(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined; 

    let foo = (true, 42.0, -1i16); // tuple with different values
    println!("{:?}", foo);

    let meaning = (42,); // single value tuple
    println!("{:?}", meaning);
}

fn main () {
    tuples();
}