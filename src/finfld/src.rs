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
use std::sync::Arc;

use flint_sys::fq_default::fq_default_struct as fq_struct;
use flint_sys::fq_default::fq_default_ctx_struct as fq_ctx_struct;
use libc::c_long;
use num_traits::PrimInt;

use crate::*;


#[derive(Debug)]
pub struct FqCtx(pub fq_ctx_struct);

impl Drop for FqCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::fq_default::fq_default_ctx_clear(&mut self.0); }
    }
}

/// The finite field with `p^k` elements for `p` prime.
pub struct FiniteField {
    pub ctx: <Self as Parent>::Data,
}

impl Parent for FiniteField {
    type Data = Arc<FqCtx>;
    type Extra = ();
    type Element = FinFldElem;
}

impl Additive for FiniteField {
    #[inline]
    fn zero(&self) -> FinFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default::fq_default_init(z.as_mut_ptr(), self.as_ptr());
            flint_sys::fq_default::fq_default_zero(z.as_mut_ptr(), self.as_ptr());
            FinFldElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl Multiplicative for FiniteField {
    #[inline]
    fn one(&self) -> FinFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default::fq_default_init(z.as_mut_ptr(), self.as_ptr());
            flint_sys::fq_default::fq_default_one(z.as_mut_ptr(), self.as_ptr());
            FinFldElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl AdditiveGroup for FiniteField {}

impl MultiplicativeGroup for FiniteField {}

impl Ring for FiniteField {}

impl Field for FiniteField {}

impl<T> Init3<&Integer, T, &str> for FiniteField where
    T: TryInto<c_long>
{
    /// Construct the finite field with `p^k` elements.
    fn init(p: &Integer, k: T, var: &str) -> Self {
        match k.try_into() {
            Ok(k) => {
                assert!(p.is_prime());
                assert!(k > 0);
            
                let tmp = CString::new(var).unwrap();
                let mut z = MaybeUninit::uninit();
                unsafe {
                    flint_sys::fq_default::fq_default_ctx_init(
                        z.as_mut_ptr(), 
                        p.as_ptr(), 
                        k, 
                        tmp.as_ptr()
                    );
                    FiniteField { ctx: Arc::new(FqCtx(z.assume_init())) }
                }
            },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

impl<T, U> Init3<T, U, &str> for FiniteField where
    T: PrimInt + Into<Integer>,
    U: TryInto<c_long>
{
    /// Construct the finite field with `p^k` elements.
    #[inline]
    fn init(p: T, k: U, var: &str) -> Self {
        Self::init(&p.into(), k, var)
    }
}

macro_rules! impl_new {
    (
        $cast:ident {$($t:ident)*};
        $func:path
    ) => ($(
        impl New<$t> for FiniteField {
            #[inline]
            fn new(&self, x: $t) -> FinFldElem {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    flint_sys::fq_default::fq_default_init(z.as_mut_ptr(), self.as_ptr());
                    $func(
                        z.as_mut_ptr(),
                        x as $cast,
                        self.as_ptr()
                    );
                    FinFldElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }
    )*);
    (
        $t:ident
        $func:path
    ) => (
        impl New<&$t> for FiniteField {
            #[inline]
            fn new(&self, x: &$t) -> FinFldElem {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    flint_sys::fq_default::fq_default_init(z.as_mut_ptr(), self.as_ptr());
                    $func(
                        z.as_mut_ptr(),
                        x.as_ptr(),
                        self.as_ptr()
                    );
                    FinFldElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }
        
        impl New<$t> for FiniteField {
            #[inline]
            fn new(&self, x: $t) -> FinFldElem {
                self.new(&x)
            }
        }
    );
}

impl_new! {
    u64 {u64 u32 u16 u8};
    flint_sys::fq_default::fq_default_set_ui
}

impl_new! {
    i64 {i64 i32 i16 i8};
    flint_sys::fq_default::fq_default_set_si
}

impl_new! {
    Integer
    flint_sys::fq_default::fq_default_set_fmpz
}

impl_new! {
    IntPol
    flint_sys::fq_default::fq_default_set_fmpz_poly
}

impl_new! {
    IntModPol
    flint_sys::fq_default::fq_default_set_fmpz_mod_poly
}

impl FiniteField {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fq_ctx_struct {
        &self.ctx.0
    }
}

/// An element of a finite field.
pub type FinFldElem = Elem<FiniteField>;

impl Element for FinFldElem {
    type Data = fq_struct;
    type Parent = FiniteField;

    #[inline]
    fn parent(&self) -> FiniteField {
        FiniteField { ctx: Arc::clone(&self.ctx) }
    }
}

impl AdditiveElement for FinFldElem {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { 
            flint_sys::fq_default::fq_default_is_zero(self.as_ptr(), self.ctx_as_ptr()) == 1 
        }
    }
}

impl MultiplicativeElement for FinFldElem {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { 
            flint_sys::fq_default::fq_default_is_one(self.as_ptr(), self.ctx_as_ptr()) == 1 
        }
    }
}

impl AdditiveGroupElement for FinFldElem {}

impl MultiplicativeGroupElement for FinFldElem {}

impl RingElement for FinFldElem {}

impl FieldElement for FinFldElem {}

impl FinFldElem {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fq_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fq_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &fq_ctx_struct {
        &self.ctx.0
    }
    
    /// Return a [String] representation of a finite field element.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::fq_default::fq_default_get_str(self.as_ptr(), self.ctx_as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return a pretty-printed [String] representation of a finite field element.
    #[inline]
    pub fn get_str_pretty(&self) -> String {
        unsafe {
            let s = flint_sys::fq_default::fq_default_get_str_pretty(
                self.as_ptr(), 
                self.ctx_as_ptr()
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
}
