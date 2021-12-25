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

/// Macros for bulk implementing traits.

// Helper macros //

/*
macro_rules! op_guard {
    (Div, $rhs:expr) => {
        if !$rhs.is_invertible() {
            panic!("Invalid divisor (zero or not invertible).")
        }
    };
    (Rem, $rhs:expr) => {
        // TODO: check if negative (doesn't make sense for polynomials)
        if $rhs.is_zero() {
            panic!("Invalid modulus (zero or negative).")
        }
    };
    ($op:ident, $rhs:expr) => {}
}*/

macro_rules! default {
    // From/conversion
    (From, matrix, $out_ty:ident, $in:ident) => {
        $out_ty::zero($in.nrows(), $in.ncols())
    };

    // Unary ops
    (Neg, matrix, $out_ty:ident, $in:ident) => {
        $out_ty::zero($in.nrows(), $in.ncols())
    };
    (Neg, matrix_mod, $out_ty:ident, $in:ident) => {
        $out_ty::zero($in.nrows(), $in.ncols(), &$in.modulus())
    };
    ($op:ident, $kw:ident, $out_ty:ident, $in:ident) => {
        $out_ty::default()
    };

    // Binary ops
    (Add, matrix, $out_ty:ident, $lhs:ident, $rhs:ident) => {
        $out_ty::zero($lhs.nrows(), $lhs.ncols())
    };
    (Sub, matrix, $out_ty:ident, $lhs:ident, $rhs:ident) => {
        $out_ty::zero($lhs.nrows(), $lhs.ncols())
    };
    (Mul, matrix, $out_ty:ident, $lhs:ident, $rhs:ident) => {
        $out_ty::zero($lhs.nrows(), $rhs.ncols())
    };
    ($op:ident, lhs_scalar, $out_ty:ident, $lhs:ident, $rhs:ident) => {
        $out_ty::zero($rhs.nrows(), $rhs.ncols())
    };
    ($op:ident, rhs_scalar, $out_ty:ident, $lhs:ident, $rhs:ident) => {
        $out_ty::zero($lhs.nrows(), $lhs.ncols())
    };
    ($op:ident, $kw:ident, $out_ty:ident, $lhs:ident, $rhs:ident) => {
        $out_ty::default()
    };
}

/*
macro_rules! op_guard {
    (Add, matrix, $lhs: expr, $rhs: expr) => {
        if ($lhs.ncols() != $rhs.ncols()) || ($lhs.nrows() != $rhs.nrows()) {
            panic!("Incompatible matrix dimensions.")
    	}
    };
	(Sub, matrix, $lhs: expr, $rhs: expr) => {
        if ($lhs.ncols() != $rhs.ncols()) || ($lhs.nrows() != $rhs.nrows()) {
            panic!("Incompatible matrix dimensions.")
        }
    };
    (Mul, matrix, $lhs: expr, $rhs: expr) => {
        if $lhs.ncols() != $rhs.nrows() {
            panic!("Incompatible matrix dimensions.")
    	}
    };
    (Div, $arg_kw: ident, $lhs: expr, $rhs: expr) => {
        if !$rhs.is_invertible() {
            panic!("Divisor must be nonzero and invertible.")
    	}
    };
	(Rem, $arg_kw: ident, $lhs: expr, $rhs: expr) => {
        if $rhs.is_zero() {
            panic!("Modulus must be nonzero (and positive if applicable).")
        }
    };
    ($op: ident, ctx, $lhs: expr, $rhs: expr) => {
        if $lhs.ctx() != $rhs.ctx() {
            panic!("Incompatible contexts.")
        }
    };
    ($op: ident, $arg_kw: ident, $lhs: expr, $rhs: expr) => {}
}
*/


