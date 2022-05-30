use inertia::*;

fn main() {
    let zz = IntegerRing {};
    let p1 = PolyRing::init(&zz, "x");
    let p2 = PolyRing::init(&p1, "y");
    println!("{}", p2);

    let z = PolynomialRing::default(&p1);
    let mut f = p2.new(
        &[
            p1.new(&[1, 0, 1]), 
            p1.new(&[2, 2]), 
            z, 
            p1.new(&[3, 1, 2]), 
            p1.new(&[5])
        ]
    );
    println!("{}", &f);
    
    f.set_coeff(7, &p1.new(&[0,0,0,1]));
    println!("f = {}", &f);
    
    let qq = RationalField {};
    let p1 = PolyRing::init(&qq, "x");
    let p2 = PolyRing::init(&p1, "y");
    let f = p2.new(
        &[
            p1.new(&[1, 2, 3, 4, 5]), 
            p1.new(&[-1, 0, 1]), 
            p1.new(&[qq.new([1, 2])])
        ]
    );
    println!("f = {}", &f);
    
    let g = p2.new(&[p1.new(&[2])]);
    println!("g = {}", &g);

    //let h = f*g;
    //println!("{}", h);
}
