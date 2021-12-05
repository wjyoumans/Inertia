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
use std::ops::*;

use libc::{c_long, c_ulong};
use rug::ops::*;

use crate::*;

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
    None
    Integer
    Neg {neg}
    NegAssign {neg_assign}
    flint_sys::fmpz::fmpz_neg
}

impl_unop_unsafe! {
    None
    Integer
    Not {not}
    NotAssign {not_assign}
    flint_sys::fmpz::fmpz_complement
}

impl_unop_unsafe! {
    None
    Integer, Rational
    Inv {inv}
    fmpq_inv_fmpz
}

impl_binop_unsafe! {
    None
    Integer, Integer, Integer
    
    BitAnd {bitand}
    BitAndAssign {bitand_assign}
    BitAndFrom {bitand_from}
    AssignBitAnd {assign_bitand}
    flint_sys::fmpz::fmpz_and;
  
    BitOr {bitor}
    BitOrAssign {bitor_assign}
    BitOrFrom {bitor_from}
    AssignBitOr {assign_bitor}
    flint_sys::fmpz::fmpz_or;
    
    BitXor {bitxor}
    BitXorAssign {bitxor_assign}
    BitXorFrom {bitxor_from}
    AssignBitXor {assign_bitxor}
    flint_sys::fmpz::fmpz_xor;
    
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
    None
    Integer, Integer, Rational

    Pow {pow}
    AssignPow {assign_pow}
    fmpz_pow_fmpz;
}

impl_binop_unsafe! {
    None
    op_assign
    Integer, u64 {u64 u32 u16 u8}, Integer
    
    BitAnd {bitand}
    BitAndAssign {bitand_assign}
    AssignBitAnd {assign_bitand}
    fmpz_and_ui;
  
    BitOr {bitor}
    BitOrAssign {bitor_assign}
    AssignBitOr {assign_bitor}
    fmpz_or_ui;
    
    BitXor {bitxor}
    BitXorAssign {bitxor_assign}
    AssignBitXor {assign_bitxor}
    fmpz_xor_ui;

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
    
    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    flint_sys::fmpz::fmpz_pow_ui;
}

impl_binop_unsafe! {
    None
    op_assign
    Integer, i64 {i64 i32 i16 i8}, Integer
    
    BitAnd {bitand}
    BitAndAssign {bitand_assign}
    AssignBitAnd {assign_bitand}
    fmpz_and_si;
  
    BitOr {bitor}
    BitOrAssign {bitor_assign}
    AssignBitOr {assign_bitor}
    fmpz_or_si;
    
    BitXor {bitxor}
    BitXorAssign {bitxor_assign}
    AssignBitXor {assign_bitxor}
    fmpz_xor_si;
   
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
    None
    Integer, i64 {i64 i32 i16 i8}, Rational

    Pow {pow}
    AssignPow {assign_pow}
    fmpz_pow_si;
}

