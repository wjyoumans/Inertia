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

use std::sync::Arc;


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

/// Modulo of two items with assignment into a third.
pub trait AssignRem<T, U> {
    fn assign_rem(&mut self, lhs: T, rhs: U);
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
    pub ctx: Arc<T::Data>,
    pub data: <T::Element as Element>::Data,
}

impl<T: Parent> Drop for Elem<T> {
    default fn drop(&mut self) {}
}

/// Inverse as a unary operation.
pub trait Inv {
    type Output;
    fn inv(self) -> Self::Output;
}

/// Inverse with assignment.
pub trait InvAssign {
    fn inv_assign(&mut self);
}

/// Exponentiation.
pub trait Pow<T> {
    type Output;
    fn pow(&self, exp: T) -> Self::Output;
}

/// Modular exponentiation.
pub trait Powm<T, U> {
    type Output;
    fn powm(&self, exp: T, modulus: U) -> Self::Output;
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
pub trait EvaluateFacMod<T> {
    type Output;
    fn evaluate_mod(&self, modulus: T) -> Self::Output;
}
