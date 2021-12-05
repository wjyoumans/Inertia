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
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::sync::{Arc, RwLock};

use arb_sys::arb::arb_struct;
use libc::{c_int, c_long, c_ulong};
use num_traits::{Zero, PrimInt};

use crate::*;

/// The field of real numbers with initial precision given by `ctx`.
pub struct RealField {
    pub ctx: <Self as Parent>::Data,
}

impl Parent for RealField {
    type Data = Arc<RwLock<c_long>>;
    type Extra = ();
    type Element = Real;
}

impl Additive for RealField {
    #[inline]
    fn zero(&self) -> Real {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_sys::arb::arb_init(z.as_mut_ptr());
            arb_sys::arb::arb_zero(z.as_mut_ptr());
            Real { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl Multiplicative for RealField {
    #[inline]
    fn one(&self) -> Real {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_sys::arb::arb_init(z.as_mut_ptr());
            arb_sys::arb::arb_one(z.as_mut_ptr());
            Real { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl AdditiveGroup for RealField {}

impl MultiplicativeGroup for RealField {}

impl Ring for RealField {}

impl Field for RealField {}

impl<T> Init1<T> for RealField where
    T: TryInto<c_long>
{
    fn init(prec: T) -> Self {
        match prec.try_into() {
            Ok(v) => RealField { ctx: Arc::new(RwLock::new(v)) },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

macro_rules! impl_new {
    (
        $cast:ident {$($t:ident)*};
        $func:path
    ) => ($(
        impl New<$t> for RealField {
            #[inline]
            fn new(&self, x: $t) -> Real {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    arb_sys::arb::arb_init(z.as_mut_ptr());
                    $func(
                        z.as_mut_ptr(), 
                        x as $cast,
                    );
                    Real { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }
    )*);
    (
        $t:ident
        $func:path
    ) => (
        impl New<&$t> for RealField {
            #[inline]
            fn new(&self, x: &$t) -> Real {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    arb_sys::arb::arb_init(z.as_mut_ptr());
                    $func(
                        z.as_mut_ptr(), 
                        x.as_ptr(),
                    );
                    Real { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }

        impl New<$t> for RealField {
            #[inline]
            fn new(&self, x: $t) -> Real {
                self.new(&x)
            }
        }
    );
}

impl_new! {
    u64 {u64 u32 u16 u8};
    arb_sys::arb::arb_set_ui
}

impl_new! {
    i64 {i64 i32 i16 i8};
    arb_sys::arb::arb_set_si
}

impl_new! {
    Integer
    arb_sys::arb::arb_set_fmpz
}

impl New<&Rational> for RealField {
    #[inline]
    fn new(&self, x: &Rational) -> Real {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_sys::arb::arb_init(z.as_mut_ptr());
            arb_sys::arb::arb_set_fmpq(z.as_mut_ptr(), x.as_ptr(), self.precision());
            Real { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl New<Rational> for RealField {
    #[inline]
    fn new(&self, x: Rational) -> Real {
        self.new(&x)
    }
}

impl RealField {
    /// Return the default working precision of the real field.
    pub fn precision(&self) -> c_long {
        *self.ctx.read().unwrap()
    }
    
    /// Update the default working precision of the real field. This affects all elements of the
    /// particular field.
    pub fn set_precision<T>(&self, prec: T) where 
        T: TryInto<c_long>
    {
        match prec.try_into() {
            Ok(v) => *self.ctx.write().unwrap() = v,
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

/// A real number represented as a ball over the real numbers, that is, an interval `[m +/- r] = 
/// [m - r, m + r]` where the midpoint `m` and the radius `r` are (extended) real numbers and `r` is 
/// nonnegative (possibly infinite).
pub type Real = Elem<RealField>;

impl Element for Real {
    type Data = arb_struct;
    type Parent = RealField;
}

impl AdditiveElement for Real {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { arb_sys::arb::arb_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for Real {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { arb_sys::arb::arb_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for Real {}

impl MultiplicativeGroupElement for Real {}

impl RingElement for Real {}

impl FieldElement for Real {}

impl Real {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// Arb via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &arb_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with Arb via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut arb_struct {
        &mut self.data
    }
    
    /// Return the default working precision of the real field.
    pub fn precision(&self) -> c_long {
        *self.ctx.read().unwrap()
    }
    
    /// Update the default working precision of the real field. This affects all elements of the
    /// particular field.
    pub fn set_precision<T>(&self, prec: T) where 
        T: TryInto<c_long>
    {
        match prec.try_into() {
            Ok(v) => *self.ctx.write().unwrap() = v,
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
    
    /// Return a [String] representation of the real number.
    #[inline]
    pub fn get_str(&self, n: c_long) -> String {
        unsafe {
            let s = arb_sys::arb::arb_get_str(self.as_ptr(), n, 0);
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Arb returned invalid UTF-8!")
            }
        }
    }
}
