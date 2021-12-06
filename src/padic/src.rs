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


use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::sync::Arc;

use flint_sys::padic::padic_ctx_struct;
use flint_sys::padic::padic_struct;
use libc::c_long;
use num_traits::PrimInt;

use crate::*;


pub struct PadicCtx(padic_ctx_struct);

impl Drop for PadicCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::padic::padic_ctx_clear(&mut self.0); }
    }
}

/// The p-adic completion of the rational numbers.
pub struct PadicField {
    pub ctx: <Self as Parent>::Data,
}

impl Parent for PadicField {
    type Data = Arc<PadicCtx>;
    type Extra = ();
    type Element = PadicElem;
}

impl Additive for PadicField {
    #[inline]
    fn zero(&self) -> PadicElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::padic::padic_init(z.as_mut_ptr());
            flint_sys::padic::padic_zero(z.as_mut_ptr());
            PadicElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl Multiplicative for PadicField {
    #[inline]
    fn one(&self) -> PadicElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::padic::padic_init(z.as_mut_ptr());
            flint_sys::padic::padic_one(z.as_mut_ptr());
            PadicElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl AdditiveGroup for PadicField {}

impl MultiplicativeGroup for PadicField {}

impl Ring for PadicField {}

impl Field for PadicField {}

impl<T> Init2<&Integer, T> for PadicField where
    T: TryInto<c_long>
{
    fn init(p: &Integer, k: T) -> Self {
        match k.try_into() {
            Ok(kk) => {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    flint_sys::padic::padic_ctx_init(
                        z.as_mut_ptr(), 
                        p.as_ptr(), 
                        0, 
                        kk, 
                        PADIC_DEFAULT_PRINT_MODE
                    );
                    PadicField { ctx: Arc::new(PadicCtx(z.assume_init())) }
                }
            },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

impl<T, U> Init2<T, U> for PadicField where
    T: Into<Integer>,
    U: TryInto<c_long>
{
    fn init(p: T, k: U) -> Self {
        Self::init(&p.into(), k)
    }
}

macro_rules! impl_new {
    (
        $cast:ident {$($t:ident)*};
        $func:path
    ) => ($(
        impl New<$t> for PadicField {
            #[inline]
            fn new(&self, x: $t) -> PadicElem {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    flint_sys::padic::padic_init(z.as_mut_ptr());
                    $func(
                        z.as_mut_ptr(), 
                        x as $cast,
                        self.as_ptr(),
                    );
                    PadicElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }
    )*);
    (
        $t:ident
        $func:path
    ) => (
        impl New<&$t> for PadicField {
            #[inline]
            fn new(&self, x: &$t) -> PadicElem {
                let mut z = MaybeUninit::uninit();
                unsafe {
                    flint_sys::padic::padic_init(z.as_mut_ptr());
                    $func(
                        z.as_mut_ptr(), 
                        x.as_ptr(),
                        self.as_ptr(),
                    );
                    PadicElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
                }        
            }
        }

        impl New<$t> for PadicField {
            #[inline]
            fn new(&self, x: $t) -> PadicElem {
                self.new(&x)
            }
        }
    );
}

impl_new! {
    u64 {u64 u32 u16 u8};
    flint_sys::padic::padic_set_ui
}

impl_new! {
    i64 {i64 i32 i16 i8};
    flint_sys::padic::padic_set_si
}

impl_new! {
    Integer
    flint_sys::padic::padic_set_fmpz
}

impl_new! {
    IntMod
    flint_sys::padic::padic_set_fmpz
}

impl_new! {
    Rational
    flint_sys::padic::padic_set_fmpq
}

impl PadicField {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &padic_ctx_struct {
        &self.ctx.0
    }
}

/// An element of a p-adic field.
pub type PadicElem = Elem<PadicField>;

impl Element for PadicElem {
    type Data = padic_struct;
    type Parent = PadicField;
}

impl AdditiveElement for PadicElem {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::padic::padic_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for PadicElem {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::padic::padic_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for PadicElem {}

impl MultiplicativeGroupElement for PadicElem {}

impl RingElement for PadicElem {}

impl FieldElement for PadicElem {}

impl PadicElem {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &padic_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut padic_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &padic_ctx_struct {
        &self.ctx.0
    }
}
