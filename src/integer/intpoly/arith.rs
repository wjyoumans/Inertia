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
use std::ops::*;

use libc::{c_int, c_long, c_ulong};
use rug::ops::*;

use crate::*;

// TODO: Div, Pow, Inv -> RatFunc

impl_cmp_unsafe! {
    eq
    IntPoly
    flint_sys::fmpz_poly::fmpz_poly_equal
}

impl_cmp_unsafe! {
    eq
    IntPoly, Integer
    flint_sys::fmpz_poly::fmpz_poly_equal_fmpz
}

impl_cmp_unsafe! {
    eq
    IntPoly, Rational
    fmpz_poly_equal_fmpq
}

impl_cmp_unsafe! {
    eq
    IntPoly, u64 {u64 u32 u16 u8}
    fmpz_poly_equal_ui
}

impl_cmp_unsafe! {
    eq
    IntPoly, i64 {i64 i32 i16 i8}
    fmpz_poly_equal_si
}

impl_unop_unsafe! {
    None
    IntPoly
    Neg {neg}
    NegAssign {neg_assign}
    flint_sys::fmpz_poly::fmpz_poly_neg
}

impl_binop_unsafe! {
    None
    IntPoly, IntPoly, IntPoly
    
    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    flint_sys::fmpz_poly::fmpz_poly_add;
    
    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    flint_sys::fmpz_poly::fmpz_poly_sub;
    
    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    flint_sys::fmpz_poly::fmpz_poly_mul;
    
    Rem {rem}
    RemAssign {rem_assign}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    flint_sys::fmpz_poly::fmpz_poly_rem;
}

impl_binop_unsafe! {
    None
    op_assign
    IntPoly, Integer, IntPoly
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    flint_sys::fmpz_poly::fmpz_poly_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    flint_sys::fmpz_poly::fmpz_poly_sub_fmpz;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpz_poly::fmpz_poly_scalar_mul_fmpz;
    
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    flint_sys::fmpz_poly::fmpz_poly_scalar_mod_fmpz;
}

impl_binop_unsafe! {
    None
    op_from
    Integer, IntPoly, IntPoly
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_poly_fmpz_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    flint_sys::fmpz_poly::fmpz_poly_fmpz_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_poly_fmpz_scalar_mul;
    
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_poly_fmpz_scalar_mod;
}

impl_binop_unsafe! {
    None
    op_assign
    IntPoly, u64 {u64 u32 u16 u8}, IntPoly
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_poly_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_poly_sub_ui;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpz_poly::fmpz_poly_scalar_mul_ui;
    
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_poly_scalar_mod_ui;
}

impl_binop_unsafe! {
    None
    op_assign
    IntPoly, i64 {i64 i32 i16 i8}, IntPoly
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    flint_sys::fmpz_poly::fmpz_poly_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    flint_sys::fmpz_poly::fmpz_poly_sub_si;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpz_poly::fmpz_poly_scalar_mul_si;
    
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_poly_scalar_mod_si;
}

impl_binop_unsafe! {
    None
    op_from
    u64 {u64 u32 u16 u8}, IntPoly, IntPoly
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_poly_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_poly_ui_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_poly_ui_scalar_mul;
    
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_poly_ui_scalar_mod;
}

impl_binop_unsafe! {
    None
    op_from
    i64 {i64 i32 i16 i8}, IntPoly, IntPoly
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_poly_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_poly_si_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_poly_si_scalar_mul;
    
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_poly_si_scalar_mod;
}

#[inline]
unsafe fn fmpz_poly_equal_fmpq(
    f: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    x: *const flint_sys::fmpq::fmpq,
    ) -> c_int
{
    if flint_sys::fmpz::fmpz_is_one(&(*x).den) == 1 {
        flint_sys::fmpz_poly::fmpz_poly_equal_fmpz(f, &(*x).num)
    } else {
        0
    }
}

#[inline]
unsafe fn fmpz_poly_equal_ui(
    f: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    x: c_ulong,
    ) -> c_int
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(z.as_mut_ptr(), x);
    let b = flint_sys::fmpz_poly::fmpz_poly_equal_fmpz(f, z.as_ptr());
    flint_sys::fmpz::fmpz_clear(z.as_mut_ptr());
    b
}

#[inline]
unsafe fn fmpz_poly_equal_si(
    f: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    x: c_long,
    ) -> c_int
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_si(z.as_mut_ptr(), x);
    let b = flint_sys::fmpz_poly::fmpz_poly_equal_fmpz(f, z.as_ptr());
    flint_sys::fmpz::fmpz_clear(z.as_mut_ptr());
    b
}

#[inline]
unsafe fn fmpz_poly_add_ui(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    x: c_ulong,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_ui(res, x);
    flint_sys::fmpz_poly::fmpz_poly_add(res, f, res);
}

#[inline]
unsafe fn fmpz_poly_sub_ui(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    x: c_ulong,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_ui(res, x);
    flint_sys::fmpz_poly::fmpz_poly_sub(res, f, res);
}

#[inline]
unsafe fn fmpz_poly_scalar_mod_ui(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    x: c_ulong,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_ui(res, x);
    flint_sys::fmpz_poly::fmpz_poly_rem(res, f, res);
}

#[inline]
unsafe fn fmpz_poly_scalar_mod_si(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    x: c_long,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_si(res, x);
    flint_sys::fmpz_poly::fmpz_poly_rem(res, f, res);
}

#[inline]
unsafe fn fmpz_poly_fmpz_add(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: *const flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_add_fmpz(res, g, f);
}

#[inline]
unsafe fn fmpz_poly_fmpz_scalar_mul(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: *const flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_scalar_mul_fmpz(res, g, f);
}

#[inline]
unsafe fn fmpz_poly_fmpz_scalar_mod(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: *const flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_fmpz(res, f);
    flint_sys::fmpz_poly::fmpz_poly_rem(res, res, g);
}

#[inline]
unsafe fn fmpz_poly_ui_add(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: c_ulong,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_ui(res, f);
    flint_sys::fmpz_poly::fmpz_poly_add(res, res, g);
}

#[inline]
unsafe fn fmpz_poly_ui_sub(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: c_ulong,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_ui(res, f);
    flint_sys::fmpz_poly::fmpz_poly_sub(res, res, g);
}

#[inline]
unsafe fn fmpz_poly_ui_scalar_mul(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: c_ulong,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_scalar_mul_ui(res, g, f);
}

#[inline]
unsafe fn fmpz_poly_ui_scalar_mod(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: c_ulong,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_ui(res, f);
    flint_sys::fmpz_poly::fmpz_poly_rem(res, res, g);
}

#[inline]
unsafe fn fmpz_poly_si_add(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: c_long,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_add_si(res, g, f);
}

#[inline]
unsafe fn fmpz_poly_si_sub(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: c_long,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_sub_si(res, g, f);
    flint_sys::fmpz_poly::fmpz_poly_neg(res, res);
}

#[inline]
unsafe fn fmpz_poly_si_scalar_mul(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: c_long,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_scalar_mul_si(res, g, f);
}

#[inline]
unsafe fn fmpz_poly_si_scalar_mod(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: c_long,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_si(res, f);
    flint_sys::fmpz_poly::fmpz_poly_rem(res, res, g);
}