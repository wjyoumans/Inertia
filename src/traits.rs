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

/// Traits for operations and algebraic structures.


/// Inverse as a unary operation.
pub trait Inv {
    type Output;
    fn inv(self) -> Self::Output;
}

/// Inverse with assignment.
pub trait InvAssign {
    fn inv_assign(&mut self);
}

/// Bitwise `and` of two items with assignment into a third.
pub trait AssignBitAnd<T, U> {
    fn assign_bitand(&mut self, lhs: T, rhs: U);
}

/// Bitwise `or` of two items with assignment into a third.
pub trait AssignBitOr<T, U> {
    fn assign_bitor(&mut self, lhs: T, rhs: U);
}

/// Bitwise `xor` of two items with assignment into a third.
pub trait AssignBitXor<T, U> {
    fn assign_bitxor(&mut self, lhs: T, rhs: U);
}

/// Addition of two items with assignment into a third.
pub trait AssignAdd<T, U> {
    fn assign_add(&mut self, lhs: T, rhs: U);
}

/// Subtraction of two items with assignment into a third.
pub trait AssignSub<T, U> {
    fn assign_sub(&mut self, lhs: T, rhs: U);
}

/// Multiplication of two items with assignment into a third.
pub trait AssignMul<T, U> {
    fn assign_mul(&mut self, lhs: T, rhs: U);
}

/// Division of two items with assignment into a third.
pub trait AssignDiv<T, U> {
    fn assign_div(&mut self, lhs: T, rhs: U);
}

/// Exponentiation of two items with assignment into a third.
pub trait AssignPow<T, U> {
    fn assign_pow(&mut self, lhs: T, rhs: U);
}

/// Remainder of two items with assignment into a third.
pub trait AssignRem<T, U> {
    fn assign_rem(&mut self, lhs: T, rhs: U);
}

/// Evaluation of an expression at `x`.
pub trait Evaluate<T> {
    type Output;
    fn evaluate(&self, x: T) -> Self::Output;
}

/// Modular evaluation of an expression at `x`.
pub trait EvaluateMod<T, U> {
    type Output;
    fn evaluate_mod(&self, x: T, modulus: U) -> Self::Output;
}

/// Evaluation of a `Product`.
pub trait EvaluateProduct {
    type Output;
    fn evaluate(&self) -> Self::Output;
}

/// Modular evaluation of a `Product`.
pub trait EvaluateProductMod<T> {
    type Output;
    fn evaluate_mod(&self, modulus: T) -> Self::Output;
}

/// Factorization.
pub trait Factorizable {
    type Output;
    fn factor(&self) -> Self::Output;
}

// Traits for implementing different initializations of Parents

pub trait Init: Parent {
    fn init() -> Self;
}

pub trait Init1<T>: Parent {
    fn init(x: T) -> Self;
}

pub trait Init2<T, U>: Parent {
    fn init(x: T, y: U) -> Self;
}

pub trait Init3<T, U, V>: Parent {
    fn init(x: T, y: U, z: V) -> Self;
}

// Traits for implementing different Element constructors for Parents

pub trait New<T>: Parent {
    fn new(&self, x: T) -> <Self as Parent>::Element;
}

/// A generic parent, for example an algebraic structure like a ring.
pub trait Parent {
    type Data;
    type Element: Element;
}

/// An generic element of a `Parent`.
pub trait Element {
    type Data;
    type Parent: Parent;
}

pub trait Additive: Parent {
    fn zero(&self) -> <Self as Parent>::Element;
}
pub trait AdditiveElement: Element {
    fn is_zero(&self) -> bool;
}

pub trait Multiplicative: Parent {
    fn one(&self) -> <Self as Parent>::Element;
}
pub trait MultiplicativeElement: Element {
    fn is_one(&self) -> bool;
}

pub trait AdditiveGroup: Additive {
    #[inline]
    fn identity(&self) -> <Self as Parent>::Element {
        self.zero()
    }
}
pub trait AdditiveGroupElement: AdditiveElement {
    #[inline]
    fn is_identity(&self) -> bool {
        self.is_zero()
    }
}

pub trait MultiplicativeGroup: Multiplicative {
    #[inline]
    fn identity(&self) -> <Self as Parent>::Element {
        self.one()
    }
}
pub trait MultiplicativeGroupElement: MultiplicativeElement {
    #[inline]
    fn is_identity(&self) -> bool {
        self.is_one()
    }
}

pub trait Module: AdditiveGroup {}
pub trait ModuleElement: AdditiveGroupElement {}

pub trait VectorSpace: Module {}
pub trait VectorSpaceElement: ModuleElement {}

pub trait MatrixSpace: VectorSpace {}
pub trait MatrixSpaceElement: VectorSpaceElement {
    // nrows
    // ncols
    // one
    // zero
    // get_entry
    // set_entry
    // is_empty
    // is_square
    // hcat
    // vcat
    // solve
    // rref
    // nullspace
}

pub trait Ring: AdditiveGroup + MultiplicativeGroup {}
pub trait RingElement: AdditiveGroupElement + MultiplicativeGroupElement {}

pub trait PolynomialRing<T: Ring>: Ring {
    // gen
    // basis (indeterminates)
}
pub trait PolynomialRingElement<T: Ring>: RingElement {
    // len
    // degree
    // get_coeff
    // set_coeff
    // coefficients
}

pub trait Field: Ring {
    // basis
    // gen
}
pub trait FieldElement: RingElement {
    // norm(&self)
    // trace(&self)
}

pub trait NumberField: Field {} // + PolynomialRing (Q[x]/f)
pub trait NumberFieldElement: FieldElement {} // + PolynomialRingElement

/// An element of a `Parent`. In cases where the parent holds important context data we use the 
/// thread-safe [Arc] reference counter to avoid cleaning up the parent until all elements are dropped.
pub struct Elem<T: Parent> {
    pub ctx: T::Data,
    pub data: <T::Element as Element>::Data,
}

impl<T: Parent> Drop for Elem<T> {
    default fn drop(&mut self) {}
}

impl<T: Parent> fmt::Debug for Elem<T> {
    default fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Debug not implemented.")
    }
}
