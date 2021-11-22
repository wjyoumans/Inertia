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
use rug::ops::*;
use libc::{c_int, c_long, c_ulong};
use crate::traits::*;
use crate::integer::src::Integer;
use crate::ratpol::src::RatPol;

impl_cmp_unsafe! {
    eq
    RatPol
    flint_sys::fmpq_poly::fmpq_poly_equal
}

impl_cmp_unsafe! {
    eq
    RatPol, Integer
    fmpq_poly_equal_fmpz
}

impl_cmp_unsafe! {
    eq
    RatPol, u64 {u64 u32 u16 u8}
    fmpq_poly_equal_ui
}

impl_cmp_unsafe! {
    eq
    RatPol, i64 {i64 i32 i16 i8}
    fmpq_poly_equal_si
}

impl_unop_unsafe! {
    RatPol
    Neg {neg}
    NegAssign {neg_assign}
    flint_sys::fmpq_poly::fmpq_poly_neg
}

impl_binop_unsafe! {
    RatPol, RatPol, RatPol
    
    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    flint_sys::fmpq_poly::fmpq_poly_add;
    
    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    flint_sys::fmpq_poly::fmpq_poly_sub;
    
    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    flint_sys::fmpq_poly::fmpq_poly_mul;
    
    Rem {rem}
    RemAssign {rem_assign}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    flint_sys::fmpq_poly::fmpq_poly_rem;
}

impl_binop_unsafe! {
    op_assign
    RatPol, Integer, RatPol
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    flint_sys::fmpq_poly::fmpq_poly_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    flint_sys::fmpq_poly::fmpq_poly_sub_fmpz;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpq_poly::fmpq_poly_scalar_mul_fmpz;
   
    /*
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    flint_sys::fmpq_poly::fmpq_poly_scalar_mod_fmpz;
    */
}

impl_binop_unsafe! {
    op_from
    Integer, RatPol, RatPol
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_poly_fmpz_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    flint_sys::fmpq_poly::fmpq_poly_fmpz_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_poly_fmpz_scalar_mul;
   
    /*
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpq_poly_fmpq_scalar_mod;
    */
}

impl_binop_unsafe! {
    op_assign
    RatPol, u64 {u64 u32 u16 u8}, RatPol
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpq_poly_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpq_poly_sub_ui;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpq_poly::fmpq_poly_scalar_mul_ui;
    
    /*
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpq_poly_scalar_mod_ui;
    */
}

impl_binop_unsafe! {
    op_assign
    RatPol, i64 {i64 i32 i16 i8}, RatPol
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    flint_sys::fmpq_poly::fmpq_poly_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    flint_sys::fmpq_poly::fmpq_poly_sub_si;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpq_poly::fmpq_poly_scalar_mul_si;
    
    /*
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpq_poly_scalar_mod_si;
    */
}

impl_binop_unsafe! {
    op_from
    u64 {u64 u32 u16 u8}, RatPol, RatPol
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_poly_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_poly_ui_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_poly_ui_scalar_mul;
    
    /*
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpq_poly_ui_scalar_mod;
    */
}

impl_binop_unsafe! {
    op_from
    i64 {i64 i32 i16 i8}, RatPol, RatPol
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_poly_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_poly_si_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_poly_si_scalar_mul;
    
    /*
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpq_poly_si_scalar_mod;
    */
}

unsafe fn fmpq_poly_equal_fmpz(
    f: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    x: *const flint_sys::fmpz::fmpz,
    ) -> c_int
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
    flint_sys::fmpq_poly::fmpq_poly_set_fmpz(z.as_mut_ptr(), x);
    flint_sys::fmpq_poly::fmpq_poly_equal(f, z.as_ptr())
}

unsafe fn fmpq_poly_equal_ui(
    f: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    x: c_ulong,
    ) -> c_int
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
    flint_sys::fmpq_poly::fmpq_poly_set_ui(z.as_mut_ptr(), x);
    flint_sys::fmpq_poly::fmpq_poly_equal(f, z.as_ptr())
}

