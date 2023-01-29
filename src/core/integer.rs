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
use std::str::FromStr;

use inertia_core as core;
use inertia_core::{impl_unop, impl_from, impl_binop};

use inertia_algebra::*;
use inertia_algebra::ops::*;
use inertia_generic::mat::GenericMatSpace;

use crate::{IntPolyRing, IntoPolyRing, IntoMatSpace};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////
// IntegerRing
///////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntegerRing {}
pub type Integers = IntegerRing;

impl fmt::Display for Integers {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Integer ring")
    }
}

impl Parent for Integers {
    type Element = Integer;
}

impl Identity<Additive> for Integers {
    fn identity(&self) -> Integer {
        Integer(core::Integer::zero())
    }
}

impl Divisible<Additive> for Integers {}

impl Associative<Additive> for Integers {}

impl Commutative<Additive> for Integers {}

impl Identity<Multiplicative> for Integers {
    fn identity(&self) -> Integer {
        Integer(core::Integer::one())
    }
}

impl Associative<Multiplicative> for Integers {}

impl Commutative<Multiplicative> for Integers {}

impl Distributive for Integers {}

impl IntoPolyRing for Integers {
    type Inner = IntPolyRing;
}

impl IntoMatSpace for Integers {
    type Inner = GenericMatSpace<Self>;
}

impl Integers {
    #[inline]
    pub fn init() -> Self {
        Integers {}
    }
}

///////////////////////////////////////////////////////////////////////
// Integer
///////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Integer(pub(crate) core::Integer);

impl AsRef<Integer> for Integer {
    #[inline]
    fn as_ref(&self) -> &Integer {
        self
    }
}

impl fmt::Display for Integer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Element for Integer {
    type Parent = Integers;
    
    #[inline]
    fn parent(&self) -> Integers {
        Integers {}
    }
}

impl Operation<Additive> for Integer {
    #[inline]
    fn operate(&self, right: &Self) -> Self {
        Integer(&self.0 + &right.0)
    }
}

impl IsIdentity<Additive> for Integer {
    #[inline]
    fn is_identity(&self) -> bool {
        self.0.is_zero()
    }
}

impl TwoSidedInverse<Additive> for Integer {
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        Integer(-&self.0)
    }
}

impl Operation<Multiplicative> for Integer {
    #[inline]
    fn operate(&self, right: &Self) -> Self {
        Integer(&self.0 * &right.0)
    }
}

impl IsIdentity<Multiplicative> for Integer {
    #[inline]
    fn is_identity(&self) -> bool {
        self.0.is_one()
    }
}

impl Integer {
    #[inline]
    pub fn zero() -> Self {
        Integer(core::Integer::zero())
    }
    
    #[inline]
    pub fn one() -> Self {
        Integer(core::Integer::one())
    }

    #[inline]
    pub fn with_capacity(limbs: u64) -> Self {
        Integer(core::Integer::with_capacity(limbs))
    }
    
    #[inline]
    pub fn into_inner(self) -> core::Integer {
        self.0
    }
    
    #[inline]
    pub fn inner(&self) -> &core::Integer {
        &self.0
    }
    
    #[inline]
    pub fn inner_mut(&mut self) -> &mut core::Integer {
        &mut self.0
    }

    #[inline]
    pub fn to_str_radix(&self, base: u8) -> String {
        core::Integer::to_str_radix(&self.0, base)
    }

    #[inline]
    pub fn from_raw(inner: core::Integer) -> Self {
        Integer(inner)
    }
}

///////////////////////////////////////////////////////////////////////
// Constructor
///////////////////////////////////////////////////////////////////////

impl<T> NewElement<T> for Integers
where
    Integer: New<T>
{
    #[inline]
    fn new(&self, src: T) -> Integer {
        Integer::new(src)
    }
}

impl<T: Into<Integer>> New<T> for Integer {
    #[inline]
    fn new(src: T) -> Self {
        src.into()
    }
}

impl New<&Integer> for Integer {
    #[inline]
    fn new(src: &Integer) -> Self {
        src.clone()
    }
}

///////////////////////////////////////////////////////////////////////
// Conversion
///////////////////////////////////////////////////////////////////////

// TODO: IntMod

derive_from!{Integer, usize u8 u16 u32 u64 isize i8 i16 i32 i64}

impl FromStr for Integer {
    type Err = <core::Integer as FromStr>::Err;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Integer(core::Integer::from_str(s)?))
    }
}

///////////////////////////////////////////////////////////////////////
// Ops
///////////////////////////////////////////////////////////////////////

derive_unop! {
    None
    Integer

    Neg, neg
    NegAssign, neg_assign
}

derive_binop! {
    None
    Integer, Integer, Integer
    
    Add, add
    AddAssign, add_assign
    AddFrom, add_from
    AssignAdd, assign_add
    
    Sub, sub
    SubAssign, sub_assign
    SubFrom, sub_from
    AssignSub, assign_sub
    
    Mul, mul
    MulAssign, mul_assign
    MulFrom, mul_from
    AssignMul, assign_mul
    
    Rem, rem
    RemAssign, rem_assign
    RemFrom, rem_from
    AssignRem, assign_rem
}

/*
derive_binop! {
    Integer, {u64}, Integer

    Add, add
    AddAssign, add_assign
    AssignAdd, assign_add
}
*/
