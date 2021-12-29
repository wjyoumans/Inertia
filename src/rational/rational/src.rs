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

use flint_sys::fmpq::fmpq;

use crate::*;

// RationalField //

/// A rational field that can be used as a [Rational] "factory".
#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct RationalField {}

impl Parent for RationalField {
    type Element = Rational;
    type Context = ();

    #[inline]
    fn default(&self) -> Rational {
        Rational::default()
    }
}

impl Additive for RationalField {
    #[inline]
    fn zero(&self) -> Rational {
        Rational::default()
    }
}

impl Multiplicative for RationalField {
    #[inline]
    fn one(&self) -> Rational {
        let mut res = Rational::default();
        unsafe { flint_sys::fmpq::fmpq_one(res.as_mut_ptr()); }
        res
    }
}

impl AdditiveGroup for RationalField {}

impl MultiplicativeGroup for RationalField {}

impl Ring for RationalField {}

impl Field for RationalField {
    type BaseField = RationalField;
    
    #[inline]
    fn base_field(&self) -> RationalField {
        RationalField {}
    }
}

impl InitParent for RationalField {
    #[inline]
    fn init() -> Self {
        RationalField {}
    }
}

impl NewElement<&Rational> for RationalField {
    #[inline]
    fn new(&self, x: &Rational) -> Rational {
        x.clone()
    }
}

impl<T> NewElement<T> for RationalField where 
    T: Into<Rational>
{
    #[inline]
    fn new(&self, x: T) -> Rational {
        x.into()
    }
}

/// An arbitrary precision rational number. The field `data` is a FLINT
/// [fmpq][flint_sys::fmpq::fmpq].
pub type Rational = Elem<RationalField>;

#[derive(Debug)]
pub struct RationalData {
    pub elem: fmpq,
}

impl Drop for RationalData {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpq::fmpq_clear(&mut self.elem); }
    }
}

impl Element for Rational {
    type Data = RationalData;
    type Parent = RationalField;

    #[inline]
    fn parent(&self) -> RationalField {
        RationalField {}
    }
}

impl AdditiveElement for Rational {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::fmpq::fmpq_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for Rational {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::fmpq::fmpq_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for Rational {}

impl MultiplicativeGroupElement for Rational {}

impl RingElement for Rational {}

impl FieldElement for Rational {}

impl Rational {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpq {
        &self.data.elem
    }
   
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpq {
        &mut self.data.elem
    }

    // TODO: does this share mem?
    /// Returns the numerator of a rational number as an [Integer].
    #[inline]
    pub fn numerator(&self) -> Integer {
        Integer { data: IntegerData { elem: self.data.elem.num } }
    }
    
    // TODO: does this share mem?
    /// Returns the denominator of a rational number as an [Integer].
    #[inline]
    pub fn denominator(&self) -> Integer {
        Integer { data: IntegerData { elem: self.data.elem.den } }
    }

    /// Rounds the rational number down to the nearest [Integer].
    #[inline]
    pub fn floor(&self) -> Integer {
        Integer::fdiv(&self.numerator(), &self.denominator())
    }

    /// Rounds the rational number up to the nearest [Integer].
    #[inline]
    pub fn ceil(&self) -> Integer {
        Integer::cdiv(&self.numerator(), &self.denominator())
    }
    
    /// Rounds the rational number to the nearest [Integer].
    #[inline]
    pub fn round(&self) -> Integer {
        Integer::tdiv(&self.numerator(), &self.denominator())
    }
    
    /// Returns -1 if the rational number is negative, +1 if it is positive, and 0 otherwise.
    #[inline]
    pub fn sign(&self) -> i32 {
        unsafe {
            flint_sys::fmpq::fmpq_sgn(self.as_ptr())
        }
    }

    /// Returns the absolute value of a rational number.
    #[inline]
    pub fn abs(&self) -> Rational {
        unsafe {
            let mut res = Rational::default();
            flint_sys::fmpq::fmpq_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }

    /// Returns the height of a rational number, the largest of the absolute values of its numerator 
    /// and denominator.
    #[inline]
    pub fn height(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpq::fmpq_height(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }

    /// Return the greatest common divisor of rational numbers `(p,q), (r,s)` which is defined to
    /// be the canonicalization of `gcd((ps, qr)/qs)`.
    #[inline]
    pub fn gcd(&self, other: &Rational) -> Rational {
        let mut res = Rational::default();
        unsafe {
            flint_sys::fmpq::fmpq_gcd(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
        res
    }

    /* TODO: make sure this makes sense
    #[inline]
    pub fn xgcd(&self, other: &Rational) -> (Rational, Integer, Integer) {
        unsafe {
            let mut d = Rational::default();
            let mut a = Integer::default();
            let mut b = Integer::default();
            flint_sys::fmpq::fmpq_gcd_cofactors(
                d.as_mut_ptr(), 
                a.as_mut_ptr(), 
                b.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ptr());
            (d, a, b)
        }
    }*/
    
    // TODO: Random, enumeration, continued fractions, special functions, dedekind sums.
}

impl Factorizable for Rational {
    type Output = Product<Integer>;
    fn factor(&self) -> Self::Output {
        assert!(self != &0);
        
        if self == &1 {
            Product::from(Integer::from(1))
        } else { 
            self.numerator().factor() * self.denominator().factor().inv()
        }
    }
}