/// Macros for overloading comparison operators
macro_rules! impl_cmp {
    // a = a
    (
        eq
        $t:ident
        {
            $($code:tt)*
        }
    ) => {
        impl Eq for $t {}
        
        impl Eq for &$t {}

        impl PartialEq for $t {
            #[inline]
            $($code)*
        }

        impl PartialEq<&$t> for $t {
            #[inline]
            fn eq(&self, rhs: &&$t) -> bool {
                rhs.eq(&self)
            }
        }
        
        impl PartialEq<$t> for &$t {
            #[inline]
            fn eq(&self, rhs: &$t) -> bool {
                self.eq(&rhs)
            }
        }
    };
    // a > a
    (
        ord
        $t:ident
        {
            $($code:tt)*
        }
    ) => {
        impl Ord for $t {
            #[inline]
            $($code)*
        }
        
        impl PartialOrd for $t {
            #[inline]
            fn partial_cmp(&self, rhs: &$t) -> Option<Ordering> {
                Some(self.cmp(rhs))
            }
        }
    };
    // a = b
    (
        eq
        $t1:ident, $t2:ident
        {
            $($code:tt)*
        }
    ) => {
        impl PartialEq<$t2> for $t1 {
            #[inline]
            $($code)*
        }

        impl PartialEq<&$t2> for $t1 {
            #[inline]
            fn eq(&self, rhs: &&$t2) -> bool {
                (&self).eq(rhs)
            }
        }
        
        impl PartialEq<$t2> for &$t1 {
            #[inline]
            fn eq(&self, rhs: &$t2) -> bool {
                self.eq(&rhs)
            }
        }
    };
    // a > b
    (
        ord
        $t1:ident, $t2:ident
        {
            $($code:tt)*
        }
    ) => {
        impl PartialOrd<$t2> for $t1 {
            #[inline]
            $($code)*
        }
    };

}

/// Macros for overloading comparison operators with unsafe functions.
macro_rules! impl_cmp_unsafe {
    (
        eq
        $t:ident
        $func:path
    ) => {
        impl_cmp! {
            eq
            $t
            {
                fn eq(&self, rhs: &$t) -> bool {
                    unsafe { $func(self.as_ptr(), rhs.as_ptr()) != 0 }
                }
            }
        }
    };
    (
        eq
        $t1:ident, $t2:ident
        $func:path
    ) => {
        impl_cmp! {
            eq
            $t1, $t2
            {
                fn eq(&self, rhs: &$t2) -> bool {
                    unsafe { $func(self.as_ptr(), rhs.as_ptr()) != 0 }
                }
            }
        }
        impl_cmp! {
            eq
            $t2, $t1
            {
                fn eq(&self, rhs: &$t1) -> bool {
                    unsafe { $func(rhs.as_ptr(), self.as_ptr()) != 0 }
                }
            }
        }
    };
    (
        eq
        $t1:ident, $cast:ident {$($t2:ident)+}
        $func:path
    ) => ($(
        impl_cmp! {
            eq
            $t1, $t2
            {
                fn eq(&self, rhs: &$t2) -> bool {
                    unsafe { $func(self.as_ptr(), *rhs as $cast) != 0 }
                }
            }
        }
        impl_cmp! {
            eq
            $t2, $t1
            {
                fn eq(&self, rhs: &$t1) -> bool {
                    unsafe { $func(rhs.as_ptr(), *self as $cast) != 0 }
                }
            }
        }
    )+);
    (
        ord
        $t:ident
        $func:path
    ) => {
        impl_cmp! {
            ord
            $t
            {
                fn cmp(&self, rhs: &$t) -> Ordering {
                    let cmp = unsafe { $func(self.as_ptr(), rhs.as_ptr()) };
                    if cmp == 0 {
                        Equal
                    } else if cmp < 0 {
                        Less
                    } else {
                        Greater
                    }
                }
            }
        }
    };
    (
        ord
        $t1:ident, $t2:ident
        $func:path
    ) => {
        impl_cmp! {
            ord
            $t1, $t2
            {
                fn partial_cmp(&self, rhs: &$t2) -> Option<Ordering> {
                    let cmp = unsafe { $func(self.as_ptr(), rhs.as_ptr()) };
                    if cmp == 0 {
                        Some(Equal)
                    } else if cmp < 0 {
                        Some(Less)
                    } else {
                        Some(Greater)
                    }
                }
            }
        }
        impl_cmp! {
            ord
            $t2, $t1
            {
                fn partial_cmp(&self, rhs: &$t1) -> Option<Ordering> {
                    let cmp = unsafe { $func(rhs.as_ptr(), self.as_ptr()) };
                    if cmp == 0 {
                        Some(Equal)
                    } else if cmp > 0 {
                        Some(Less)
                    } else {
                        Some(Greater)
                    }
                }
            }
        }
    };
    (
        ord
        $t1:ident, $cast:ident {$($t2:ident)+}
        $func:path
    ) => ($(
        impl_cmp! {
            ord
            $t1, $t2
            {
                fn partial_cmp(&self, rhs: &$t2) -> Option<Ordering> {
                    let cmp = unsafe { $func(self.as_ptr(), *rhs as $cast) };
                    if cmp == 0 {
                        Some(Equal)
                    } else if cmp < 0 {
                        Some(Less)
                    } else {
                        Some(Greater)
                    }
                }
            }
        }
        impl_cmp! {
            ord
            $t2, $t1
            {
                fn partial_cmp(&self, rhs: &$t1) -> Option<Ordering> {
                    let cmp = unsafe { $func(rhs.as_ptr(), *self as $cast) };
                    if cmp == 0 {
                        Some(Equal)
                    } else if cmp > 0 {
                        Some(Less)
                    } else {
                        Some(Greater)
                    }
                }
            }
        }
    )+)
}


