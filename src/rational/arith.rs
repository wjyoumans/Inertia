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

use libc::{c_int, c_long, c_ulong};
use rug::ops::*;

use crate::traits::*;
use crate::integer::src::Integer;
use crate::rational::src::Rational;


impl_cmp_unsafe! {
    eq
    Rational
    flint_sys::fmpq::fmpq_equal
}

impl_cmp_unsafe! {
    ord
    Rational
    flint_sys::fmpq::fmpq_cmp
}

impl_cmp_unsafe! {
    eq
    Rational, Integer
    flint_sys::fmpq::fmpq_cmp_fmpz
}

impl_cmp_unsafe! {
    ord
    Rational, Integer
    flint_sys::fmpq::fmpq_cmp_fmpz
}

impl_cmp_unsafe! {
    eq
    Integer, Rational
    fmpq_fmpz_cmp
}

impl_cmp_unsafe! {
    ord
    Integer, Rational
    fmpq_fmpz_cmp
}

impl_cmp_unsafe! {
    eq
    Rational, u64 {u64 u32 u16 u8}
    flint_sys::fmpq::fmpq_cmp_ui
}

impl_cmp_unsafe! {
    ord
    Rational, u64 {u64 u32 u16 u8}
    flint_sys::fmpq::fmpq_cmp_ui
}

impl_cmp_unsafe! {
    eq
    Rational, i64 {i64 i32 i16 i8}
    flint_sys::fmpq::fmpq_cmp_si
}

impl_cmp_unsafe! {
    ord
    Rational, i64 {i64 i32 i16 i8}
    flint_sys::fmpq::fmpq_cmp_si
}

impl_unop_unsafe! {
    Integer, Rational
    Inv {inv}
    fmpq_inv_fmpz
}

impl_unop_unsafe! {
    Rational
    Neg {neg}
    NegAssign {neg_assign}
    flint_sys::fmpq::fmpq_neg
}

impl_unop_unsafe! {
    Rational
    Inv {inv}
    InvAssign {inv_assign}
    flint_sys::fmpq::fmpq_inv
}

impl_binop_unsafe! {
    Rational, Rational, Rational
    
    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    flint_sys::fmpq::fmpq_add;
    
    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    flint_sys::fmpq::fmpq_sub;
    
    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    flint_sys::fmpq::fmpq_mul;
    
    Div {div}
    DivAssign {div_assign}
    DivFrom {div_from}
    AssignDiv {assign_div}
    flint_sys::fmpq::fmpq_div;
}

impl_binop_unsafe! {
    op_assign
    Rational, Integer, Rational
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    flint_sys::fmpq::fmpq_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    flint_sys::fmpq::fmpq_sub_fmpz;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpq::fmpq_mul_fmpz;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    flint_sys::fmpq::fmpq_div_fmpz;
}

impl_binop_unsafe! {
    Rational, Integer, Integer

    Rem {rem}
    AssignRem {assign_rem}
    flint_sys::fmpq::fmpq_mod_fmpz;
}

impl_binop_unsafe! {
    op_from
    Integer, Rational, Rational
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_fmpz_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_fmpz_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_fmpz_mul;
    
    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpq_fmpz_div;
}

impl_binop_unsafe! {
    op_assign
    Rational, u64 {u64 u32 u16 u8}, Rational
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    flint_sys::fmpq::fmpq_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    flint_sys::fmpq::fmpq_sub_ui;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpq::fmpq_mul_ui;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpq_div_ui;
}

impl_binop_unsafe! {
    Rational, u64 {u64 u32 u16 u8}, Integer

    Rem {rem}
    AssignRem {assign_rem}
    fmpq_mod_ui;
}

impl_binop_unsafe! {
    op_assign
    Rational, i64 {i64 i32 i16 i8}, Rational
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    flint_sys::fmpq::fmpq_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    flint_sys::fmpq::fmpq_sub_si;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpq::fmpq_mul_si;
    
    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpq_div_si;
}

impl_binop_unsafe! {
    Rational, i64 {i64 i32 i16 i8}, Integer

    Rem {rem}
    AssignRem {assign_rem}
    fmpq_mod_si;
}

impl_binop_unsafe! {
    op_from
    u64 {u64 u32 u16 u8}, Rational, Rational
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_ui_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_ui_mul;
    
    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpq_ui_div;
}

impl_binop_unsafe! {
    op_from
    i64 {i64 i32 i16 i8}, Rational, Rational
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpq_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpq_si_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpq_si_mul;
    
    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpq_si_div;
}


#[inline]
unsafe fn fmpq_inv_fmpz(
    res: *mut flint_sys::fmpq::fmpq,
    f: *const flint_sys::fmpz::fmpz)
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpq::fmpq_init(z.as_mut_ptr());
    flint_sys::fmpq::fmpq_set_fmpz_den1(z.as_mut_ptr(), f);
    flint_sys::fmpq::fmpq_inv(res, z.as_ptr());
}

#[inline]
unsafe fn fmpq_fmpz_cmp(
    f: *const flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpq::fmpq) -> c_int {
    -flint_sys::fmpq::fmpq_cmp_fmpz(g, f)
}

