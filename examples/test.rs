use inertia::prelude::*;

fn main() {
    let mut q = rat!([3, 4]);
    let mut r = rat!([1, 2]);
    let n = q.denominator();

    q *= r;
    println!("{}", q.denominator());
    println!("{}", n);
}

