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
    x: Arc<String>,
}

impl Parent for IntModPolRing {
    type Data = Arc<FmpzModCtx>;
    type Extra = Arc<String>;
    type Element = IntModPol;

    #[inline]
    fn default(&self) -> IntModPol {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz_mod_poly::fmpz_mod_poly_init(z.as_mut_ptr(), self.as_ptr()); 
            IntModPol { 
                ctx: Arc::clone(&self.ctx), 
                extra: Arc::clone(&self.x), 
                data: z.assume_init() 
            }
        }
    }
}

impl Additive for IntModPolRing {
    #[inline]
    fn zero(&self) -> IntModPol {
        self.default()
    }
}

impl Multiplicative for IntModPolRing {
    #[inline]
    fn one(&self) -> IntModPol {
        let mut res = self.default();
        unsafe { flint_sys::fmpz_mod_poly::fmpz_mod_poly_one(res.as_mut_ptr(), self.as_ptr()); }
        res 
    }
}

impl AdditiveGroup for IntModPolRing {}

impl MultiplicativeGroup for IntModPolRing {}

impl Ring for IntModPolRing {}

impl PolynomialRing for IntModPolRing {
    type BaseRing = IntModRing;

    #[inline]
    fn base_ring(&self) -> IntModRing {
        IntModRing { ctx: Arc::clone(&self.ctx) }
    }
}

impl Init2<&Integer, &str> for IntModPolRing {
    #[inline]
    fn init(n: &Integer, x: &str) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mod::fmpz_mod_ctx_init(z.as_mut_ptr(), n.as_ptr());
            IntModPolRing { ctx: Arc::new(FmpzModCtx(z.assume_init())), x: Arc::new(x.to_owned()) }
        }
    }
}

impl<T> Init2<T, &str> for IntModPolRing where 
    T: Into<Integer>
{
    #[inline]
    fn init(n: T, x: &str) -> Self {
        Self::init(&n.into(), x)
    }
}

macro_rules! impl_new {
    (
        $cast:ident {$($t:ident)*};
        $func:path
    ) => ($(
        impl New<$t> for IntModPolRing {
            #[inline]
            fn new(&self, x: $t) -> IntModPol {
                let mut res = self.default();
                unsafe { $func(res.as_mut_ptr(), x as $cast, self.as_ptr()); }
                res
            }
        }
    )*);
    (
        $t:ident
        $func:path
    ) => (
        impl New<&$t> for IntModPolRing {
            #[inline]
            fn new(&self, x: &$t) -> IntModPol {
                let mut res = self.default();
                unsafe { $func(res.as_mut_ptr(), x.as_ptr(), self.as_ptr()); }
                res
            }
        }
        
        impl New<$t> for IntModPolRing {
            #[inline]
            fn new(&self, x: $t) -> IntModPol {
                self.new(&x)
            }
        }
    );
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

impl_new! {
    u64 {u64 u32 u16 u8};
    flint_sys::fmpz_mod_poly::fmpz_mod_poly_set_ui
}

impl_new! {
    i64 {i64 i32 i16 i8};
    fmpz_mod_poly_set_si
}

impl_new! {
    Integer
    flint_sys::fmpz_mod_poly::fmpz_mod_poly_set_fmpz
}

impl_new! {
    IntPol
    flint_sys::fmpz_mod_poly::fmpz_mod_poly_set_fmpz_poly
}

impl_new! {
    IntModPol
    flint_sys::fmpz_mod_poly::fmpz_mod_poly_set
}

impl IntModPolRing {
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
pub type IntModPol = Elem<IntModPolRing>;

impl Element for IntModPol {
    type Data = fmpz_mod_poly_struct;
    type Parent = IntModPolRing;

    #[inline]
    fn parent(&self) -> IntModPolRing {
        IntModPolRing { ctx: Arc::clone(&self.ctx), x: Arc::clone(&self.extra) }
    }
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

impl PolynomialRingElement for IntModPol {}

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
        IntPol::from(self).get_str()
    }
    
    /// Return a pretty-printed [String] representation of a finite field element.
    #[inline]
    pub fn get_str_pretty(&self) -> String {
        IntPol::from(self).get_str_pretty()
    }
}
