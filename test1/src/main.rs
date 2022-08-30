use nannou::prelude::*;

#[derive(Debug)]
struct Wrapper {
    n: i32,
}

fn main() {
    let mut s = Wrapper { n: 10_i32 };
    s.n = clamp(s.n, -2, 2);
    println!("{:?}", s);
}
