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
use std::hash::{Hash, Hasher};
use std::fmt;
use std::mem::MaybeUninit;
use std::sync::{Arc, RwLock};

use arb_sys::acb::acb_struct;
use arb_sys::arb::arb_struct;
use libc::c_long;

use crate::*;

#[derive(Debug)]
pub struct ComplexCtx(pub RwLock<c_long>);

impl Hash for ComplexCtx {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.read().unwrap().hash(state)
    }
}

/// The field of complex numbers with initial precision given by `ctx`.
#[derive(Debug, Hash, Clone)]
pub struct ComplexField {
    pub prec: Arc<ComplexCtx>,
}

impl Parent for ComplexField {
    type Element = Complex;
    type Context = Arc<ComplexCtx>;

    #[inline]
    fn default(&self) -> Complex {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_sys::acb::acb_init(z.as_mut_ptr());
            Complex { 
                data: z.assume_init(),
                prec: Arc::clone(&self.prec), 
            }
        }
    }
}

impl Additive for ComplexField {
    #[inline]
    fn zero(&self) -> Complex {
        self.default()
    }
}

impl Multiplicative for ComplexField {
    #[inline]
    fn one(&self) -> Complex {
        let mut res = self.default();
        unsafe { arb_sys::acb::acb_one(res.as_mut_ptr()); }
        res
    }
}

impl AdditiveGroup for ComplexField {}

impl MultiplicativeGroup for ComplexField {}

impl Ring for ComplexField {}

impl Field for ComplexField {
    type BaseField = ComplexField;

    #[inline]
    fn base_field(&self) -> ComplexField {
        ComplexField { prec: Arc::clone(&self.prec) }
    }
}

