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
use std::fmt;
use std::mem::MaybeUninit;
use std::sync::Arc;


use flint_sys::fq_default_poly::fq_default_poly_struct as fq_poly_struct;
use flint_sys::fq_default::fq_default_ctx_struct as fq_ctx_struct;
use libc::{c_long, c_ulong};

use crate::*;

/// The ring of polynomials over the finite field with `p^k` elements.
#[derive(Debug, Clone)]
pub struct FinFldPolRing {
    ctx: Arc<FqCtx>,
    x: Arc<String>,
}

impl Parent for FinFldPolRing {
    type Element = FinFldPol;

    #[inline]
    fn default(&self) -> FinFldPol {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default_poly::fq_default_poly_init(z.as_mut_ptr(), self.as_ptr());
            FinFldPol {
                data: FinFldPolData {
                    ctx: Arc::clone(&self.ctx),
                    x: Arc::clone(&self.x),
                    elem: z.assume_init()
                }
            }
        }
    }
}

impl Additive for FinFldPolRing {
    #[inline]
    fn zero(&self) -> FinFldPol {
        self.default()
    }
}

impl Multiplicative for FinFldPolRing {
    #[inline]
    fn one(&self) -> FinFldPol {
        let mut res = self.default();
        unsafe { flint_sys::fq_default_poly::fq_default_poly_one(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
}

impl AdditiveGroup for FinFldPolRing {}

impl Ring for FinFldPolRing {}

impl PolynomialRing for FinFldPolRing {
    type BaseRing = FiniteField;

    #[inline]
    fn base_ring(&self) -> FiniteField {
        FiniteField { ctx: Arc::clone(&self.ctx) }
    }

    #[inline]
    fn gens(&self) -> Vec<FinFldPol> {
        vec!(self.new(vec![0,1].as_slice()))
    }
}

impl<T> Init4<&Integer, T, &str, &str> for FinFldPolRing where 
    T: TryInto<c_long>,
{
    /// Construct the ring of polynomials over the finite field with `p^k` elements.
    #[inline]
    fn init(p: &Integer, k: T, var: &str, x: &str) -> Self { 
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
                    FinFldPolRing { ctx: Arc::new(FqCtx(z.assume_init())), x: Arc::new(x.to_owned()) }
                }
            },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

impl<T, U> Init4<T, U, &str, &str> for FinFldPolRing where 
    T: Into<Integer>,
    U: TryInto<c_long>,
{
    /// Construct the ring of polynomials over the finite field with `p^k` elements.
    #[inline]
    fn init(p: T, k: U, var: &str, x: &str) -> Self {
        Self::init(&p.into(), k, var, x)
    }
}

#[inline]
unsafe fn fq_default_poly_set_ui(
    f: *mut fq_poly_struct,
    x: c_ulong,
    ctx: *const fq_ctx_struct) 
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(z.as_mut_ptr(), x);
    flint_sys::fq_default_poly::fq_default_poly_set_coeff_fmpz(f, 0, z.as_ptr(), ctx);
    flint_sys::fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
unsafe fn fq_default_poly_set_si(
    f: *mut fq_poly_struct,
    x: c_long,
    ctx: *const fq_ctx_struct) 
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_si(z.as_mut_ptr(), x);
    flint_sys::fq_default_poly::fq_default_poly_set_coeff_fmpz(f, 0, z.as_ptr(), ctx);
    flint_sys::fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
unsafe fn fq_default_poly_set_fmpz(
    f: *mut fq_poly_struct,
    x: *const flint_sys::fmpz::fmpz,
    ctx: *const fq_ctx_struct) 
{
    flint_sys::fq_default_poly::fq_default_poly_set_coeff_fmpz(f, 0, x, ctx);
}

impl_new_unsafe! {
    ctx
    FinFldPolRing, u64 {u64 u32 u16 u8}
    fq_default_poly_set_ui
}

impl_new_unsafe! {
    ctx
    FinFldPolRing, i64 {i64 i32 i16 i8}
    fq_default_poly_set_si
}

impl_new_unsafe! {
    ctx
    FinFldPolRing, Integer
    fq_default_poly_set_fmpz
}

impl_new_unsafe! {
    ctx
    FinFldPolRing, IntMod
    fq_default_poly_set_fmpz
}

impl_new_unsafe! {
    ctx
    FinFldPolRing, IntPol
    flint_sys::fq_default_poly::fq_default_poly_set_fmpz_poly
}

impl_new_unsafe! {
    ctx
    FinFldPolRing, IntModPol
    flint_sys::fq_default_poly::fq_default_poly_set_fmpz_mod_poly
}

impl_new_unsafe! {
    ctx
    FinFldPolRing, FinFldElem
    flint_sys::fq_default_poly::fq_default_poly_set_fq_default
}

impl_new_unsafe! {
    ctx
    FinFldPolRing, FinFldPol
    flint_sys::fq_default_poly::fq_default_poly_set
}

impl_new_unsafe! {
    pol
    FinFldPolRing, {u64 u32 u16 u8 i64 i32 i16 i8 Integer IntMod IntPol IntModPol FinFldElem}
}


impl FinFldPolRing {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fq_ctx_struct {
        &self.ctx.0
    }
}

/// An element of a finite field.
pub type FinFldPol = Elem<FinFldPolRing>;

pub struct FinFldPolData {
    pub elem: fq_poly_struct,
    pub ctx: Arc<FqCtx>,
    pub x: Arc<String>,
}

impl Drop for FinFldPolData {
    fn drop(&mut self) {
        unsafe { 
            flint_sys::fq_default_poly::fq_default_poly_clear(&mut self.elem, &self.ctx.0);
        }
    }
}

impl fmt::Debug for FinFldPolData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = CString::new((*self.x).clone()).unwrap();
        unsafe {
            let s = flint_sys::fq_default_poly::fq_default_poly_get_str_pretty(
                &self.elem, 
                x.as_ptr(),
                &self.ctx.0
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => {
                    f.debug_struct("FinFldPolData")
                        .field("elem", &s.to_owned())
                        .field("ctx", &self.ctx)
                        .finish()
                },
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
}

impl Element for FinFldPol {
    type Data = FinFldPolData;
    type Parent = FinFldPolRing;

    #[inline]
    fn parent(&self) -> FinFldPolRing {
        FinFldPolRing { ctx: Arc::clone(&self.data.ctx), x: Arc::clone(&self.data.x) }
    }
}

impl AdditiveElement for FinFldPol {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { 
            flint_sys::fq_default_poly::fq_default_poly_is_zero(
                self.as_ptr(), 
                self.ctx_as_ptr()
            ) == 1 
        }
    }
}

impl MultiplicativeElement for FinFldPol {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { 
            flint_sys::fq_default_poly::fq_default_poly_is_one(
                self.as_ptr(), 
                self.ctx_as_ptr()
            ) == 1 
        }
    }
}

impl AdditiveGroupElement for FinFldPol {}

impl RingElement for FinFldPol {}

impl PolynomialRingElement for FinFldPol {
    type BaseRingElement = FinFldElem;

    /// Return the length of the polynomial, equivalently, the degree plus one.
    #[inline]
    fn len(&self) -> c_long {
        unsafe { flint_sys::fq_default_poly::fq_default_poly_length(self.as_ptr(), self.ctx_as_ptr())}
    }
    
    /// Return the degree of the polynomial.
    #[inline]
    fn degree(&self) -> c_long {
        unsafe { flint_sys::fq_default_poly::fq_default_poly_degree(self.as_ptr(), self.ctx_as_ptr())}
    }
    
    #[inline]
    fn get_coeff(&self, i: usize) -> FinFldElem {
        let mut res = self.parent().base_ring().default();
        unsafe {
            flint_sys::fq_default_poly::fq_default_poly_get_coeff(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                i as i64,
                self.ctx_as_ptr()
            );
            res
        }
    }
    
    #[inline]
    fn set_coeff(&mut self, i: usize, coeff: &FinFldElem) {
        unsafe {
            flint_sys::fq_default_poly::fq_default_poly_set_coeff(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.as_ptr(),
                self.ctx_as_ptr()
            );
        }
    }
}

impl FinFldPol {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fq_poly_struct {
        &self.data.elem
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fq_poly_struct {
        &mut self.data.elem
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &fq_ctx_struct {
        &self.data.ctx.0
    }
    
    /// Return a [String] representation of a polynomial over a finite field.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::fq_default_poly::fq_default_poly_get_str(
                self.as_ptr(),
                self.ctx_as_ptr()
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return a pretty-printed [String] representation of a polynomial over a finite field.
    #[inline]
    pub fn get_str_pretty(&self) -> String {
        let x = CString::new((*self.data.x).clone()).unwrap();
        unsafe {
            let s = flint_sys::fq_default_poly::fq_default_poly_get_str_pretty(
                self.as_ptr(), 
                x.as_ptr(),
                self.ctx_as_ptr()
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return the length of the polynomial, equivalently, the degree plus one.
    pub fn len(&self) -> c_long {
        unsafe {
            flint_sys::fq_default_poly::fq_default_poly_length(
                self.as_ptr(),
                self.ctx_as_ptr()
            )
        }
    }

    /// Get the i-th coefficient of a polynomial over a finite field.
    #[inline]
    pub fn get_coeff(&self, i: usize) -> FinFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fq_default::fq_default_init(z.as_mut_ptr(), self.ctx_as_ptr());
            flint_sys::fq_default_poly::fq_default_poly_get_coeff(
                z.as_mut_ptr(), 
                self.as_ptr(), 
                i as i64,
                self.ctx_as_ptr()
            );
            FinFldElem { 
                data: FinFldElemData {
                    ctx: Arc::clone(&self.data.ctx), 
                    elem: z.assume_init() 
                }
            }
        }
    }
    
    /// Set the i-th coefficient of a polynomial over a finite field.
    #[inline]
    pub fn set_coeff(&mut self, i: usize, coeff: &FinFldElem) {
        unsafe {
            flint_sys::fq_default_poly::fq_default_poly_set_coeff(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.as_ptr(),
                self.ctx_as_ptr()
            );
        }
    }
    
    // NOTE: there is also fq_default_poly_get/set_coeff_fmpz

    /// Return an [FinFldElem] vector containing the coefficients of the polynomial.
    #[inline]
    pub fn coefficients(&self) -> Vec<FinFldElem> {
        let len = self.len();

        let mut vec = Vec::<FinFldElem>::default();
        for i in 0..len {
            vec.push(self.get_coeff(i as usize));
        }
        vec
    }
}
