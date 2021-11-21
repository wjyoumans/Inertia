use std::fmt::Debug;
use std::ffi::CString;
use crate::integer::src::Integer;
use crate::rational::src::Rational;


macro_rules! impl_from_prim {
    ($cast:ident $func:path; $($t:ident)*) => ($(
        impl_from! {
            impl From<&$t> for Rational {
                fn from(src: &$t) -> Rational {
                    let mut res = Rational::default();
                    unsafe { $func(res.as_mut_ptr(), *src as $cast, 1); }
                    res
                }
            }
        }

    )*)
}

impl_from_prim! {u64 flint_sys::fmpq::fmpq_set_ui; usize u64 u32 u16 u8 }
impl_from_prim! {i64 flint_sys::fmpq::fmpq_set_si; isize i64 i32 i16 i8 }

impl_from! {
    impl From<&Integer> for Rational {
        fn from(src: &Integer) -> Rational {
            let mut res = Rational::default();
            unsafe { 
                flint_sys::fmpq::fmpq_set_fmpz_frac(
                    res.as_mut_ptr(), 
                    src.as_ptr(), 
                    Integer::from(1).as_ptr()
                ); 
            }
            res
        }
    }
}

impl<'a, T: Debug> From<&'a [T]> for Rational where &'a T: Into<Integer> {
    fn from(src: &'a [T]) -> Rational {
        assert_eq!(2, src.len());
       
        let den: Integer = (&src[1]).into();
        assert!(!den.is_zero());

        let mut res = Rational::default();
        unsafe { 
            flint_sys::fmpq::fmpq_set_fmpz_frac(
                res.as_mut_ptr(), 
                (&src[0]).into().as_ptr(),
                den.as_ptr()
            ); 
        }
        res
    }
}

impl From<&str> for Rational {
    fn from(s :&str) -> Rational {
        let c_str = CString::new(s).expect("String contains 0 byte.");

        let mut z = Rational::default();
        unsafe {
            let res = flint_sys::fmpq::fmpq_set_str(z.as_mut_ptr(), c_str.as_ptr(), 10);
            assert_eq!(res, 0);
            z
        }
    }
}

impl From<Rational> for String {
    fn from(x: Rational) -> String {
        format!("{}/{}", x.numerator().to_str_radix(10), x.denominator().to_str_radix(10))
    }
}

impl From<&Rational> for String {
    fn from(x: &Rational) -> String {
        if x.denominator() == 1 {
            x.numerator().to_str_radix(10)
        } else {
            format!("{}/{}", &x.numerator().to_str_radix(10), &x.denominator().to_str_radix(10))
        }
    }
}

