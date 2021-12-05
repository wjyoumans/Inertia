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


use std::mem::MaybeUninit;
use std::sync::Arc;

use flint_sys::fmpz_mod_poly::fmpz_mod_poly_struct;
use flint_sys::fmpz_mod::fmpz_mod_ctx_struct;

use crate::*;


/// The ring of polynomials with coefficients integers mod `n` for any integer `n`.
pub struct IntModPolRing {
    ctx: <Self as Parent>::Data,
}

impl Parent for IntModPolRing {
    type Data = Arc<FmpzModCtx>;
    type Extra = ();
    type Element = IntModPol;
}

impl Additive for IntModPolRing {
    #[inline]
    fn zero(&self) -> IntModPol {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_zero(z.as_mut_ptr(), self.as_ptr()); 
            IntModPol { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl Multiplicative for IntModPolRing {
    #[inline]
    fn one(&self) -> IntModPol {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_one(z.as_mut_ptr(), self.as_ptr()); 
            IntModPol { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }
}

impl AdditiveGroup for IntModPolRing {}

impl MultiplicativeGroup for IntModPolRing {}

impl Ring for IntModPolRing {}

impl PolynomialRing<IntModRing> for IntModPolRing {}

impl IntModPolRing {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz_mod_ctx_struct {
        &self.ctx.0
    }

    /// Construct the ring of polynomials with coefficients integers mod `n`.
    pub fn init(n: &Integer) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod::fmpz_mod_ctx_init(z.as_mut_ptr(), n.as_ptr());
            IntModPolRing { ctx: Arc::new(FmpzModCtx(z.assume_init())) }
        }
    }

    /// Create a new polynomial over integers mod `n`.
    pub fn new<T: Into<IntModPol>>(&self, x: T) -> IntModPol {
        x.into()
    }
}

/// An element of the ring of integers mod `n`.
pub type IntModPol = Elem<IntModPolRing>;

impl Element for IntModPol {
    type Data = fmpz_mod_poly_struct;
    type Parent = IntModPolRing;
}

impl AdditiveElement for IntModPol {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe {
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_is_zero(self.as_ptr(), self.ctx_as_ptr()) == 1
        }
    }
}

impl MultiplicativeElement for IntModPol {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { 
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_is_one(self.as_ptr(), self.ctx_as_ptr()) == 1 
        }
    }
}

impl AdditiveGroupElement for IntModPol {}

impl MultiplicativeGroupElement for IntModPol {}

impl RingElement for IntModPol {}

impl PolynomialRingElement<IntModRing> for IntModPol {}

impl IntModPol {
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
    
    /// Return a [String] representation of a polynomial over integers mod `n`.
    #[inline]
    pub fn get_str(&self) -> String {
        IntPol::from(self).get_str()
    }
    
    /// Return a pretty-printed [String] representation of a finite field element.
    #[inline]
    pub fn get_str_pretty(&self) -> String {
        IntPol::from(self).get_str_pretty("x")
    }
}