/// Macros for overloading unary operators.
macro_rules! impl_unop {
    (
        // assign
        $t:ident
        $op:ident {$meth:ident}
        {
            $($code:tt)*
        }
        $op_assign:ident {$meth_assign:ident}
        {
            $($code_assign:tt)*
        }
    ) => {
        impl $op for $t {
            type Output = $t;
            #[inline]
            fn $meth(mut self) -> $t {
                self.$meth_assign();
                self
            }
        }
        
        impl $op for &$t {
            type Output = $t;
            #[inline]
            $($code)*
        }

        impl $op_assign for $t {
            #[inline]
            $($code_assign)*
        }
    };
    (
        // no assign
        $t:ident, $out:ident
        $op:ident {$meth:ident}
        {
            $($code:tt)*
        }
    ) => {
        impl $op for $t {
            type Output = $out;
            #[inline]
            $($code)*
        }
        
        impl $op for &$t {
            type Output = $out;
            #[inline]
            $($code)*
        }
    };
}

/// Macros for overloading unary operators with unsafe functions.
macro_rules! impl_unop_unsafe {
    (
        $kw:ident
        $t:ident
        $op:ident {$meth:ident}
        $op_assign:ident {$meth_assign:ident}
        $func:path
    ) => {

        impl_unop! {
            $t
            $op {$meth}
            {
                fn $meth(self) -> $t {
                    let mut res = default!($op, $kw, $t, self);
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr()); }
                    res
                }
            }
            $op_assign {$meth_assign}
            {
                fn $meth_assign(&mut self) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr()); }
                }
            }
        }
    };
    (
        $kw:ident
        $t:ident, $out:ident
        $op:ident {$meth:ident}
        $func:path
    ) => {

        impl_unop! {
            $t, $out
            $op {$meth}
            {
                fn $meth(self) -> $out {
                    let mut res = default!($op, $kw, $out, self);
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr()); }
                    res
                }
            }
        }
    }
}