impl_binop_unsafe! {
    None
    op_from
    u64 {u64 u32 u16 u8}, Integer, Integer
    
    BitAnd {bitand}
    BitAndFrom {bitand_from}
    AssignBitAnd {assign_bitand}
    fmpz_ui_and;
  
    BitOr {bitor}
    BitOrFrom {bitor_from}
    AssignBitOr {assign_bitor}
    fmpz_ui_or;
    
    BitXor {bitxor}
    BitXorFrom {bitxor_from}
    AssignBitXor {assign_bitxor}
    fmpz_ui_xor;
   
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

/* TODO: not working?
impl_binop_unsafe! {
    None
    u64 {u64 u32 u16 i8}, Integer, Rational

    Pow {pow}
    AssignPow {assign_pow}
    fmpz_ui_pow;
}*/

impl_binop_unsafe! {
    None
    op_from
    i64 {i64 i32 i16 i8}, Integer, Integer
    
    BitAnd {bitand}
    BitAndFrom {bitand_from}
    AssignBitAnd {assign_bitand}
    fmpz_si_and;
  
    BitOr {bitor}
    BitOrFrom {bitor_from}
    AssignBitOr {assign_bitor}
    fmpz_si_or;
    
    BitXor {bitxor}
    BitXorFrom {bitxor_from}
    AssignBitXor {assign_bitxor}
    fmpz_si_xor;
   
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

/* TODO: not working?
impl_binop_unsafe! {
    None
    i64 {i64 i32 i16 i8}, Integer, Rational

    Pow {pow}
    AssignPow {assign_pow}
    fmpz_si_pow;
}
*/

#[inline]
unsafe fn fmpz_and_ui(
    res: *mut flint_sys::fmpz::fmpz,
    f: *const flint_sys::fmpz::fmpz,
    x: c_ulong)
{
    flint_sys::fmpz::fmpz_init_set_ui(res, x);
    flint_sys::fmpz::fmpz_and(res, f, res);
}

#[inline]
unsafe fn fmpz_and_si(
    res: *mut flint_sys::fmpz::fmpz,
    f: *const flint_sys::fmpz::fmpz, 
    x: c_long)
{
    flint_sys::fmpz::fmpz_init_set_si(res, x);
    flint_sys::fmpz::fmpz_and(res, f, res);
}

#[inline]
unsafe fn fmpz_ui_and(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_ulong,
    f: *const flint_sys::fmpz::fmpz) 
{
    fmpz_and_ui(res, f, x);
}

#[inline]
unsafe fn fmpz_si_and(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_long,
    f: *const flint_sys::fmpz::fmpz) 
{
    fmpz_and_si(res, f, x);
}

#[inline]
unsafe fn fmpz_or_ui(
    res: *mut flint_sys::fmpz::fmpz,
    f: *const flint_sys::fmpz::fmpz,
    x: c_ulong)
{
    flint_sys::fmpz::fmpz_init_set_ui(res, x);
    flint_sys::fmpz::fmpz_or(res, f, res);
}

#[inline]
unsafe fn fmpz_or_si(
    res: *mut flint_sys::fmpz::fmpz,
    f: *const flint_sys::fmpz::fmpz, 
    x: c_long)
{
    flint_sys::fmpz::fmpz_init_set_si(res, x);
    flint_sys::fmpz::fmpz_or(res, f, res);
}

#[inline]
unsafe fn fmpz_ui_or(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_ulong,
    f: *const flint_sys::fmpz::fmpz) 
{
    fmpz_or_ui(res, f, x);
}

#[inline]
unsafe fn fmpz_si_or(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_long,
    f: *const flint_sys::fmpz::fmpz) 
{
    fmpz_or_si(res, f, x);
}

#[inline]
unsafe fn fmpz_xor_ui(
    res: *mut flint_sys::fmpz::fmpz,
    f: *const flint_sys::fmpz::fmpz,
    x: c_ulong)
{
    flint_sys::fmpz::fmpz_init_set_ui(res, x);
    flint_sys::fmpz::fmpz_xor(res, f, res);
}

#[inline]
unsafe fn fmpz_xor_si(
    res: *mut flint_sys::fmpz::fmpz,
    f: *const flint_sys::fmpz::fmpz, 
    x: c_long)
{
    flint_sys::fmpz::fmpz_init_set_si(res, x);
    flint_sys::fmpz::fmpz_xor(res, f, res);
}

#[inline]
unsafe fn fmpz_ui_xor(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_ulong,
    f: *const flint_sys::fmpz::fmpz) 
{
    fmpz_xor_ui(res, f, x);
}

#[inline]
unsafe fn fmpz_si_xor(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_long,
    f: *const flint_sys::fmpz::fmpz) 
{
    fmpz_xor_si(res, f, x);
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
    res: *mut flint_sys::fmpz::fmpz,
    x: c_ulong,
    h: *const flint_sys::fmpz::fmpz) 
{
    flint_sys::fmpz::fmpz_init_set_ui(res, x);
    flint_sys::fmpz::fmpz_tdiv_r(res, res, h);
}

#[inline]
unsafe fn fmpz_si_tdiv_r(
    res: *mut flint_sys::fmpz::fmpz,
    x: c_long,
    h: *const flint_sys::fmpz::fmpz) 
{
    flint_sys::fmpz::fmpz_init_set_si(res, x);
    flint_sys::fmpz::fmpz_tdiv_r(res, res, h);
}

#[inline]
unsafe fn fmpq_inv_fmpz(
    res: *mut flint_sys::fmpq::fmpq,
    f: *const flint_sys::fmpz::fmpz)
{
    flint_sys::fmpq::fmpq_set_fmpz_den1(res, f);
    flint_sys::fmpq::fmpq_inv(res, res);
}

#[inline]
unsafe fn fmpz_pow_fmpz(
    res: *mut flint_sys::fmpq::fmpq,
    f: *const flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpz::fmpz,
    )
{
    flint_sys::fmpq::fmpq_set_fmpz_den1(res, f);
    flint_sys::fmpq::fmpq_pow_fmpz(res, res, g);
}

#[inline]
unsafe fn fmpz_pow_si(
    res: *mut flint_sys::fmpq::fmpq,
    f: *const flint_sys::fmpz::fmpz,
    g: c_long,
    )
{
    flint_sys::fmpq::fmpq_set_fmpz_den1(res, f);
    flint_sys::fmpq::fmpq_pow_si(res, res, g);
}

/*
#[inline]
unsafe fn fmpz_ui_pow(
    res: *mut flint_sys::fmpq::fmpq,
    f: c_ulong,
    g: *const flint_sys::fmpz::fmpz,
    )
{
    flint_sys::fmpq::fmpq_set_ui_den1(res, f);
    flint_sys::fmpq::fmpq_pow_fmpz(res, res, g);
}

#[inline]
unsafe fn fmpz_si_pow(
    res: *mut flint_sys::fmpq::fmpq,
    f: c_long,
    g: *const flint_sys::fmpz::fmpz,
    )
{
    flint_sys::fmpq::fmpq_set_si_den1(res, f);
    flint_sys::fmpq::fmpq_pow_fmpz(res, res, g);
}
*/
