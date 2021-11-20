use std::fmt::Debug;
use crate::integer::src::Integer;
use crate::rational::src::Rational;
use crate::intpol::src::IntPol;


macro_rules! impl_from_prim {
    ($cast:ident $func:path; $($t:ident)*) => ($(
        impl_from! {
            impl From<&$t> for IntPol {
                fn from(src: &$t) -> IntPol {
                    let mut res = IntPol::default();
                    unsafe { $func(res.as_mut_ptr(), *src as $cast); }
                    res
                }
            }
        }

    )*)
}

impl_from_prim! {u64 flint_sys::fmpz_poly::fmpz_poly_set_ui; usize u64 u32 u16 u8 }
impl_from_prim! {i64 flint_sys::fmpz_poly::fmpz_poly_set_si; isize i64 i32 i16 i8 }

impl_from! {
    impl From<&Integer> for IntPol {
        fn from(src: &Integer) -> IntPol {
            let mut res = IntPol::default();
            unsafe {
                flint_sys::fmpz_poly::fmpz_poly_set_fmpz(
                    res.as_mut_ptr(),
                    src.as_ptr()
                    );
            }
            res
        }
    }
}

impl<'a, T: Debug> From<&'a [T]> for IntPol where &'a T: Into<Integer> {
    fn from(src: &'a [T]) -> IntPol {
        let mut res = IntPol::default();
        for (i, x) in src.iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}

impl From<&IntPol> for String {
    fn from(x: &IntPol) -> String {
        /*
        let mut vec: Vec<u8> = Vec::new();
        let mut v = vec.as_mut_ptr();
        v = unsafe { flint_sys::fmpz_poly::fmpz_poly_get_str(x.as_ptr()) as *mut u8 };
        match String::from_utf8(vec) {
            Ok(s) => s,
            Err(_) => panic!("Flint returned invalid UTF-8!")
        }*/
        
        let mut vec = vec![];
        let deg = x.degree();

        if deg >= 1 {
            let c = x.get_coeff(0);

            if c.sign() > 0 {
                vec.push(format!("{}", c));
                vec.push("+".to_string());
            } else if c.sign() < 0 {
                vec.push(format!("{}", c.abs()));
                vec.push("-".to_string());
            }
        }

        if deg >= 2 {
            let c = x.get_coeff(1);
            if c.sign() > 0 {
                if c.is_one() {
                    vec.push("x".to_string());
                } else {
                    vec.push(format!("{}*x", c));
                }
                vec.push("+".to_string());
            } else if c.sign() < 0 {
                if c.abs().is_one() {
                    vec.push("x".to_string());
                } else {
                    vec.push(format!("{}*x", c.abs()));
                }
                vec.push("-".to_string());
            }
        }

        for i in 2..deg-1 {
            let c = x.get_coeff(i as usize);
            if c.sign() > 0 {
                if c.is_one() {
                    vec.push(format!("x^{}", i));
                } else {
                    vec.push(format!("{}*x^{}", c, i));
                }
                vec.push("+".to_string());
            } else if c.sign() < 0 {
                if c.abs().is_one() {
                    vec.push(format!("x^{}", i));
                } else {
                    vec.push(format!("{}*x^{}", c.abs(), i));
                }
                vec.push("-".to_string());
            }
        }

        let c = x.get_coeff(deg as usize);
        if deg == 0 {
            vec.push(format!("{}", c));
        } else if deg == 1 {
            if c.abs().is_one() {
                if c.sign() > 0 {
                    vec.push("x".to_string());
                } else {
                    vec.push("-x".to_string());
                }
            } else {
                vec.push(format!("{}*x", c));
            }
        } else if c.abs().is_one() {
            if c.sign() > 0 {
                vec.push(format!("x^{}", deg));
            } else {
                vec.push(format!("-x^{}", deg));
            }
        } else {
            vec.push(format!("{}*x^{}", c, deg));
        }

        vec.reverse();
        vec.join(" ")
    }
}

impl<'a> From<IntPol> for String {
    fn from(x: IntPol) -> String {
        String::from(&x)
    }
}