/// Macros for overloading binary operators.
macro_rules! impl_binop {
    (
        // a + a = a
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            {
                $($code:tt)*
            }
            $op_assign:ident {$meth_assign:ident}
            {
                $($code_assign:tt)*
            }
            $op_from:ident {$meth_from:ident}
            {
                $($code_from:tt)*
            }
            $assign_op:ident {$assign_meth:ident}
            {
                $($assign_code:tt)*
            }
        )*
    ) => ($( 

        impl $op<&$t2> for &$t1 {
            type Output = $out;
            #[inline]
            $($code)*
        }

        impl $op<$t2> for &$t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, mut rhs: $t2) -> $out {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl $op<&$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(mut self, rhs: &$t2) -> $out {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl $op<$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(mut self, rhs: $t2) -> $out {
                self.$meth_assign(&rhs);
                self
            }
        }
       
        impl_binop! {@op_assign
            $t1, $t2, $out
            $op_assign {$meth_assign}
            {
                $($code_assign)*
            }
        }
        
        impl_binop! {@op_from
            $t1, $t2, $out
            $op_from {$meth_from}
            {
                $($code_from)*
            }
        }
       
        impl_binop! {@assign_op
            $t1, $t2, $out
            $assign_op {$assign_meth}
            {
                $($assign_code)*
            }
        }
    )*);
    (
        // a + b = a
        op_assign
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            {
                $($code:tt)*
            }
            $op_assign:ident {$meth_assign:ident}
            {
                $($code_assign:tt)*
            }
            $assign_op:ident {$assign_meth:ident}
            {
                $($assign_code:tt)*
            }
        )*
    ) => ($( 
        
        impl $op<&$t2> for &$t1 {
            type Output = $out;
            #[inline]
            $($code)*
        }

        impl $op<$t2> for &$t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, rhs: $t2) -> $out {
                self.$meth(&rhs)
            }
        }
        
        impl $op<&$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(mut self, rhs: &$t2) -> $out {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl $op<$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(mut self, rhs: $t2) -> $out {
                self.$meth_assign(&rhs);
                self
            }
        }
        
        impl_binop! {@op_assign
            $t1, $t2, $out
            $op_assign {$meth_assign}
            {
                $($code_assign)*
            }
        }
        
        impl_binop! {@assign_op
            $t1, $t2, $out
            $assign_op {$assign_meth}
            {
                $($assign_code)*
            }
        }
    )*);
    (
        // a + b = b
        op_from
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            {
                $($code:tt)*
            }
            $op_from:ident {$meth_from:ident}
            {
                $($code_from:tt)*
            }
            $assign_op:ident {$assign_meth:ident}
            {
                $($assign_code:tt)*
            }
        )*
    ) => ($( 

        impl $op<&$t2> for &$t1 {
            type Output = $out;
            #[inline]
            $($code)*
        }

        impl $op<$t2> for &$t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, mut rhs: $t2) -> $out {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl $op<&$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, rhs: &$t2) -> $out {
                (&self).$meth(rhs)
            }
        }
        
        impl $op<$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, mut rhs: $t2) -> $out {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl_binop! {@op_from
            $t1, $t2, $out
            $op_from {$meth_from}
            {
                $($code_from)*
            }
        }
        impl_binop! {@assign_op
            $t1, $t2, $out
            $assign_op {$assign_meth}
            {
                $($assign_code)*
            }
        }
    )*);
    (
        // a + b = c
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            {
                $($code:tt)*
            }
            $assign_op:ident {$assign_meth:ident}
            {
                $($assign_code:tt)*
            }
        )*
    ) => ($( 
        impl $op<&$t2> for &$t1 {
            type Output = $out;
            #[inline]
            $($code)*
        }
        
        impl $op<$t2> for &$t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, rhs: $t2) -> $out {
                self.$meth(&rhs)
            }
        }
        
        impl $op<&$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, rhs: &$t2) -> $out {
                (&self).$meth(rhs)
            }
        }
        
        impl $op<$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, rhs: $t2) -> $out {
                (&self).$meth(&rhs)
            }
        }
        
        impl_binop! {@assign_op
            $t1, $t2, $out
            $assign_op {$assign_meth}
            {
                $($assign_code)*
            }
        }
    )*);
    (
        @op_assign
        $t1:ident, $t2:ident, $out:ident
        $op_assign:ident {$meth_assign:ident}
        {
            $($code_assign:tt)*
        }
    ) => {
        impl $op_assign<&$t2> for $t1 {
            #[inline]
            $($code_assign)*
        }

        impl $op_assign<$t2> for $t1 {
            #[inline]
            fn $meth_assign(&mut self, rhs: $t2) {
                self.$meth_assign(&rhs);
            }
        }
    };
    (
        @op_from
        $t1:ident, $t2:ident, $out:ident
        $op_from:ident {$meth_from:ident}
        {
            $($code_from:tt)*
        }
    ) => {
        impl $op_from<&$t1> for $t2 {
            #[inline]
            $($code_from)*
        }

        impl $op_from<$t1> for $t2 {
            #[inline]
            fn $meth_from(&mut self, lhs: $t1) {
                self.$meth_from(&lhs);
            }
        }
    };
    (
        @assign_op
        $t1:ident, $t2:ident, $out:ident
        $assign_op:ident {$assign_meth:ident}
        {
            $($assign_code:tt)*
        }
    ) => {
        impl $assign_op<&$t1, &$t2> for $out {
            #[inline]
            $($assign_code)*
        }
        
        impl $assign_op<$t1, &$t2> for $out {
            #[inline]
            fn $assign_meth(&mut self, lhs: $t1, rhs: &$t2) {
                self.$assign_meth(&lhs, rhs);
            }
        }
        
        impl $assign_op<&$t1, $t2> for $out {
            #[inline]
            fn $assign_meth(&mut self, lhs: &$t1, rhs: $t2) {
                self.$assign_meth(lhs, &rhs);
            }
        }
        
        impl $assign_op<$t1, $t2> for $out {
            #[inline]
            fn $assign_meth(&mut self, lhs: $t1, rhs: $t2) {
                self.$assign_meth(&lhs, &rhs);
            }
        }
    };
}