#[inline]
unsafe fn fmpq_fmpz_add(
    res: *mut flint_sys::fmpq::fmpq,
    x: *const flint_sys::fmpz::fmpz, 
    f: *const flint_sys::fmpq::fmpq)
{
    flint_sys::fmpq::fmpq_add_fmpz(res, f, x);
}

#[inline]
unsafe fn fmpq_fmpz_sub(
    res: *mut flint_sys::fmpq::fmpq,
    x: *const flint_sys::fmpz::fmpz, 
    f: *const flint_sys::fmpq::fmpq)
{
    flint_sys::fmpq::fmpq_sub_fmpz(res, f, x);
    flint_sys::fmpq::fmpq_neg(res, res);
}

#[inline]
unsafe fn fmpq_fmpz_mul(
    res: *mut flint_sys::fmpq::fmpq,
    x: *const flint_sys::fmpz::fmpz, 
    f: *const flint_sys::fmpq::fmpq)
{
    flint_sys::fmpq::fmpq_mul_fmpz(res, f, x);
}


#[inline]
unsafe fn fmpq_fmpz_div(
    res: *mut flint_sys::fmpq::fmpq,
    x: *const flint_sys::fmpz::fmpz, 
    f: *const flint_sys::fmpq::fmpq)
{
    flint_sys::fmpq::fmpq_div_fmpz(res, f, x);
    flint_sys::fmpq::fmpq_inv(res, res);
}

#[inline]
unsafe fn fmpq_ui_add(
    res: *mut flint_sys::fmpq::fmpq,
    x: c_ulong,
    f: *const flint_sys::fmpq::fmpq) 
{
    flint_sys::fmpq::fmpq_add_ui(res, f, x);
}

#[inline]
unsafe fn fmpq_si_add(
    res: *mut flint_sys::fmpq::fmpq,
    x: c_long,
    f: *const flint_sys::fmpq::fmpq) 
{
    flint_sys::fmpq::fmpq_add_si(res, f, x);
}

#[inline]
unsafe fn fmpq_ui_sub(
    res: *mut flint_sys::fmpq::fmpq,
    x: c_ulong,
    f: *const flint_sys::fmpq::fmpq) 
{
    flint_sys::fmpq::fmpq_sub_ui(res, f, x);
    flint_sys::fmpq::fmpq_neg(res, res);
}

#[inline]
unsafe fn fmpq_si_sub(
    res: *mut flint_sys::fmpq::fmpq,
    x: c_long,
    f: *const flint_sys::fmpq::fmpq) 
{
    flint_sys::fmpq::fmpq_sub_si(res, f, x);
    flint_sys::fmpq::fmpq_neg(res, res);
}

#[inline]
unsafe fn fmpq_ui_mul(
    res: *mut flint_sys::fmpq::fmpq,
    f: c_ulong,
    g: *const flint_sys::fmpq::fmpq,
    )
{
    flint_sys::fmpq::fmpq_mul_ui(res, g, f);
}

#[inline]
unsafe fn fmpq_si_mul(
    res: *mut flint_sys::fmpq::fmpq,
    f: c_long,
    g: *const flint_sys::fmpq::fmpq,
    )
{
    flint_sys::fmpq::fmpq_mul_si(res, g, f);
}

#[inline]
unsafe fn fmpq_div_ui(
    res: *mut flint_sys::fmpq::fmpq,
    f: *const flint_sys::fmpq::fmpq,
    g: c_ulong,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(z.as_mut_ptr(), g);
    flint_sys::fmpq::fmpq_div_fmpz(res, f, z.as_ptr());
}

#[inline]
unsafe fn fmpq_div_si(
    res: *mut flint_sys::fmpq::fmpq,
    f: *const flint_sys::fmpq::fmpq,
    g: c_long,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_si(z.as_mut_ptr(), g);
    flint_sys::fmpq::fmpq_div_fmpz(res, f, z.as_ptr());
}

#[inline]
unsafe fn fmpq_ui_div(
    res: *mut flint_sys::fmpq::fmpq,
    f: c_ulong,
    g: *const flint_sys::fmpq::fmpq,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(z.as_mut_ptr(), f);
    fmpq_fmpz_div(res, z.as_ptr(), g);
}

#[inline]
unsafe fn fmpq_si_div(
    res: *mut flint_sys::fmpq::fmpq,
    f: c_long,
    g: *const flint_sys::fmpq::fmpq,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_si(z.as_mut_ptr(), f);
    fmpq_fmpz_div(res, z.as_ptr(), g);
}

#[inline]
unsafe fn fmpq_mod_ui(
    res: *mut flint_sys::fmpz::fmpz,
    f: *const flint_sys::fmpq::fmpq,
    g: c_ulong,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(z.as_mut_ptr(), g);
    flint_sys::fmpq::fmpq_mod_fmpz(res, f, z.as_ptr());
}

#[inline]
unsafe fn fmpq_mod_si(
    res: *mut flint_sys::fmpz::fmpz,
    f: *const flint_sys::fmpq::fmpq,
    g: c_long,
    )
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_si(z.as_mut_ptr(), g);
    flint_sys::fmpq::fmpq_mod_fmpz(res, f, z.as_ptr());
}
