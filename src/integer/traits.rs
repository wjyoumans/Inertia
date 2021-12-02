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


use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;

use flint_sys::fmpz::fmpz;
use rug::ops::Pow;
use rustc_hash::FxHashMap;

use crate::traits::*;
use crate::product::src::Product;
use crate::integer::src::{Integer, IntegerRing};
use crate::rational::src::Rational;


// IntegerRing //

impl Parent for IntegerRing {
    type Data = ();
    type Element = Integer;
}

// Integer //

impl Element for Integer {
    type Data = fmpz;
    type Parent = IntegerRing;
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            flint_sys::fmpz::fmpz_init_set(z.as_mut_ptr(), &self.data); 
            Integer { ctx: (), data: z.assume_init() }
        }
    }
}

impl Default for Integer {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz::fmpz_init(z.as_mut_ptr());
            Integer { ctx: (), data: z.assume_init() }
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for Integer {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz::fmpz_clear(self.as_mut_ptr());}
    }
}

impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ui_vector().hash(state);
    }
}

impl Factorizable for Integer {
    type Output = Product<Integer>;
    fn factor(&self) -> Self::Output {
        assert!(self != &0);
        if self == &1 {
            return Product::from(Integer::from(1))
        };
       
        let mut fac = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_factor::fmpz_factor_init(fac.as_mut_ptr());
            let mut fac = fac.assume_init();
            
            flint_sys::fmpz_factor::fmpz_factor(&mut fac, self.as_ptr());

            let n = fac.num as usize;
            let base = std::slice::from_raw_parts(fac.p, n);
            let exp = std::slice::from_raw_parts(fac.exp, n);
            
            let mut hashmap = FxHashMap::<Integer, Integer>::default();
            for (p, k) in base.iter().zip(exp) {
                hashmap.insert(Integer { ctx: (), data: p.clone() }, Integer::from(k));
            }
            
            flint_sys::fmpz_factor::fmpz_factor_clear(&mut fac);
            let fac = Product::<Integer>::from(hashmap);
            fac
        }
    }
}

impl EvaluateProduct for Product<Integer> {
    type Output = Rational;
    fn evaluate(&self) -> Rational {
        let mut x = Rational::from(1);
        for (p, k) in self.hashmap.iter() {
            x *= p.pow(k);
        }
        x
    }
}

impl EvaluateProductMod<Integer> for Product<Integer> {
    type Output = Result<Integer, ()>;
    #[inline]
    fn evaluate_mod(&self, modulus: Integer) -> Result<Integer, ()> {
        self.evaluate_mod(&modulus)
    }
}

impl EvaluateProductMod<&Integer> for Product<Integer> {
    type Output = Result<Integer, ()>;
    fn evaluate_mod(&self, modulus: &Integer) -> Result<Integer, ()> {
        let mut x = Integer::from(1);
        for (p, k) in self.hashmap.iter() {
            x *= p.powm(k, modulus)?;
            x %= modulus;
        }
        Ok(x)
    }
}
