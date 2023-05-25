use inertia::*;

fn main() {
    let a = Integer::new(123);
    println!("a = {}", a);

    let zz = IntegerRing::init();
    let b = zz.new(2);
    println!("b = {}", b);

    let c = a * b;
    println!("a * b = {}", c);

    let zx = PolyRing::init(&zz, "x");
    println!("{}", zx);

    let f = zx.new([1, 0, 1]);
    println!("{}", &f);
    //println!("{}", &f*f.clone());
    //println!("{}", c * f);

    let g = c*&f;
    println!("{}", g);
}