/// Macros for overloading binary operators with unsafe functions.
macro_rules! impl_binop_unsafe {
    (
        // a + a = a
        $kw:ident
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_assign:ident {$meth_assign:ident}
            $op_from:ident {$meth_from:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )+
    ) => ($(
        impl_binop! {
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = default!($op, $kw, $out, self, rhs);
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                    res
                }
            }
            $op_assign {$meth_assign}
            {
                fn $meth_assign(&mut self, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                }
            }
            $op_from {$meth_from}
            {
                fn $meth_from(&mut self, lhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), self.as_ptr()); }
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), rhs.as_ptr()); }
                }
            }
        }
    )+);
    (
        // a + b = a
        $kw:ident
        op_assign
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_assign:ident {$meth_assign:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )+
    ) => ($(
        impl_binop! {
            op_assign
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = default!($op, $kw, $out, self, rhs);
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                    res
                }
            }
            $op_assign {$meth_assign}
            {
                fn $meth_assign(&mut self, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), rhs.as_ptr()); }
                }
            }
        }
    )+);
    (
        // a + b = a, b primitive
        $kw:ident
        op_assign    
        $t1:ident, $cast:ty {$($t2:ident)+}, $out:ident
        
        $op:ident {$meth:ident}
        $op_assign:ident {$meth_assign:ident}
        $assign_op:ident {$assign_meth:ident}
        $func:path;

        $($next:tt)*
    ) => ($(
        impl_binop_unsafe! {@inner
            $kw
            op_assign
            $t1, $cast {$t2}, $out
            
            $op {$meth}
            $op_assign {$meth_assign}
            $assign_op {$assign_meth}
            $func;
        })+

        impl_binop_unsafe! {
            $kw
            op_assign
            $t1, $cast {$($t2)+}, $out
            $($next)*
        }
    );
    (@inner
        $kw:ident
        op_assign
        $t1:ident, $cast:ty {$t2:ident}, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_assign:ident {$meth_assign:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )*
    ) => ($(
        impl_binop! {
            op_assign
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = default!($op, $kw, $out, self, rhs);
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), *rhs as $cast); }
                    res
                }
            }
            $op_assign {$meth_assign}
            {
                fn $meth_assign(&mut self, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), *rhs as $cast); }
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), *rhs as $cast); }
                }
            }
        }
    )*);
    (
        $kw:ident
        op_assign
        $t1:ident, $cast:ty {$($t2:ident)+}, $out:ident
    ) => {};
    (
        // a + b = b
        $kw:ident
        op_from
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_from:ident {$meth_from:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )+
    ) => ($(
        impl_binop! {
            op_from
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = default!($op, $kw, $out, self, rhs);
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                    res
                }
            }
            $op_from {$meth_from}
            {
                fn $meth_from(&mut self, lhs: &$t1) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), self.as_ptr()); }
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), rhs.as_ptr()); }
                }
            }
        }
    )+);
    (
        // a + b = b, a primitive
        $kw:ident
        op_from
        $cast:ty {$($t1:ident)+}, $t2:ident, $out:ident
        
        $op:ident {$meth:ident}
        $op_from:ident {$meth_from:ident}
        $assign_op:ident {$assign_meth:ident}
        $func:path;

        $($next:tt)*
    ) => ($(
        impl_binop_unsafe! {@inner
            $kw
            op_from
            $cast {$t1}, $t2, $out
            
            $op {$meth}
            $op_from {$meth_from}
            $assign_op {$assign_meth}
            $func;
        })+

        impl_binop_unsafe! {
            $kw
            op_from
            $cast {$($t1)+}, $t2, $out
            $($next)*
        }
    );
    (@inner
        $kw:ident
        op_from
        $cast:ty {$t1:ident}, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_from:ident {$meth_from:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )*
    ) => ($(
        impl_binop! {
            op_from
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = default!($op, $kw, $out, self, rhs);
                    unsafe { $func(res.as_mut_ptr(), *self as $cast, rhs.as_ptr()); }
                    res
                }
            }
            $op_from {$meth_from}
            {
                fn $meth_from(&mut self, lhs: &$t1) {
                    unsafe { $func(self.as_mut_ptr(), *lhs as $cast, self.as_ptr()); }
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), *lhs as $cast, rhs.as_ptr()); }
                }
            }
        }
    )*);
    (
        $kw:ident
        op_from
        $cast:ty {$($t1:ident)+}, $t2:ident, $out:ident
    ) => {};
    (
        // a + b = c
        $kw:ident
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )+
    ) => ($(
        impl_binop! {
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = default!($op, $kw, $out, self, rhs);
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                    res
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), rhs.as_ptr()); }
                }
            }
        }
    )+);
    (
        // a + b = c, a primitive
        $kw:ident
        $cast:ty {$($t1:ident)+}, $t2:ident, $out:ident
        
        $op:ident {$meth:ident}
        $assign_op:ident {$assign_meth:ident}
        $func:path;

        $($next:tt)*
    ) => ($(
        impl_binop_unsafe! {@inner
            $kw
            $cast {$t1}, $t2, $out
            
            $op {$meth}
            $assign_op {$assign_meth}
            $func;
        })+

        impl_binop_unsafe! {
            $kw
            $cast {$($t1)+}, $t2, $out
            $($next)*
        }
    );
    (@inner
        $kw:ident
        $cast:ty {$t1:ident}, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )*
    ) => ($(
        impl_binop! {
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = default!($op, $kw, $out, self, rhs);
                    unsafe { $func(res.as_mut_ptr(), *self as $cast, rhs.as_ptr()); }
                    res
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), *lhs as $cast, rhs.as_ptr()); }
                }
            }
        }
    )*);
    (
        $kw:ident
        $cast:ty {$($t1:ident)+}, $t2:ident, $out:ident
    ) => {};
    (
        // a + b = c, b primitive
        $kw:ident
        $t1:ident, $cast:ty {$($t2:ident)+}, $out:ident
        
        $op:ident {$meth:ident}
        $assign_op:ident {$assign_meth:ident}
        $func:path;

        $($next:tt)*
    ) => ($(
        impl_binop_unsafe! {@inner
            $kw
            $t1, $cast {$t2}, $out
            
            $op {$meth}
            $assign_op {$assign_meth}
            $func;
        })+

        impl_binop_unsafe! {
            $kw
            $t1, $cast {$($t2)+}, $out
            $($next)*
        }
    );
    (@inner
        $kw:ident
        $t1:ident, $cast:ty {$t2:ident}, $out:ident
        $(
            $op:ident {$meth:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )*
    ) => ($(
        impl_binop! {
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = default!($op, $kw, $out, self, rhs);
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), *rhs as $cast); }
                    res
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), *rhs as $cast); }
                }
            }
        }
    )*);
    (
        $kw:ident
        $t1:ident, $cast:ty {$($t2:ident)+}, $out:ident
    ) => {};
}

