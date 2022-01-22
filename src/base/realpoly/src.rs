/*
 *  Copyright (C) 2021 William Youmans
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */


use std::convert::TryInto;
use std::fmt;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::sync::{Arc, RwLock};

use arb_sys::arb_poly::arb_poly_struct;
use libc::c_long;

use crate::*;


/// The field of real numbers with initial precision given by `prec`.
pub type RealPolyRing = PolyRing<RealField>;

impl Parent for RealPolyRing {
    type Element = RealPoly;
    type Context = Arc<RealCtx>;

    #[inline]
    fn default(&self) -> RealPoly {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_sys::arb_poly::arb_poly_init(z.as_mut_ptr());
            RealPoly { 
                data: RealPolyData {
                    prec: Arc::clone(&self.ctx), 
                    elem: z.assume_init(),
                    x: Arc::clone(&self.var)
                }
            }
        }

    }
}

impl Additive for RealPolyRing {
    #[inline]
    fn zero(&self) -> RealPoly {
        self.default()
    }
}

impl Multiplicative for RealPolyRing {
    #[inline]
    fn one(&self) -> RealPoly {
        let mut res = self.default();
        unsafe { arb_sys::arb_poly::arb_poly_set_si(res.as_mut_ptr(), 1 as c_long); }
        res
    }
}

impl AdditiveGroup for RealPolyRing {}

impl MultiplicativeGroup for RealPolyRing {}

impl Ring for RealPolyRing {}

impl PolynomialRing for RealPolyRing {
    type BaseRing = RealField;

    #[inline]
    fn base_ring(&self) -> RealField {
        RealField { prec: Arc::clone(&self.ctx) }
    }

    #[inline]
    fn gens(&self) -> Vec<RealPoly> {
        vec![self.new(vec![0,1].as_slice())]
    }

}

