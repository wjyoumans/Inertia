
use inertia::prelude::*;

fn main() {
    let rr = RationalField::init();
    let rx = PolyRing::<RationalField>::init("x");
    let r = rx.new(1);
}