/// Macros for implementing `From` for conversions.
macro_rules! impl_from {
    (
        $t1:ident, $t2:ident
        {
            $($code:tt)*
        }
    ) => {
        impl From<$t2> for $t1 {
            #[inline]
            fn from(src: $t2) -> $t1 {
                <$t1>::from(&src)
            }
        }

        impl From<&$t2> for $t1 {
            #[inline]
            $($code)*
        }
    };
    (
        pol
        $t1:ident, $cast:ident {$($t2:ident)*}
    ) => ($(
        impl From<&[$t2]> for $t1 {
            #[inline]
            fn from(src: &[$t2]) -> $t1 {
                let mut res = <$t1>::default();
                for (i, x) in src.iter().enumerate() {
                    res.set_coeff(i, &<$cast>::from(x));
                }
                res
            }
        }
        
        impl From<Vec<$t2>> for $t1 {
            #[inline]
            fn from(src: Vec<$t2>) -> $t1 {
                <$t1>::from(src.as_slice())
            }
        }
    )*);
    (
        matrix
        $t1:ident, $cast:ident {$($t2:ident)*}
    ) => ($(
        
        impl From<&[&[$t2]]> for $t1 {
            fn from(mat: &[&[$t2]]) -> $t1 {
                let m = mat.len() as c_long;
                let n = mat.iter().map(|x| x.len()).max().unwrap() as c_long;
                let mut res = <$t1>::zero(m, n);

                for (i, row) in mat.iter().enumerate() {
                    for (j, x) in row.iter().enumerate() {
                        res.set_entry(i, j, &<$cast>::from(x));
                    }
                }
                res
            }
        }
        
        impl From<&[Vec<$t2>]> for $t1 {
            fn from(mat: &[Vec<$t2>]) -> $t1 {
                let m = mat.len() as c_long;
                let n = mat.iter().map(|x| x.len()).max().unwrap() as c_long;
                let mut res = <$t1>::zero(m, n);

                for (i, row) in mat.iter().enumerate() {
                    for (j, x) in row.iter().enumerate() {
                        res.set_entry(i, j, &<$cast>::from(x));
                    }
                }
                res
            }
        }
        
        impl From<Vec<&[$t2]>> for $t1 {
            #[inline]
            fn from(mat: Vec<&[$t2]>) -> $t1 {
                <$t1>::from(mat.as_slice())
            }
        }
       
        impl From<Vec<Vec<$t2>>> for $t1 {
            #[inline]
            fn from(mat: Vec<Vec<$t2>>) -> $t1 {
                <$t1>::from(mat.as_slice())
            }
        }
    )*);
}

