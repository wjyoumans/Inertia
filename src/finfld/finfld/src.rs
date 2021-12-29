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
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::sync::Arc;

use flint_sys::fq_default::fq_default_struct as fq_struct;
use flint_sys::fq_default::fq_default_ctx_struct as fq_ctx_struct;
use libc::c_long;

use crate::*;


/// The finite field with `p^k` elements for `p` prime.
#[derive(Debug, Clone, Hash)]
pub struct FiniteField {
    pub ctx: Arc<FqCtx>,
}

#[derive(Debug)]
pub struct FqCtx(pub fq_ctx_struct);

impl Drop for FqCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::fq_default::fq_default_ctx_clear(&mut self.0); }
    }
}

impl Hash for FqCtx {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut res = Integer::default();
        unsafe { 
            flint_sys::fq_default::fq_default_ctx_prime(res.as_mut_ptr(), &self.0);
            res.hash(state);

            let zp = IntModPolyRing::init(res, "x");
            let mut res = zp.default();
            flint_sys::fq_default::fq_default_ctx_modulus(res.as_mut_ptr(), &self.0);
            res.hash(state)
        }
    }
}

impl Parent for FiniteField {
    type Element = FinFldElem;
    type Context = Arc<FqCtx>;

    #[inline]
    fn default(&self) -> FinFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default::fq_default_init(z.as_mut_ptr(), self.as_ptr());
            FinFldElem { 
                data: FinFldElemData { 
                    ctx: Arc::clone(&self.ctx), 
                    elem: z.assume_init() 
                } 
            }
        }
    }
}

impl Additive for FiniteField {
    #[inline]
    fn zero(&self) -> FinFldElem {
        self.default()
    }
}

impl Multiplicative for FiniteField {
    #[inline]
    fn one(&self) -> FinFldElem {
        let mut res = self.default();
        unsafe { flint_sys::fq_default::fq_default_one(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
}

impl AdditiveGroup for FiniteField {}

impl MultiplicativeGroup for FiniteField {}

impl Ring for FiniteField {}

impl Field for FiniteField {
    type BaseField = FiniteField;

    #[inline]
    fn base_field(&self) -> FiniteField {
        FiniteField::init(self.prime(), 1, "o")
    }
}

impl<T> InitParent3<&Integer, T, &str> for FiniteField where
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

impl<T, U> InitParent3<T, U, &str> for FiniteField where
    T: Into<Integer>,
    U: TryInto<c_long>
{
    /// Construct the finite field with `p^k` elements.
    #[inline]
    fn init(p: T, k: U, var: &str) -> Self {
        Self::init(&p.into(), k, var)
    }
}

impl_new_unsafe! {
    ctx
    FiniteField, u64 {u64 u32 u16 u8}
    flint_sys::fq_default::fq_default_set_ui
}

impl_new_unsafe! {
    ctx
    FiniteField, i64 {i64 i32 i16 i8}
    flint_sys::fq_default::fq_default_set_si
}

impl_new_unsafe! {
    ctx
    FiniteField, Integer
    flint_sys::fq_default::fq_default_set_fmpz
}

impl_new_unsafe! {
    ctx
    FiniteField, IntMod
    flint_sys::fq_default::fq_default_set_fmpz
}

impl_new_unsafe! {
    ctx
    FiniteField, IntPoly
    flint_sys::fq_default::fq_default_set_fmpz_poly
}

impl_new_unsafe! {
    ctx
    FiniteField, IntModPoly
    flint_sys::fq_default::fq_default_set_fmpz_mod_poly
}

impl_new_unsafe! {
    ctx
    FiniteField, FinFldElem
    flint_sys::fq_default::fq_default_set
}

impl FiniteField {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fq_ctx_struct {
        &self.ctx.0
    }

    #[inline]
    pub fn modulus(&self) -> IntModPoly {
        let zp = IntModPolyRing::init(self.prime(), "x");
        let mut res = zp.default();
        unsafe { flint_sys::fq_default::fq_default_ctx_modulus(res.as_mut_ptr(), self.as_ptr()); }
        res
    }

    #[inline]
    pub fn prime(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fq_default::fq_default_ctx_prime(res.as_mut_ptr(), self.as_ptr()); }
        res
    }

    #[inline]
    pub fn degree(&self) -> c_long {
        unsafe { flint_sys::fq_default::fq_default_ctx_degree(self.as_ptr()) }
    }

    #[inline]
    pub fn order(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fq_default::fq_default_ctx_order(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
}

/// An element of a finite field.
pub type FinFldElem = Elem<FiniteField>;

#[derive(Debug)]
pub struct FinFldElemData {
    pub elem: fq_struct,
    pub ctx: Arc<FqCtx>,
}

impl Drop for FinFldElemData {
    fn drop(&mut self) {
        unsafe { 
            flint_sys::fq_default::fq_default_clear(&mut self.elem, &self.ctx.0);
        }
    }
}

impl Element for FinFldElem {
    type Data = FinFldElemData;
    type Parent = FiniteField;

    #[inline]
    fn parent(&self) -> FiniteField {
        FiniteField { ctx: Arc::clone(&self.data.ctx) }
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
        &self.data.elem
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fq_struct {
        &mut self.data.elem
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &fq_ctx_struct {
        &self.data.ctx.0
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

    #[inline]
    pub fn modulus(&self) -> IntModPoly {
        let zp = IntModPolyRing::init(self.prime(), "x");
        let mut res = zp.default();
        unsafe { flint_sys::fq_default::fq_default_ctx_modulus(res.as_mut_ptr(), self.ctx_as_ptr()); }
        res
    }

    #[inline]
    pub fn prime(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fq_default::fq_default_ctx_prime(res.as_mut_ptr(), self.ctx_as_ptr()); }
        res
    }

    #[inline]
    pub fn degree(&self) -> c_long {
        unsafe { flint_sys::fq_default::fq_default_ctx_degree(self.ctx_as_ptr()) }
    }

    #[inline]
    pub fn order(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fq_default::fq_default_ctx_order(res.as_mut_ptr(), self.ctx_as_ptr()); }
        res
    }
}
