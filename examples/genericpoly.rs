use inertia::*;

fn main() {
    let zz = IntegerRing {};
    let p1 = PolyRing::init(&zz, "x");
    let p2 = PolyRing::init(&p1, "y");
    println!("{}", p2);

    let z = PolynomialRing::default(&p1);
    let mut p = p2.new(
        &[
            p1.new(&[1, 0, 1]), 
            p1.new(&[2, 2]), 
            z, 
            p1.new(&[3, 1, 2]), 
            p1.new(&[5])
        ]
    );
    println!("{}", &p);
    
    p.set_coeff(7, &p1.new(&[0,0,0,1]));
    println!("{}", &p);
    
    let qq = RationalField {};
    let p1 = PolyRing::init(&qq, "x");
    let p2 = PolyRing::init(&p1, "y");
    let p = p2.new(
        &[
            p1.new(&[1, 2, 3, 4, 5]), 
            p1.new(&[-1, 0, 1]), 
            p1.new(&[qq.new([1, 2])])
        ]
    );
    println!("{}", p);
}