impl<T> InitParent1<T> for ComplexField where
    T: TryInto<c_long>
{
    fn init(prec: T) -> Self {
        match prec.try_into() {
            Ok(v) => ComplexField { prec: Arc::new(ComplexCtx(RwLock::new(v))) },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

macro_rules! impl_new_arr {
    (
        $cast:ident {$($t:ident)*};
        $func:path
    ) => ($(
        impl NewElement<[$t; 2]> for ComplexField {
            #[inline]
            fn new(&self, x: [$t; 2]) -> Complex {
                let mut res = self.default();
                unsafe {
                    $func(
                        res.as_mut_ptr(), 
                        x[0] as $cast,
                        x[1] as $cast
                    );
                }
                res
            }
        }
    )*);
    (
        $($t:ident)+;
        $func:path
    ) => ($(
        impl NewElement<[$t; 2]> for ComplexField {
            #[inline]
            fn new(&self, x: [$t; 2]) -> Complex {
                let mut res = self.default();
                unsafe {
                    $func(
                        res.as_mut_ptr(), 
                        Integer::from(x[0]).as_ptr(),
                        Integer::from(x[1]).as_ptr()
                    );
                }
                res
            }
        }
    )+);
    (
        $t:ident
        $func:path
    ) => (
        impl NewElement<[&$t; 2]> for ComplexField {
            #[inline]
            fn new(&self, x: [&$t; 2]) -> Complex {
                let mut res = self.default();
                unsafe {
                    $func(
                        res.as_mut_ptr(), 
                        x[0].as_ptr(),
                        x[1].as_ptr()
                    );
                }
                res
            }
        }
        
        impl NewElement<[$t; 2]> for ComplexField {
            #[inline]
            fn new(&self, x: [$t; 2]) -> Complex {
                self.new([&x[0], &x[1]])
            }
        }
    );
}

impl_new_unsafe! {
    ComplexField, u64 {u64 u32 u16 u8}
    arb_sys::acb::acb_set_ui
}

impl_new_unsafe! {
    ComplexField, i64 {i64 i32 i16 i8}
    arb_sys::acb::acb_set_si
}

impl_new_unsafe! {
    ComplexField, f64 {f64}
    arb_sys::acb::acb_set_d
}

impl_new_unsafe! {
    ComplexField, Integer
    arb_sys::acb::acb_set_fmpz
}

impl_new_unsafe! {
    ComplexField, IntMod
    arb_sys::acb::acb_set_fmpz
}

impl_new_unsafe! {
    ComplexField, Real
    arb_sys::acb::acb_set_arb
}

impl_new_unsafe! {
    prec
    ComplexField, Rational
    arb_sys::acb::acb_set_fmpq
}

impl_new_arr! {
    u64 u32 u16 u8;
    arb_sys::acb::acb_set_fmpz_fmpz
}

impl_new_arr! {
    i64 {i64 i32 i16 i8};
    arb_sys::acb::acb_set_si_si
}

impl_new_arr! {
    f64 {f64};
    arb_sys::acb::acb_set_d_d
}

impl_new_arr! {
    Integer
    arb_sys::acb::acb_set_fmpz_fmpz
}

impl_new_arr! {
    IntMod
    arb_sys::acb::acb_set_fmpz_fmpz
}

impl_new_arr! {
    Real
    arb_sys::acb::acb_set_arb_arb
}

impl NewElement<[&Rational; 2]> for ComplexField {
    #[inline]
    fn new(&self, x: [&Rational; 2]) -> Complex {
        let rr = RealField::init(self.precision());
        self.new([ rr.new(x[0]), rr.new(x[1]) ])
    }
}

impl NewElement<[Rational; 2]> for ComplexField {
    #[inline]
    fn new(&self, x: [Rational; 2]) -> Complex {
        self.new([&x[0], &x[1]])
    }
}

impl ComplexField {
    /// Return the default working precision of the complex field.
    pub fn precision(&self) -> c_long {
        *self.prec.0.read().unwrap()
    }
    
    /// Update the default working precision of the complex field. This affects all elements of the
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

/// A complex number represented as a pair of [Reals][Real], representing real and imaginary parts
/// with separate error bounds.
pub struct Complex {
    pub data: acb_struct,
    pub prec: Arc<ComplexCtx>,
}

impl Drop for Complex {
    fn drop(&mut self) {
        unsafe { 
            arb_sys::acb::acb_clear(&mut self.data);
        }
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let r = CStr::from_ptr(
                arb_sys::arb::arb_get_str(
                    &self.data.real, 
                    ARB_DEFAULT_NUM_DIGITS, 
                    ARB_DEFAULT_PRINT_MODE
                    )
                ).to_str();
            let i = CStr::from_ptr(
                arb_sys::arb::arb_get_str(
                    &self.data.imag, 
                    ARB_DEFAULT_NUM_DIGITS, 
                    ARB_DEFAULT_PRINT_MODE
                    )
                ).to_str();
            if r.is_ok() && i.is_ok() {
                f.debug_struct("Complex")
                    .field("data", &format!("{} + i*{}", r.unwrap(), i.unwrap()))
                    .field("prec", &self.prec)
                    .finish()
            } else {
                panic!("Arb returned invalid UTF-8!")
            }
        }
    }
}

impl Element for Complex {
    type Parent = ComplexField;

    #[inline]
    fn parent(&self) -> ComplexField {
        ComplexField { prec: Arc::clone(&self.prec) }
    }
}

impl AdditiveElement for Complex {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { arb_sys::acb::acb_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for Complex {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { arb_sys::acb::acb_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for Complex {}

impl MultiplicativeGroupElement for Complex {}

impl RingElement for Complex {}

impl FieldElement for Complex {}

impl Complex {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// Arb via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &acb_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with Arb via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut acb_struct {
        &mut self.data
    }
    
    /// A reference to the underlying FFI struct of the real part of a complex number. This is only 
    /// needed to interface directly with Arb via the FFI.
    #[inline]
    pub fn real_as_ptr(&self) -> &arb_struct {
        &self.data.real
    }
    
    /// A mutable reference to the underlying FFI struct of the real part of a complex number. This is
    /// only needed to interface directly with Arb via the FFI.
    #[inline]
    pub fn real_as_mut_ptr(&mut self) -> &mut arb_struct {
        &mut self.data.real
    }
    
    /// A reference to the underlying FFI struct of the imaginary part of a complex number. This is 
    /// only needed to interface directly with Arb via the FFI.
    #[inline]
    pub fn imag_as_ptr(&self) -> &arb_struct {
        &self.data.imag
    }
    
    /// A mutable reference to the underlying FFI struct of the imaginary part of a complex number. 
    /// This is only needed to interface directly with Arb via the FFI.
    #[inline]
    pub fn imag_as_mut_ptr(&mut self) -> &mut arb_struct {
        &mut self.data.imag
    }
    
    /// Return the default working precision of the complex field.
    pub fn precision(&self) -> c_long {
        *self.prec.0.read().unwrap()
    }
    
    /// Update the default working precision of the complex field. This affects all elements of the
    /// particular field.
    pub fn set_precision<T>(&self, prec: T) where 
        T: TryInto<c_long>
    {
        match prec.try_into() {
            Ok(v) => *self.prec.0.write().unwrap() = v,
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
    
    /// Return a [String] representation of the complex number.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let r = CStr::from_ptr(
                arb_sys::arb::arb_get_str(
                    self.real_as_ptr(), 
                    ARB_DEFAULT_NUM_DIGITS, 
                    ARB_DEFAULT_PRINT_MODE
                    )
                ).to_str();
            let i = CStr::from_ptr(
                arb_sys::arb::arb_get_str(
                    self.imag_as_ptr(), 
                    ARB_DEFAULT_NUM_DIGITS, 
                    ARB_DEFAULT_PRINT_MODE
                    )
                ).to_str();
            if r.is_ok() && i.is_ok() {
                format!("{} + i*{}", r.unwrap(), i.unwrap())
            } else {
                panic!("Arb returned invalid UTF-8!")
            }
        }
    }
}
