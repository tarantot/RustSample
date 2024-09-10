// STRUCTS

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures()
{
    let p1 = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {}).", p1.x, p1.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let _myline = Line { start: p1, end: p2};
}

fn main()
{
    structures();
}