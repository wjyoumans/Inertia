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

#[doc(hidden)]
/// A wrapper for structs coming from FFI bindings.
pub struct Wrap<T> {
    pub wrap: T,
}

impl<T> Drop for Wrap<T> {
    default fn drop(&mut self) {}
}

/// An element of a `Parent`. We use the thread-safe [Arc] reference counter to avoid cleaning up
/// the parent until all elements are dropped.
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

/*
pub trait Zero: Parent {
    fn zero(&self) -> Self::Element;
}

pub trait One: Parent {
    fn one(&self) -> Self::Element;
}
*/