unsafe fn fmpq_poly_equal_si(
    f: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    x: c_long,
    ) -> c_int
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
    flint_sys::fmpq_poly::fmpq_poly_set_si(z.as_mut_ptr(), x);
    flint_sys::fmpq_poly::fmpq_poly_equal(f, z.as_ptr())
}

unsafe fn fmpq_poly_add_ui(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    x: c_ulong,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(z.as_mut_ptr(), x);
    flint_sys::fmpq_poly::fmpq_poly_add_fmpz(res, f, z.as_ptr());
}

unsafe fn fmpq_poly_sub_ui(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    x: c_ulong,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(z.as_mut_ptr(), x);
    flint_sys::fmpq_poly::fmpq_poly_sub_fmpz(res, f, z.as_ptr());
}

/*
unsafe fn fmpq_poly_scalar_mod_ui(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    x: c_ulong,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
    flint_sys::fmpq_poly::fmpq_poly_set_ui(z.as_mut_ptr(), x);
    flint_sys::fmpq_poly::fmpq_poly_rem(res, f, z.as_ptr());
}

unsafe fn fmpq_poly_scalar_mod_si(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    x: c_long,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
    flint_sys::fmpq_poly::fmpq_poly_set_si(z.as_mut_ptr(), x);
    flint_sys::fmpq_poly::fmpq_poly_rem(res, f, z.as_ptr());
}*/

unsafe fn fmpq_poly_fmpz_add(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: *const flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    )
{
    flint_sys::fmpq_poly::fmpq_poly_add_fmpz(res, g, f);
}

unsafe fn fmpq_poly_fmpz_scalar_mul(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: *const flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    )
{
    flint_sys::fmpq_poly::fmpq_poly_scalar_mul_fmpz(res, g, f);
}

/*
unsafe fn fmpq_poly_fmpz_scalar_mod(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: *const flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_fmpz(res, f);
    flint_sys::fmpz_poly::fmpz_poly_rem(res, res, g);
}*/

unsafe fn fmpq_poly_ui_add(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: c_ulong,
    g: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(z.as_mut_ptr(), f);
    flint_sys::fmpq_poly::fmpq_poly_add_fmpz(res, g, z.as_ptr());
}

unsafe fn fmpq_poly_ui_sub(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: c_ulong,
    g: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(z.as_mut_ptr(), f);
    flint_sys::fmpq_poly::fmpq_poly_fmpz_sub(res, z.as_ptr(), g);
}

unsafe fn fmpq_poly_ui_scalar_mul(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: c_ulong,
    g: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    )
{
    flint_sys::fmpq_poly::fmpq_poly_scalar_mul_ui(res, g, f);
}

/*
unsafe fn fmpq_poly_ui_scalar_mod(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: c_ulong,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_ui(res, f);
    flint_sys::fmpz_poly::fmpz_poly_rem(res, res, g);
}
*/

unsafe fn fmpq_poly_si_add(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: c_long,
    g: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    )
{
    flint_sys::fmpq_poly::fmpq_poly_add_si(res, g, f);
}

unsafe fn fmpq_poly_si_sub(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: c_long,
    g: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    )
{
    flint_sys::fmpq_poly::fmpq_poly_sub_si(res, g, f);
    flint_sys::fmpq_poly::fmpq_poly_neg(res, res);
}

unsafe fn fmpq_poly_si_scalar_mul(
    res: *mut flint_sys::fmpq_poly::fmpq_poly_struct,
    f: c_long,
    g: *const flint_sys::fmpq_poly::fmpq_poly_struct,
    )
{
    flint_sys::fmpq_poly::fmpq_poly_scalar_mul_si(res, g, f);
}

/*
unsafe fn fmpq_poly_si_scalar_mod(
    res: *mut flint_sys::fmpz_poly::fmpz_poly_struct,
    f: c_long,
    g: *const flint_sys::fmpz_poly::fmpz_poly_struct,
    )
{
    flint_sys::fmpz_poly::fmpz_poly_set_si(res, f);
    flint_sys::fmpz_poly::fmpz_poly_rem(res, res, g);
}
*/
