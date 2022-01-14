use std::mem::MaybeUninit;
use flint_sys::fmpz::*;
use flint_sys::fmpq::*;
use flint_sys::fmpz_poly::*;
use flint_sys::fmpz_poly_q::*;

use inertia::prelude::*;

fn main() {
    /*
    let rr = RatFuncField::init("x");
    let mut x = rr.default();

    unsafe {
        flint_sys::fmpz_poly::fmpz_poly_set(&mut x.data.num, IntPol::from(2).as_ptr());
        //fmpz_poly_q_set_fmpz(x.as_mut_ptr(), Integer::from(2).as_ptr());
        println!("{}", &x);
    }
    */

    /*
    unsafe {
        let mut num = MaybeUninit::uninit();
        let mut den = MaybeUninit::uninit();
        flint_sys::fmpz_poly::fmpz_poly_init(num.as_mut_ptr());
        flint_sys::fmpz_poly::fmpz_poly_init(den.as_mut_ptr());

        flint_sys::fmpz_poly::fmpz_poly_set_si(num.as_mut_ptr(), 2);
        flint_sys::fmpz_poly::fmpz_poly_set_si(den.as_mut_ptr(), 1);

        let f = RatFunc {
            ctx: (),
            extra: Arc::new("x".to_owned()),
            data: flint_sys::fmpz_poly_q::fmpz_poly_q_struct {
                num: num.assume_init(),
                den: den.assume_init(),
            }
        };
        println!("{}", &f);

    }*/

    unsafe {
        let mut z = MaybeUninit::uninit();
        fmpq_init(z.as_mut_ptr());

        fmpz_set_si(&mut (*z.as_mut_ptr()).num, 2);
        fmpq_clear(z.as_mut_ptr());
    }

    /*
    unsafe {
        let mut z = MaybeUninit::uninit();
        fmpz_poly_q_init(z.as_mut_ptr());

        fmpz_poly_set_si(&mut (*z.as_mut_ptr()).num as *mut _, 2);
        fmpz_poly_q_clear(z.as_mut_ptr());
    }*/
}

/*
unsafe fn fmpz_poly_q_set_fmpz(
    rop: *mut flint_sys::fmpz_poly_q::fmpz_poly_q_struct,
    op: *const flint_sys::fmpz::fmpz,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_fmpz(&mut (*rop).num, op);
    flint_sys::fmpz_poly::fmpz_poly_one(&mut (*rop).den);
}*/
