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


use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::sync::Arc;

use flint_sys::fmpz_mod_poly::fmpz_mod_poly_struct;
use flint_sys::fmpz_mod::fmpz_mod_ctx_struct;

use crate::*;


/// The ring of polynomials with coefficients integers mod `n` for any integer `n`.
pub type IntModPolyRing = PolyRing<IntModRing>;

impl Parent for IntModPolyRing {
    type Element = IntModPoly;
    type Context = Arc<FmpzModCtx>;

    #[inline]
    fn default(&self) -> IntModPoly {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_init(z.as_mut_ptr(), self.as_ptr()); 
            IntModPoly { 
                data: z.assume_init(),
                ctx: Arc::clone(&self.ctx), 
                x: Arc::clone(&self.var), 
            }
        }
    }
}

impl Additive for IntModPolyRing {
    #[inline]
    fn zero(&self) -> IntModPoly {
        self.default()
    }
}

impl Multiplicative for IntModPolyRing {
    #[inline]
    fn one(&self) -> IntModPoly {
        let mut res = self.default();
        unsafe { flint_sys::fmpz_mod_poly::fmpz_mod_poly_one(res.as_mut_ptr(), self.as_ptr()); }
        res 
    }
}

impl AdditiveGroup for IntModPolyRing {}

impl Ring for IntModPolyRing {}

impl PolynomialRing for IntModPolyRing {
    type BaseRing = IntModRing;

    #[inline]
    fn base_ring(&self) -> IntModRing {
        IntModRing { ctx: Arc::clone(&self.ctx) }
    }

    #[inline]
    fn gens(&self) -> Vec<IntModPoly> {
        vec!(self.new(vec![0,1].as_slice()))
    }
}

impl InitParent2<&Integer, &str> for IntModPolyRing {
    #[inline]
    fn init(n: &Integer, x: &str) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod::fmpz_mod_ctx_init(z.as_mut_ptr(), n.as_ptr());
            IntModPolyRing { 
                phantom: PhantomData::<IntModRing>,
                ctx: Arc::new(FmpzModCtx(z.assume_init())), 
                var: Arc::new(x.to_owned()) 
            }
        }
    }
}

impl<T> InitParent2<T, &str> for IntModPolyRing where 
    T: Into<Integer>
{
    #[inline]
    fn init(n: T, x: &str) -> Self {
        Self::init(&n.into(), x)
    }
}

#[inline]
unsafe fn fmpz_mod_poly_set_si(
    f: *mut fmpz_mod_poly_struct,
    x: c_long,
    ctx: *const fmpz_mod_ctx_struct) 
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_si(z.as_mut_ptr(), x);
    flint_sys::fmpz_mod_poly::fmpz_mod_poly_set_fmpz(f, z.as_ptr(), ctx);
    flint_sys::fmpz::fmpz_clear(z.as_mut_ptr());
}

impl_new_unsafe! {
    ctx
    IntModPolyRing, u64 {u64 u32 u16 u8}
    flint_sys::fmpz_mod_poly::fmpz_mod_poly_set_ui
}

impl_new_unsafe! {
    ctx
    IntModPolyRing, i64 {i64 i32 i16 i8}
    fmpz_mod_poly_set_si
}

impl_new_unsafe! {
    ctx
    IntModPolyRing, Integer
    flint_sys::fmpz_mod_poly::fmpz_mod_poly_set_fmpz
}

impl_new_unsafe! {
    ctx
    IntModPolyRing, IntMod
    flint_sys::fmpz_mod_poly::fmpz_mod_poly_set_fmpz
}

impl_new_unsafe! {
    ctx
    IntModPolyRing, IntPoly
    flint_sys::fmpz_mod_poly::fmpz_mod_poly_set_fmpz_poly
}

impl_new_unsafe! {
    ctx
    IntModPolyRing, IntModPoly
    flint_sys::fmpz_mod_poly::fmpz_mod_poly_set
}

impl_new_unsafe! {
    pol
    IntModPolyRing, {u64 u32 u16 u8 i64 i32 i16 i8 Integer IntMod}
}


impl IntModPolyRing {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz_mod_ctx_struct {
        &self.ctx.0
    }

    /// Return the modulus `n` of the integers mod `n`.
    pub fn modulus(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { 
            let n = flint_sys::fmpz_mod::fmpz_mod_ctx_modulus(self.as_ptr()); 
            flint_sys::fmpz::fmpz_set(res.as_mut_ptr(), n);
        }
        res
    }
}

/// An element of the ring of integers mod `n`.
#[derive(Debug)]
pub struct IntModPoly {
    pub data: fmpz_mod_poly_struct,
    pub ctx: Arc<FmpzModCtx>,
    pub x: Arc<String>,
}

impl Drop for IntModPoly {
    fn drop(&mut self) {
        unsafe { 
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_clear(&mut self.data, &self.ctx.0);
        }
    }
}

impl Element for IntModPoly {
    type Parent = IntModPolyRing;

    #[inline]
    fn parent(&self) -> IntModPolyRing {
        IntModPolyRing { 
            phantom: PhantomData::<IntModRing>,
            ctx: Arc::clone(&self.ctx), 
            var: Arc::clone(&self.x) 
        }
    }
}

impl AdditiveElement for IntModPoly {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe {
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_is_zero(self.as_ptr(), self.ctx_as_ptr()) == 1
        }
    }
}

impl MultiplicativeElement for IntModPoly {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { 
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_is_one(self.as_ptr(), self.ctx_as_ptr()) == 1 
        }
    }
}

impl AdditiveGroupElement for IntModPoly {}

impl RingElement for IntModPoly {}

impl PolynomialRingElement for IntModPoly {
    type BaseRingElement = IntMod;

    /// Return the length of the polynomial, equivalently, the degree plus one.
    #[inline]
    fn len(&self) -> c_long {
        unsafe { flint_sys::fmpz_mod_poly::fmpz_mod_poly_length(self.as_ptr(), self.ctx_as_ptr())}
    }
    
    /// Return the degree of the polynomial.
    #[inline]
    fn degree(&self) -> c_long {
        unsafe { flint_sys::fmpz_mod_poly::fmpz_mod_poly_degree(self.as_ptr(), self.ctx_as_ptr())}
    }
   
    fn var(&self) -> String {
        (*self.x).clone()
    }

    #[inline]
    fn get_coeff(&self, i: usize) -> IntMod {
        let mut res = self.parent().base_ring().default();
        unsafe {
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_get_coeff_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                i as i64,
                self.ctx_as_ptr()
            );
            res
        }
    }
    
    #[inline]
    fn set_coeff(&mut self, i: usize, coeff: &IntMod) {
        unsafe {
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_set_coeff_fmpz(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.as_ptr(),
                self.ctx_as_ptr()
            );
        }
    }

    /// Return a pretty-printed [String] representation of a finite field element.
    #[inline]
    fn get_str_pretty(&self) -> String {
        IntPoly::from(self).get_str_pretty()
    }
}

impl IntModPoly {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz_mod_poly_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz_mod_poly_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &fmpz_mod_ctx_struct {
        &self.ctx.0
    }
   
    /// Return the modulus `n` of the integers mod `n`.
    pub fn modulus(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { 
            let n = flint_sys::fmpz_mod::fmpz_mod_ctx_modulus(self.ctx_as_ptr()); 
            flint_sys::fmpz::fmpz_set(res.as_mut_ptr(), n);
        }
        res
    }

    /// Return a [String] representation of a polynomial over integers mod `n`.
    #[inline]
    pub fn get_str(&self) -> String {
        IntPoly::from(self).get_str()
    }    
}
