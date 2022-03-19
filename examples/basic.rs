
use inertia::prelude::*;

fn main() {
    //let rr = RationalField::init();
    //let rx = PolyRing::<RationalField>::init("x");
    //let r = rx.new(1);

    let r1 = Integer::from(1);
    let m1 = Integer::from(2);
    let r2 = Integer::from(3);
    let m2 = Integer::from(5);
    println!("{}", crt(&r1, m1, r2, m2));
}
