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

use arb_sys::acb::acb_struct;
use arb_sys::arb::arb_struct;
use libc::{c_int, c_long, c_ulong};
use num_traits::{Zero, PrimInt, Signed, Unsigned};

use crate::*;

/// The field of complex numbers with initial precision given by `ctx`.
pub struct ComplexField {
    pub ctx: <Self as Parent>::Data,
}

impl Parent for ComplexField {
    type Data = Arc<RwLock<c_long>>;
    type Extra = ();
    type Element = Complex;

    #[inline]
    fn default(&self) -> Complex {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_sys::acb::acb_init(z.as_mut_ptr());
            Complex { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
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
        ComplexField { ctx: Arc::clone(&self.ctx) }
    }
}

impl<T> Init1<T> for ComplexField where
    T: TryInto<c_long>
{
    fn init(prec: T) -> Self {
        match prec.try_into() {
            Ok(v) => ComplexField { ctx: Arc::new(RwLock::new(v)) },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

macro_rules! impl_new_arr {
    (
        $cast:ident {$($t:ident)*};
        $func:path
    ) => ($(
        impl New<[$t; 2]> for ComplexField {
            #[inline]
            fn new(&self, x: [$t; 2]) -> Complex {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    arb_sys::acb::acb_init(z.as_mut_ptr());
                    $func(
                        z.as_mut_ptr(), 
                        x[0] as $cast,
                        x[1] as $cast
                    );
                    Complex { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }
    )*);
    (
        $t:ident
        $func:path
    ) => (
        impl New<[$t; 2]> for ComplexField {
            #[inline]
            fn new(&self, x: [$t; 2]) -> Complex {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    arb_sys::acb::acb_init(z.as_mut_ptr());
                    $func(
                        z.as_mut_ptr(), 
                        x[0].as_ptr(),
                        x[1].as_ptr()
                    );
                    Complex { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }
    );
    (
        $($t:ident)+;
        $func:path
    ) => ($(
        impl New<[$t; 2]> for ComplexField {
            #[inline]
            fn new(&self, x: [$t; 2]) -> Complex {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    arb_sys::acb::acb_init(z.as_mut_ptr());
                    $func(
                        z.as_mut_ptr(), 
                        Integer::from(x[0]).as_ptr(),
                        Integer::from(x[1]).as_ptr()
                    );
                    Complex { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }
    )+);
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
    Real
    arb_sys::acb::acb_set_arb_arb
}

impl New<[&Rational; 2]> for ComplexField {
    #[inline]
    fn new(&self, x: [&Rational; 2]) -> Complex {
        let rr = RealField::init(self.precision());
        self.new([rr.new(x[0]), rr.new(x[1])])
    }
}

macro_rules! impl_new {
    (
        $cast:ident {$($t:ident)*};
        $func:path
    ) => ($(
        impl New<$t> for ComplexField {
            #[inline]
            fn new(&self, x: $t) -> Complex {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    arb_sys::acb::acb_init(z.as_mut_ptr());
                    $func(
                        z.as_mut_ptr(), 
                        x as $cast,
                    );
                    Complex { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }
    )*);
    (
        $t:ident
        $func:path
    ) => (
        impl New<&$t> for ComplexField {
            #[inline]
            fn new(&self, x: &$t) -> Complex {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    arb_sys::acb::acb_init(z.as_mut_ptr());
                    $func(
                        z.as_mut_ptr(), 
                        x.as_ptr(),
                    );
                    Complex { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }

        impl New<$t> for ComplexField {
            #[inline]
            fn new(&self, x: $t) -> Complex {
                self.new(&x)
            }
        }
    );
}

impl_new! {
    u64 {u64 u32 u16 u8};
    arb_sys::acb::acb_set_ui
}

impl_new! {
    i64 {i64 i32 i16 i8};
    arb_sys::acb::acb_set_si
}

impl_new! {
    f64 {f64};
    arb_sys::acb::acb_set_d
}

impl_new! {
    Integer
    arb_sys::acb::acb_set_fmpz
}

impl_new! {
    Real
    arb_sys::acb::acb_set_arb
}

impl New<&Rational> for ComplexField {
    #[inline]
    fn new(&self, x: &Rational) -> Complex {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_sys::acb::acb_init(z.as_mut_ptr());
            arb_sys::acb::acb_set_fmpq(z.as_mut_ptr(), x.as_ptr(), self.precision());
            Complex { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl New<Rational> for ComplexField {
    #[inline]
    fn new(&self, x: Rational) -> Complex {
        self.new(&x)
    }
}

impl ComplexField {
    /// Return the default working precision of the complex field.
    pub fn precision(&self) -> c_long {
        *self.ctx.read().unwrap()
    }
    
    /// Update the default working precision of the complex field. This affects all elements of the
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

/// A complex number represented as a pair of [Reals][Real], representing real and imaginary parts
/// with separate error bounds.
pub type Complex = Elem<ComplexField>;

impl Element for Complex {
    type Data = acb_struct;
    type Parent = ComplexField;

    #[inline]
    fn parent(&self) -> ComplexField {
        ComplexField { ctx: Arc::clone(&self.ctx) }
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
        *self.ctx.read().unwrap()
    }
    
    /// Update the default working precision of the complex field. This affects all elements of the
    /// particular field.
    pub fn set_precision<T>(&self, prec: T) where 
        T: TryInto<c_long>
    {
        match prec.try_into() {
            Ok(v) => *self.ctx.write().unwrap() = v,
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
    
    /// Return a [String] representation of the complex number.
    #[inline]
    pub fn get_str(&self, n: c_long) -> String {
        unsafe {
            let r = CStr::from_ptr(arb_sys::arb::arb_get_str(self.real_as_ptr(), n, 0)).to_str();
            let i = CStr::from_ptr(arb_sys::arb::arb_get_str(self.imag_as_ptr(), n, 0)).to_str();
            if r.is_ok() && i.is_ok() {
                format!("{} + i*{}", r.unwrap(), i.unwrap())
            } else {
                panic!("Arb returned invalid UTF-8!")
            }
        }
    }
}