/// Macros for implementing `From` for conversions with unsafe functions.
macro_rules! impl_from_unsafe {
    (
        // a -> b with context
        ctx
        $t1:ident, $t2:ident
        $func:path
    ) => (
        impl_from! {
            $t1, $t2
            {
                fn from(src: &$t2) -> $t1 {
                    let mut res = default!(From, ctx, $t1, src);
                    unsafe { $func(res.as_mut_ptr(), src.as_ptr(), src.ctx_as_ptr()); }
                    res
                }
            }
        }
    );
    (
        // a -> b
        $kw:ident
        $t1:ident, $t2:ident
        $func:path
    ) => (
        impl_from! {
            $t1, $t2
            {
                fn from(src: &$t2) -> $t1 {
                    let mut res = default!(From, $kw, $t1, src);
                    unsafe { $func(res.as_mut_ptr(), src.as_ptr()); }
                    res
                }
            }
        }
    );
    (
        // a -> b, with third argument (precision, etc)
        $kw:ident
        $t1:ident, $t2:ident, $arg:expr;
        $func:path
    ) => (
        impl_from! {
            $t1, $t2
            {
                fn from(src: &$t2) -> $t1 {
                    let mut res = default!(From, $kw, $t1, src);
                    unsafe { $func(res.as_mut_ptr(), src.as_ptr(), $arg); }
                    res
                }
            }
        }
    );
    (
        // a -> b, a primitive
        $kw:ident
        $t1:ident, $cast:ident {$($t2:ident)*}
        $func:path
    ) => ($(
        impl_from! {
            $t1, $t2
            {
                fn from(src: &$t2) -> $t1 {
                    let mut res = default!(From, $kw, $t1, src);
                    unsafe { $func(res.as_mut_ptr(), *src as $cast); }
                    res
                }
            }
        }

    )*)
}


