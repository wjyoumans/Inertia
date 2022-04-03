use inertia::*;

fn main() {
    let zz = IntegerRing::init();
    let zp = polynomial_ring!(zz);
    println!("{}", zp);
    let p = zp.new(1);
    println!("{}", p);
    
    let zz = RationalField::init();
    let zp = polynomial_ring!(zz);
    println!("{}", zp);
    let mut p = zp.new(0);
    println!("{}", p);
    p.set_coeff(2, 1);
    println!("{:#?}", p);
    p.set_coeff(5, 14);
    println!("{:#?}", p);
}
