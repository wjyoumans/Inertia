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
//use std::str::FromStr;
use std::cell::RefCell;
use std::rc::Rc;

use inertia_core as core;
use inertia_core::{impl_unop, impl_from, impl_binop};

use inertia_algebra::*;
use inertia_algebra::ops::*;
use inertia_generic::poly::GenericPolyRing;
use inertia_generic::mat::GenericMatSpace;

use crate::{Integer, Integers};
use crate::poly::IntoPolyRing;
use crate::mat::IntoMatSpace;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////
// IntPolyCtx
///////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PolyCtx<T> 
where
    T: Clone + fmt::Debug + Eq + Hash + PartialEq
{
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "T: Serialize",
            deserialize = "T: Deserialize<'de>",
        ))
    )]
    base_ring: T,
    var: RefCell<String>
}

impl<T> Hash for PolyCtx<T> 
where
    T: Clone + fmt::Debug + Eq + Hash + PartialEq
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring.hash(state);
        self.var.borrow().hash(state);
    }
}

impl<T> PolyCtx<T>
where
    T: Clone + fmt::Debug + Eq + Hash + PartialEq
{
    #[inline]
    pub fn new(ring: &T, var: String) -> Self {
        PolyCtx {
            base_ring: ring.clone(),
            var: RefCell::new(var)
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntPolyCtx(Rc<PolyCtx<Integers>>);

impl Default for IntPolyCtx {
    #[inline]
    fn default() -> Self {
        IntPolyCtx::new("x".to_string())
    }
}

impl IntPolyCtx {
    #[inline]
    pub fn new(var: String) -> Self {
        IntPolyCtx(Rc::new(PolyCtx::new(&Integers::init(), var)))
    }
}

///////////////////////////////////////////////////////////////////////
// IntPolyRing
///////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntPolyRing {
    ctx: IntPolyCtx
}

impl fmt::Display for IntPolyRing {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Univariate polynomial ring in {} over {}", self.var(), 
               self.base_ring())
    }
}

impl Parent for IntPolyRing {
    type Element = IntPoly;
}

impl Identity<Additive> for IntPolyRing {
    #[inline]
    fn identity(&self) -> IntPoly {
        IntPoly::from_raw(core::IntPoly::zero(), self.context().clone())
    }
}

impl Divisible<Additive> for IntPolyRing {}

impl Associative<Additive> for IntPolyRing {}

impl Commutative<Additive> for IntPolyRing {}

impl Identity<Multiplicative> for IntPolyRing {
    #[inline]
    fn identity(&self) -> IntPoly {
        IntPoly::from_raw(core::IntPoly::one(), self.context().clone())
    }
}

impl Associative<Multiplicative> for IntPolyRing {}

impl Commutative<Multiplicative> for IntPolyRing {}

impl Distributive for IntPolyRing {}

impl PolynomialRing<Integers> for IntPolyRing {
    type Element = IntPoly;

    #[inline]
    fn init<S: Into<String>>(_: &Integers, var: S) -> Self {
        IntPolyRing { ctx: IntPolyCtx::new(var.into()) }
    }

    #[inline]
    fn base_ring(&self) -> &Integers {
        &self.context().0.base_ring
    }

    #[inline]
    fn var(&self) -> String {
        self.context().0.var.borrow().to_string()
    }

    #[inline]
    fn set_var<S: Into<String>>(&mut self, var: S) {
        self.context().0.var.replace(var.into());
    }
}

impl IntoPolyRing for IntPolyRing {
    type Inner = GenericPolyRing<Self>;
}

impl IntoMatSpace for IntPolyRing {
    type Inner = GenericMatSpace<Self>;
}

impl IntPolyRing {
    pub fn context(&self) -> &IntPolyCtx {
        &self.ctx
    }
}

///////////////////////////////////////////////////////////////////////
// IntPoly
///////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntPoly {
    inner: core::IntPoly,
    ctx: IntPolyCtx
}

impl AsRef<IntPoly> for IntPoly {
    #[inline]
    fn as_ref(&self) -> &IntPoly {
        self
    }
}

impl fmt::Display for IntPoly {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner().fmt(f)
    }
}

impl Element for IntPoly {
    type Parent = IntPolyRing;

    #[inline]
    fn parent(&self) -> IntPolyRing {
        IntPolyRing { ctx: self.context().clone() }
    }
}

impl Operation<Additive> for IntPoly {
    #[inline]
    fn operate(&self, right: &Self) -> Self {
        let ctx = self.context();
        assert_eq!(ctx, right.context());
        IntPoly::from_raw(self.inner() + right.inner(), ctx.clone())
    }
}

impl IsIdentity<Additive> for IntPoly {
    #[inline]
    fn is_identity(&self) -> bool {
        self.inner().is_zero()
    }
}

impl TwoSidedInverse<Additive> for IntPoly {
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        IntPoly::from_raw(-self.inner(), self.context().clone())
    }
}

impl Operation<Multiplicative> for IntPoly {
    #[inline]
    fn operate(&self, right: &Self) -> Self {
        let ctx = self.context();
        assert_eq!(ctx, right.context());
        IntPoly::from_raw(self.inner() * right.inner(), ctx.clone())
    }
}