macro_rules! impl_new_unsafe {
    (
        // new from primitive type
        $t1:ident, $cast:ident {$($t2:ident)*}
        $func:path
    ) => ($(
        impl New<&$t2> for $t1 {
            #[inline]
            fn new(&self, x: &$t2) -> <$t1 as Parent>::Element {
                let mut res = self.default();
                unsafe { $func(res.as_mut_ptr(), *x as $cast); }
                res
            }
        }
        
        impl New<$t2> for $t1 {
            #[inline]
            fn new(&self, x: $t2) -> <$t1 as Parent>::Element {
                self.new(&x)
            }
        }
    )*);
    (
        // new without context
        $t1:ident, $t2:ident
        $func:path
    ) => (
        impl New<&$t2> for $t1 {
            #[inline]
            fn new(&self, x: &$t2) -> <$t1 as Parent>::Element {
                let mut res = self.default();
                unsafe { $func(res.as_mut_ptr(), x.as_ptr()); }
                res
            }
        }
        
        impl New<$t2> for $t1 {
            #[inline]
            fn new(&self, x: $t2) -> <$t1 as Parent>::Element {
                self.new(&x)
            }
        }
    );
    (
        // new from primitive with context
        ctx
        $t1:ident, $cast:ident {$($t2:ident)*}
        $func:path
    ) => ($(
        impl New<&$t2> for $t1 {
            #[inline]
            fn new(&self, x: &$t2) -> <$t1 as Parent>::Element {
                let mut res = self.default();
                unsafe { $func(res.as_mut_ptr(), *x as $cast, self.as_ptr()); }
                res
            }
        }
        
        impl New<$t2> for $t1 {
            #[inline]
            fn new(&self, x: $t2) -> <$t1 as Parent>::Element {
                self.new(&x)
            }
        }
    )*);
    (
        // new from non-primitive with context
        ctx
        $t1:ident, $t2:ident
        $func:path
    ) => (
        impl New<&$t2> for $t1 {
            #[inline]
            fn new(&self, x: &$t2) -> <$t1 as Parent>::Element {
                let mut res = self.default();
                unsafe { $func(res.as_mut_ptr(), x.as_ptr(), self.as_ptr()); }
                res
            }
        }
        
        impl New<$t2> for $t1 {
            #[inline]
            fn new(&self, x: $t2) -> <$t1 as Parent>::Element {
                self.new(&x)
            }
        }
    );
    (
        // new from non-primitive with precision
        prec
        $t1:ident, $t2:ident
        $func:path
    ) => (
        impl New<&$t2> for $t1 {
            #[inline]
            fn new(&self, x: &$t2) -> <$t1 as Parent>::Element {
                let mut res = self.default();
                unsafe { $func(res.as_mut_ptr(), x.as_ptr(), self.precision()); }
                res
            }
        }
        
        impl New<$t2> for $t1 {
            #[inline]
            fn new(&self, x: $t2) -> <$t1 as Parent>::Element {
                self.new(&x)
            }
        }
    );
    (
        // new from vectors of primitives with context
        pol
        $t1:ident, {$($t2:ident)*}
    ) => ($(
        impl New<&[$t2]> for $t1 {
            #[inline]
            fn new(&self, src: &[$t2]) -> <$t1 as Parent>::Element {
                let mut res = self.default();
                for (i, x) in src.iter().enumerate() {
                    res.set_coeff(i, &self.base_ring().new(x));
                }
                res
            }
        }
        
        impl New<Vec<$t2>> for $t1 {
            #[inline]
            fn new(&self, src: Vec<$t2>) -> <$t1 as Parent>::Element {
                self.new(src.as_slice())
            }
        }
    )*);
    (
        // new from vector of slices of primitive types with context
        matrix
        $t1:ident, {$($t2:ident)*}
    ) => ($(
        
        impl New<&[&[$t2]]> for $t1 {
            fn new(&self, mat: &[&[$t2]]) -> <$t1 as Parent>::Element {
                let m = mat.len() as c_long;
                let n = mat.iter().map(|x| x.len()).max().unwrap() as c_long;

                let mut res = self.default();
                assert!(m <= res.nrows());
                assert!(n <= res.ncols());

                for (i, row) in mat.iter().enumerate() {
                    for (j, x) in row.iter().enumerate() {
                        res.set_entry(i, j, &self.base_ring().new(x));
                    }
                }
                res
            }
        }
        
        impl New<&[Vec<$t2>]> for $t1 {
            fn new(&self, mat: &[Vec<$t2>]) -> <$t1 as Parent>::Element {
                let m = mat.len() as c_long;
                let n = mat.iter().map(|x| x.len()).max().unwrap() as c_long;

                let mut res = self.default();
                assert!(m <= res.nrows());
                assert!(n <= res.ncols());

                for (i, row) in mat.iter().enumerate() {
                    for (j, x) in row.iter().enumerate() {
                        res.set_entry(i, j, &self.base_ring().new(x));
                    }
                }
                res
            }
        }
        
        impl New<Vec<&[$t2]>> for $t1 {
            #[inline]
            fn new(&self, mat: Vec<&[$t2]>) -> <$t1 as Parent>::Element {
                self.new(mat.as_slice())
            }
        }
       
        impl New<Vec<Vec<$t2>>> for $t1 {
            #[inline]
            fn new(&self, mat: Vec<Vec<$t2>>) -> <$t1 as Parent>::Element {
                self.new(mat.as_slice())
            }
        }
    )*);
}
