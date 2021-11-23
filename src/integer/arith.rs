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

use std::cmp::Ordering::{self, Less, Greater, Equal};
use std::mem::MaybeUninit;
use std::ops::*;

use libc::{c_long, c_ulong};
use rug::ops::*;

use crate::traits::*;
use crate::integer::src::Integer;

impl_cmp_unsafe! {
    eq
    Integer
    flint_sys::fmpz::fmpz_equal
}

impl_cmp_unsafe! {
    ord
    Integer
    flint_sys::fmpz::fmpz_cmp
}

impl_cmp_unsafe! {
    eq
    Integer, u64 {u64 u32 u16 u8}
    flint_sys::fmpz::fmpz_equal_ui
}

impl_cmp_unsafe! {
    ord
    Integer, u64 {u64 u32 u16 u8}
    flint_sys::fmpz::fmpz_cmp_ui
}

impl_cmp_unsafe! {
    eq
    Integer, i64 {i64 i32 i16 i8}
    flint_sys::fmpz::fmpz_equal_si
}

impl_cmp_unsafe! {
    ord
    Integer, i64 {i64 i32 i16 i8}
    flint_sys::fmpz::fmpz_cmp_si
}

impl_unop_unsafe! {
    Integer
    Neg {neg}
    NegAssign {neg_assign}
    flint_sys::fmpz::fmpz_neg
}

impl_unop_unsafe! {
    Integer
    Not {not}
    NotAssign {not_assign}
    flint_sys::fmpz::fmpz_complement
}

impl_binop_unsafe! {
    Integer, Integer, Integer
    
    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    flint_sys::fmpz::fmpz_add;
    
    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    flint_sys::fmpz::fmpz_sub;
    
    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    flint_sys::fmpz::fmpz_mul;
    
    Rem {rem}
    RemAssign {rem_assign}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    flint_sys::fmpz::fmpz_tdiv_r;
}


impl_binop_unsafe! {
    op_assign
    Integer, u64 {u64 u32 u16 u8}, Integer
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    flint_sys::fmpz::fmpz_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    flint_sys::fmpz::fmpz_sub_ui;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpz::fmpz_mul_ui;
    
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_tdiv_r_ui;
}

impl_binop_unsafe! {
    op_assign
    Integer, i64 {i64 i32 i16 i8}, Integer
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    flint_sys::fmpz::fmpz_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    flint_sys::fmpz::fmpz_sub_si;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpz::fmpz_mul_si;
    
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_tdiv_r_si;
}

impl_binop_unsafe! {
    op_from
    u64 {u64 u32 u16 u8}, Integer, Integer
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_ui_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_ui_mul;
    
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_ui_tdiv_r;
}

impl_binop_unsafe! {
    op_from
    i64 {i64 i32 i16 i8}, Integer, Integer
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_si_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_si_mul;
    
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_si_tdiv_r;
}

#[inline]
unsafe fn fmpz_ui_add(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_ulong,
    f: *const flint_sys::fmpz::fmpz) 
{
    flint_sys::fmpz::fmpz_add_ui(res, f, x);
}

#[inline]
unsafe fn fmpz_si_add(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_long,
    f: *const flint_sys::fmpz::fmpz) 
{
    flint_sys::fmpz::fmpz_add_si(res, f, x);
}

#[inline]
unsafe fn fmpz_ui_sub(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_ulong,
    f: *const flint_sys::fmpz::fmpz) 
{
    flint_sys::fmpz::fmpz_sub_ui(res, f, x);
    flint_sys::fmpz::fmpz_neg(res, res);
}

#[inline]
unsafe fn fmpz_si_sub(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_long,
    f: *const flint_sys::fmpz::fmpz) 
{
    flint_sys::fmpz::fmpz_sub_si(res, f, x);
    flint_sys::fmpz::fmpz_neg(res, res);
}

#[inline]
unsafe fn fmpz_ui_mul(
    res: *mut flint_sys::fmpz::fmpz,
    f: c_ulong,
    g: *const flint_sys::fmpz::fmpz,
    )
{
    flint_sys::fmpz::fmpz_mul_ui(res, g, f);
}

#[inline]
unsafe fn fmpz_si_mul(
    res: *mut flint_sys::fmpz::fmpz,
    f: c_long,
    g: *const flint_sys::fmpz::fmpz,
    )
{
    flint_sys::fmpz::fmpz_mul_si(res, g, f);
}

#[inline]
unsafe fn fmpz_tdiv_r_ui(
    f: *mut flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpz::fmpz,
    h: c_ulong) 
{
    let r = flint_sys::fmpz::fmpz_tdiv_ui(g, h);
    flint_sys::fmpz::fmpz_set_ui(f, r);
}

#[inline]
unsafe fn fmpz_tdiv_r_si(
    f: *mut flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpz::fmpz,
    h: c_long) 
{
    let r = flint_sys::fmpz::fmpz_tdiv_ui(g, h as u64);
    flint_sys::fmpz::fmpz_set_ui(f, r);
}

#[inline]
unsafe fn fmpz_ui_tdiv_r(
    f: *mut flint_sys::fmpz::fmpz,
    x: c_ulong,
    h: *const flint_sys::fmpz::fmpz) 
{
    let mut tmp = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(tmp.as_mut_ptr(), x);
    flint_sys::fmpz::fmpz_tdiv_r(f, tmp.as_ptr(), h);
}

#[inline]
unsafe fn fmpz_si_tdiv_r(
    f: *mut flint_sys::fmpz::fmpz,
    x: c_long,
    h: *const flint_sys::fmpz::fmpz) 
{
    let mut tmp = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_si(tmp.as_mut_ptr(), x);
    flint_sys::fmpz::fmpz_tdiv_r(f, tmp.as_ptr(), h);
}
