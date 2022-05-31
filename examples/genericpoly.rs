use inertia::*;

fn main() {
    // Ring of integers Z
    let zz = IntegerRing {};
    println!("zz = {}", &zz);

    // Ring of polynomials over Z, Z[x]
    let zx = PolyRing::init(&zz, "x");
    println!("zx = {}", &zx);

    // Ring of polynomials over polynomials over Z, Z[x][y]
    let zxy = PolyRing::init(&zx, "y");
    println!("zxy = {}", &zxy);

    // Construct integer polynomials
    let zero = PolynomialRing::default(&zx);
    assert_eq!(&zero, &zx.new(0));
    assert_eq!(&zero, zx.new(0));
    assert_eq!(zx.new(0), &zero);

    let f1 = zx.new(-1);

    let f2 = zx.new(Integer::from(10));
    
    let f3 = zx.new(vec![1, 0, 0, 0, 0, -1]);

    let v = vec![1, 1];
    let f4 = zx.new(&v[..]);

    // doesnt work?
    //let f5 = zx.new(&[1, 2]);

    // Construct polynomial in Z[x][y]
    let mut f = zxy.new(&[f1, f2, f3, f4]);
    println!("f = {}", &f);
    
    f.set_coeff(7, &zx.new(vec![0, 0, 0, 1]));
    println!("f = {}", &f);

    f.set_coeff(0, &zero);
    println!("f = {}", &f);

    assert_eq!(&f, &f.clone());
    assert_eq!(&f, f.clone());
    assert_eq!(f.clone(), &f.clone());
    assert_eq!(f, f.clone());
    /*
    f.set_coeff(7, &p1.new(&[0, 0, 0, 1]));
    println!("f = {}", &f);

    let qq = RationalField {};
    let p1 = PolyRing::init(&qq, "x");
    let p2 = PolyRing::init(&p1, "y");
    let f = p2.new(&[
        p1.new(&[1, 2, 3, 4, 5]),
        p1.new(&[-1, 0, 1]),
        p1.new(&[qq.new(1, 2)]),
    ]);
    println!("f = {}", &f);

    let g = p2.new(&[p1.new(2)]);
    println!("g = {}", &g);

    //let h = f*g;
    //println!("{}", h);
    */
}