impl<T> InitParent2<T, &str> for RealPolyRing where
    T: TryInto<c_long>
{
    fn init(prec: T, var: &str) -> Self {
        match prec.try_into() {
            Ok(v) => RealPolyRing { 
                phantom: PhantomData::<RealField>,
                ctx: Arc::new(RealCtx(RwLock::new(v))), 
                var: Arc::new(var.to_owned()) 
            },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

/*
impl_new_unsafe! {
    RealPolyRing, u64 {u64 u32 u16 u8}
    arb_sys::arb_poly::arb_poly_ui
}*/

impl_new_unsafe! {
    RealPolyRing, i64 {i64 i32 i16 i8}
    arb_sys::arb_poly::arb_poly_set_si
}

/*
impl_new_unsafe! {
    RealPolyRing, Real
    arb_sys::arb_poly::arb_poly_set_coeff_arb
}*/

impl_new_unsafe! {
    prec
    RealPolyRing, IntPoly
    arb_sys::arb_poly::arb_poly_set_fmpz_poly
}

impl_new_unsafe! {
    prec
    RealPolyRing, RatPoly
    arb_sys::arb_poly::arb_poly_set_fmpq_poly
}

/*
impl_new_unsafe! {
    RealField, f64 {f64}
    arb_sys::arb::arb_set_d
}

impl_new_unsafe! {
    RealField, Integer
    arb_sys::arb::arb_set_fmpz
}

impl_new_unsafe! {
    RealField, IntMod
    arb_sys::arb::arb_set_fmpz
}

impl_new_unsafe! {
    prec
    RealField, Rational
    arb_sys::arb::arb_set_fmpq
}
*/

/*
impl_new_unsafe! {
    pol
    RealPolyRing, {i64 i32 i16 i8 Integer IntPoly RatPoly}
}*/

impl RealPolyRing {
    /// Return the default working precision of the real field.
    pub fn precision(&self) -> c_long {
        *self.ctx.0.read().unwrap()
    }
    
    /// Update the default working precision of the real field. This affects all elements of the
    /// particular field.
    pub fn set_precision<T>(&self, prec: T) where 
        T: TryInto<c_long>
    {
        match prec.try_into() {
            Ok(v) => *self.ctx.0.write().unwrap() = v,
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

pub type RealPoly = Elem<RealPolyRing>;

pub struct RealPolyData {
    pub elem: arb_poly_struct,
    pub prec: Arc<RealCtx>,
    pub x: Arc<String>,
}

impl Drop for RealPolyData {
    fn drop(&mut self) {
        unsafe { 
            arb_sys::arb_poly::arb_poly_clear(&mut self.elem);
        }
    }
}

impl fmt::Debug for RealPolyData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*
        unsafe {
            let s = arb_sys::arb_poly::arb_poly_get_str(
                &self.elem, 
                ARB_DEFAULT_NUM_DIGITS, 
                ARB_DEFAULT_PRINT_MODE
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => {
                    f.debug_struct("RealData")
                        .field("elem", &s.to_owned())
                        .field("prec", &self.prec)
                        .field("x", &self.x)
                        .finish()
                },
                Err(_) => panic!("Arb returned invalid UTF-8!")
            }
        }*/
        f.debug_struct("RealPolyData")
            .finish()
    }
}

impl Element for RealPoly {
    type Data = RealPolyData;
    type Parent = RealPolyRing;

    #[inline]
    fn parent(&self) -> RealPolyRing {
        RealPolyRing {
            phantom: PhantomData::<RealField>,
            ctx: Arc::clone(&self.data.prec), 
            var: Arc::clone(&self.data.x),
        }
    }
}

impl AdditiveElement for RealPoly {
    #[inline]
    fn is_zero(&self) -> bool {
    }
}

impl MultiplicativeElement for RealPoly {
    #[inline]
    fn is_one(&self) -> bool {
    }
}

impl AdditiveGroupElement for RealPoly {}

impl MultiplicativeGroupElement for RealPoly {}

impl RingElement for RealPoly {}

impl PolynomialRingElement for RealPoly {
    type BaseRingElement = Real;

    #[inline]
    fn len(&self) -> c_long {
        unsafe { arb_sys::arb_poly::arb_poly_length(self.as_ptr()) }
    }
    
    #[inline]
    fn degree(&self) -> c_long {
        unsafe { arb_sys::arb_poly::arb_poly_degree(self.as_ptr()) }
    }

    #[inline]
    fn var(&self) -> String {
        *self.data.x
    }
    
    #[inline]
    fn get_coeff(&self, i: usize) -> Real {
        let mut res = self.base_ring().default();
        unsafe {
            arb_sys::arb_poly::arb_poly_get_coeff_arb(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                i as i64
            );
            res
        }
    }
    
    #[inline]
    fn set_coeff(&mut self, i: usize, coeff: &Real) {
        unsafe {
            arb_sys::arb_poly::arb_poly_set_coeff_arb(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.as_ptr()
            );
        }
    }
   
    #[inline]
    fn get_str_pretty(&self) -> String {
        /*
        let v = CString::new((*self.data.x).clone()).unwrap();
        unsafe {
            let s = arb_sys::arb_poly::arb_poly_get_str_pretty(self.as_ptr(), v.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }*/
        "not implemented".to_owned()
    }
}

impl RealPoly {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// Arb via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &arb_poly_struct {
        &self.data.elem
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with Arb via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut arb_poly_struct {
        &mut self.data.elem
    }
    
    /// Return the default working precision of the real field.
    pub fn precision(&self) -> c_long {
        *self.data.prec.0.read().unwrap()
    }
    
    /// Update the default working precision of the real field. This affects all elements of the
    /// particular field.
    pub fn set_precision<T>(&self, prec: T) where 
        T: TryInto<c_long>
    {
        match prec.try_into() {
            Ok(v) => *self.data.prec.0.write().unwrap() = v,
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
   
    /*
    /// Return a [String] representation of the real number.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = arb_sys::arb::arb_get_str(
                self.as_ptr(), 
                ARB_DEFAULT_NUM_DIGITS, 
                ARB_DEFAULT_PRINT_MODE
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Arb returned invalid UTF-8!")
            }
        }
    }*/
}
