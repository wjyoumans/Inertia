
use inertia::prelude::*;

fn main() {
    let rr = RationalField::init();
    let rx = PolyRing::new(rr, "x");
    let r = rx.new(1);
}
