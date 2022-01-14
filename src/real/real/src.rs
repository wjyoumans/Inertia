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
use std::ffi::CStr;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::sync::{Arc, RwLock};

use arb_sys::arb::arb_struct;
use libc::c_long;

use crate::*;


#[derive(Debug)]
pub struct RealCtx(pub RwLock<c_long>);

impl Hash for RealCtx {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.read().unwrap().hash(state)
    }
}

/// The field of real numbers with initial precision given by `prec`.
#[derive(Debug, Hash, Clone)]
pub struct RealField {
    pub prec: Arc<RealCtx>,
}

impl Parent for RealField {
    type Element = Real;
    type Context = Arc<RealCtx>;

    #[inline]
    fn default(&self) -> Real {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_sys::arb::arb_init(z.as_mut_ptr());
            Real { 
                data: RealData {
                    prec: Arc::clone(&self.prec), 
                    elem: z.assume_init() 
                }
            }
        }

    }
}

impl Additive for RealField {
    #[inline]
    fn zero(&self) -> Real {
        self.default()
    }
}

impl Multiplicative for RealField {
    #[inline]
    fn one(&self) -> Real {
        let mut res = self.default();
        unsafe { arb_sys::arb::arb_one(res.as_mut_ptr()); }
        res
    }
}

impl AdditiveGroup for RealField {}

impl MultiplicativeGroup for RealField {}

impl Ring for RealField {}

impl Field for RealField {
    type BaseField = RealField;

    #[inline]
    fn base_field(&self) -> RealField {
        RealField { prec: Arc::clone(&self.prec) }
    }
}

impl<T> InitParent1<T> for RealField where
    T: TryInto<c_long>
{
    fn init(prec: T) -> Self {
        match prec.try_into() {
            Ok(v) => RealField { prec: Arc::new(RealCtx(RwLock::new(v))) },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

impl_new_unsafe! {
    RealField, u64 {u64 u32 u16 u8}
    arb_sys::arb::arb_set_ui
}

impl_new_unsafe! {
    RealField, i64 {i64 i32 i16 i8}
    arb_sys::arb::arb_set_si
}

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

impl RealField {
    /// Return the default working precision of the real field.
    pub fn precision(&self) -> c_long {
        *self.prec.0.read().unwrap()
    }
    
    /// Update the default working precision of the real field. This affects all elements of the
    /// particular field.
    pub fn set_precision<T>(&self, prec: T) where 
        T: TryInto<c_long>
    {
        match prec.try_into() {
            Ok(v) => *self.prec.0.write().unwrap() = v,
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

/// A real number represented as a ball over the real numbers, that is, an interval `[m +/- r] = 
/// [m - r, m + r]` where the midpoint `m` and the radius `r` are (extended) real numbers and `r` is 
/// nonnegative (possibly infinite).
pub type Real = Elem<RealField>;

pub struct RealData {
    pub elem: arb_struct,
    pub prec: Arc<RealCtx>,
}

impl Drop for RealData {
    fn drop(&mut self) {
        unsafe { 
            arb_sys::arb::arb_clear(&mut self.elem);
        }
    }
}

impl fmt::Debug for RealData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let s = arb_sys::arb::arb_get_str(
                &self.elem, 
                ARB_DEFAULT_NUM_DIGITS, 
                ARB_DEFAULT_PRINT_MODE
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => {
                    f.debug_struct("RealData")
                        .field("elem", &s.to_owned())
                        .field("prec", &self.prec)
                        .finish()
                },
                Err(_) => panic!("Arb returned invalid UTF-8!")
            }
        }
    }
}

impl Element for Real {
    type Data = RealData;
    type Parent = RealField;

    #[inline]
    fn parent(&self) -> RealField {
        RealField { prec: Arc::clone(&self.data.prec) }
    }
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
        &self.data.elem
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with Arb via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut arb_struct {
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
    }
}