impl IsIdentity<Multiplicative> for IntPoly {
    #[inline]
    fn is_identity(&self) -> bool {
        self.inner().is_one()
    }
}

impl PolynomialRingElement<Integers> for IntPoly {
    type Parent = IntPolyRing;

    type Borrow<'a> = &'a Integer;
    type BorrowMut<'a> = &'a mut Integer;

    #[inline]
    fn base_ring(&self) -> &Integers {
        &self.context().0.base_ring
    }
    
    #[inline]
    fn var(&self) -> String {
        self.parent().var()
    }

    #[inline]
    fn len(&self) -> usize {
        self.inner().len()
    }

    #[inline]
    fn degree(&self) -> i64 {
        self.inner().degree()
    }

    #[inline]
    fn get_coefficient(&self, i: usize) -> Integer {
        Integer(self.inner.get_coeff(i))
    }
    
    #[inline]
    fn set_coefficient(&mut self, i: usize, coeff: Integer) {
        self.inner.set_coeff(i, &coeff.0)
    }
    
    #[inline]
    fn get_coefficients(&self) -> Vec<Integer> {
        self.inner.get_coeffs().into_iter().map(|x| Integer(x)).collect()
    }
    
    /*
    #[inline]
    fn coeff_mut(&mut self, i: usize) -> &mut Integer {
        unimplemented!()
    }

    #[inline]
    fn coefficients(&self) -> Vec<Integer> {
        //self.0.coefficients()
        unimplemented!()
    }*/
}

impl IntPoly {
    pub fn inner(&self) -> &inertia_core::IntPoly {
        &self.inner
    }
    
    pub fn into_inner(self) -> inertia_core::IntPoly {
        self.inner
    }
    
    pub fn inner_mut(&mut self) -> &mut inertia_core::IntPoly {
        &mut self.inner
    }

    pub fn from_raw(inner: core::IntPoly, ctx: IntPolyCtx) -> Self {
        IntPoly { inner, ctx }
    }
    
    pub fn context(&self) -> &IntPolyCtx {
        &self.ctx
    }
}

///////////////////////////////////////////////////////////////////////
// Constructor
///////////////////////////////////////////////////////////////////////

impl<T> NewElement<T> for IntPolyRing
where
    core::IntPoly: New<T>
{
    #[inline]
    fn new(&self, src: T) -> IntPoly {
        IntPoly::from_raw(core::IntPoly::new(src), self.context().clone())
    }
}

///////////////////////////////////////////////////////////////////////
// Conversion
///////////////////////////////////////////////////////////////////////

derive_from!{IntPoly, IntPolyCtx, usize u8 u16 u32 u64 isize i8 i16 i32 i64}

impl_from! {
    IntPoly, Integer
    {
        fn from(src: &Integer) -> Self {
            IntPoly::from_raw(core::IntPoly::from(src.inner()), IntPolyCtx::default())
        }
    }
}

///////////////////////////////////////////////////////////////////////
// Ops
///////////////////////////////////////////////////////////////////////

derive_unop! {
    ctx
    IntPoly

    Neg, neg
    NegAssign, neg_assign
}

derive_binop! {
    ctx
    IntPoly, IntPoly, IntPoly
    
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

derive_binop! {
    ctx
    op_assign
    IntPoly, Integer, IntPoly
    
    Add, add
    AddAssign, add_assign
    AssignAdd, assign_add
    
    Sub, sub
    SubAssign, sub_assign
    AssignSub, assign_sub
    
    Mul, mul
    MulAssign, mul_assign
    AssignMul, assign_mul
    
    Rem, rem
    RemAssign, rem_assign
    AssignRem, assign_rem
}

derive_binop! {
    ctx_rhs
    op_from
    Integer, IntPoly, IntPoly
    
    Add, add
    AddFrom, add_from
    AssignAdd, assign_add
    
    Sub, sub
    SubFrom, sub_from
    AssignSub, assign_sub
    
    Mul, mul
    MulFrom, mul_from
    AssignMul, assign_mul
    
    Rem, rem
    RemFrom, rem_from
    AssignRem, assign_rem
}

/*
derive_binop! {
    IntPoly, {u64}, IntPoly

    Add, add
    AddAssign, add_assign
    AssignAdd, assign_add
}
*/

/*
///////////////////////////////////////////////////////////////////////
// Ops
///////////////////////////////////////////////////////////////////////

use inertia_core::impl_binop;

derive_binops! {
    IntPoly, IntPoly, IntPoly
    
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
}

derive_scalar_binops! {
    IntPoly, Integer
    
    Add, add
    AddAssign, add_assign
    AddFrom, add_from
    AssignAdd, assign_add
}*/

use crate::Poly;

derive_wrapper_binops! {
    Poly<Integers>, {Integer}
    
    Add, add
    AddAssign, add_assign
    AddFrom, add_from
    AssignAdd, assign_add
    
    Mul, mul
    MulAssign, mul_assign
    MulFrom, mul_from
    AssignMul, assign_mul
}
