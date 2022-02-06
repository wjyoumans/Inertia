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

/// An rational field. For consistency with other types, `RationalField` (aka `Rationals`) can be 
/// initialized and used as an [Rational] factory.
///
/// ```
/// use inertia::prelude::*;
///
/// let qq = Rationals::init();
///
/// // Initialize a new `Rational` as zero.
/// let q1 = qq.default();
///
/// // Initialize a new `Rational` and set it to zero (makes an additional call compared to 
/// // `default`)
/// let q2 = qq.new(0);
///
/// assert_eq!(q1, q2);
/// ```
#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct RationalField {}
pub type Rationals = RationalField;

impl Parent for RationalField {
    type Element = Rational;
    type Context = ();

    /// Return the default value of the field (zero whenever we have an additive structure).
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let qq = Rationals::init();
    /// let q = qq.default();
    /// assert_eq!(q, 0);
    /// ```
    #[inline]
    fn default(&self) -> Rational {
        Rational::default()
    }
}

impl Additive for RationalField {
    /// Return the additive identity zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let qq = Rationals::init();
    /// assert_eq!(qq.zero(), qq.new(0));
    /// ```
    #[inline]
    fn zero(&self) -> Rational {
        Rational::default()
    }
}

impl Multiplicative for RationalField {
    /// Return the multiplicative identity one.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let qq = Rationals::init();
    /// assert_eq!(qq.one(), qq.new(1));
    /// ```
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
   
    /// Return the base field. Subject to change in the future when absolute/relative extension
    /// syntax is solidified.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let qq = Rationals::init();
    /// assert_eq!(qq.base_field(), qq);
    /// ```
    #[inline]
    fn base_field(&self) -> RationalField {
        RationalField {}
    }
}

impl InitParent for RationalField {
    /// Initialize a `RationalField`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let qq = Rationals::init();
    ///
    /// // Initialize a new `Rational` as zero.
    /// let q1 = qq.default();
    ///
    /// // Initialize a new `Rational` and set it to zero (makes an additional call compared to 
    /// // `default`)
    /// let q2 = qq.new(0);
    ///
    /// assert_eq!(q1, q2);
    /// ```
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
    /// Construct a new `Rational`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let qq = Rationals::init();
    ///
    /// let x = qq.new([12, 3]);
    /// assert_eq!(x, 4);
    ///
    /// let x = qq.new("101");
    /// assert_eq!(x, 101);
    /// ```
    #[inline]
    fn new(&self, x: T) -> Rational {
        x.into()
    }
}

/// An arbitrary precision rational number.
///
/// Like all elements of algebraic structures in Inertia, a `Rational` can be constructed from a
/// parent using the `new` method of the [NewElement] trait.
///
/// ```
/// use inertia::prelude::*;
///
/// let qq = Rationals::init();
/// let q = qq.new([12, 2]);
/// assert_eq!(q, 6);
/// ```
///
/// For convenience, we can also use the `From` and `Default` traits to avoid instantiating an
/// `RationalField`.
///
/// ```
/// use inertia::prelude::*;
///
/// let q1 = Rational::from(7);
/// let q2 = Rational::from([14, 2]);
/// assert_eq!(q1, q2);
///
/// let q = Rational::default();
/// assert_eq!(q, 0);
/// ```
///
/// The `rat` macro is provided for making it even easier to instantiate a `Rational`:
///
/// ```
/// use inertia::prelude::*;
///
/// let q = rat!();
/// assert_eq!(q, 0);
///
/// let q = int!(7);
/// assert_eq!(q, 7);
///
/// let q = rat!("123");
/// assert_eq!(q, 123);
///
/// let q = rat!(14, 2);
/// assert_eq!(q, 7);
/// ```
#[derive(Debug)]
pub struct Rational {
    pub data: fmpq,
}

impl Drop for Rational {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpq::fmpq_clear(&mut self.data); }
    }
}

impl Element for Rational {
    type Parent = RationalField;

    /// Return the parent.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q = rat!();
    /// let qq = q.parent();
    ///
    /// assert_eq!(qq, RationalField {});
    /// ```
    #[inline]
    fn parent(&self) -> RationalField {
        RationalField {}
    }
}

impl AdditiveElement for Rational {
    /// Determine if the `Rational` is the additive identity zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = rat!(0u32);
    /// assert!(x.is_zero());
    /// ```
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::fmpq::fmpq_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for Rational {
    /// Determine if the `Rational` is the multiplicative identity one.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = rat!(1i16);
    /// assert!(x.is_one());
    /// ```
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
        &self.data
    }
   
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpq {
        &mut self.data
    }

    /// Returns the numerator of a rational number as an `Integer`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q = rat!([3, 4]);
    /// assert_eq!(q.numerator(), 3);
    /// ```
    #[inline]
    pub fn numerator(&self) -> Integer {
        Integer { data: self.data.num }
    }
    
    /// Returns the denominator of a rational number as an `Integer`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q = rat!([3, 4]);
    /// assert_eq!(q.denominator(), 4);
    /// ```
    #[inline]
    pub fn denominator(&self) -> Integer {
        Integer { data: self.data.den }
    }

    /// Rounds the rational number down to the nearest `Integer`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q = rat!([4, 5]);
    /// assert_eq!(q.floor(), 0);
    /// ```
    #[inline]
    pub fn floor(&self) -> Integer {
        Integer::fdiv_q(&self.numerator(), &self.denominator())
    }

    /// Rounds the rational number up to the nearest `Integer`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q = rat!([4, 5]);
    /// assert_eq!(q.ceil(), 1);
    /// ```
    #[inline]
    pub fn ceil(&self) -> Integer {
        Integer::cdiv_q(&self.numerator(), &self.denominator())
    }
    
    /// Rounds the rational number towards zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q = rat!([4, 5]);
    /// assert_eq!(q.truncate(), 0);
    /// ```
    #[inline]
    pub fn truncate(&self) -> Integer {
        Integer::tdiv_q(&self.numerator(), &self.denominator())
    }
    
    /// Rounds the rational number towards the nearest `Integer`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q = rat!([4, 5]);
    /// assert_eq!(q.round(), 1);
    /// ```
    #[inline]
    pub fn round(&self) -> Integer {
        Integer::ndiv_q(&self.numerator(), &self.denominator())
    }
    
    /// Returns -1 if the rational number is negative, +1 if it is positive, and 0 otherwise.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q = rat!(-10);
    /// assert_eq!(q.sign(), -1);
    ///
    /// let q = rat!(0);
    /// assert_eq!(q.sign(), 0);
    ///
    /// let q = rat!(10);
    /// assert_eq!(q.sign(), 1);
    /// ```
    #[inline]
    pub fn sign(&self) -> i32 {
        unsafe {
            flint_sys::fmpq::fmpq_sgn(self.as_ptr())
        }
    }

    /// Returns the absolute value of a rational number.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q = rat!([-1, 2]);
    /// assert_eq!(q.abs(), rat!([1, 2]));
    /// ```
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
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q = rat!([7, 9]);
    /// assert_eq!(q.height(), 9);
    /// ```
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
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let q1 = rat!([1, 2]);
    /// let q2 = rat!([5, 7]);
    /// assert_eq!(q1.gcd(q2), rat!([1, 14]));
    /// ```
    #[inline]
    pub fn gcd<T>(&self, other: T) -> Rational where
        T: AsRef<Rational>
    {
        let mut res = Rational::default();
        unsafe {
            flint_sys::fmpq::fmpq_gcd(res.as_mut_ptr(), self.as_ptr(), other.as_ref().as_ptr());
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
