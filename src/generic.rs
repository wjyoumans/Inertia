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

// Move Poly, Mat etc. into inertia. need them to be local types.
// (Leave GenericPoly etc. in inertia-generic)
// or we can't impl Add<i32> for Poly<Integer> etc.

use inertia_generic::poly::{Poly, IntoPolyRing, InnerPoly};

use inertia_algebra::ops::*;
use crate::{Integer, Integers, IntPoly};

// Derive scalar/coefficient binops for wrapper types from inertia-generic.
macro_rules! derive_wrapper_binops {
    (poly $scalar:ident, $op:ident, $meth:ident) => {
        derive_wrapper_binops!{
            @inner 
            $scalar
            $op, $meth
            Poly, InnerPoly, IntoPolyRing
        }
    };
    (
        @inner
        $scalar:ident
        $op:ident, $meth:ident
        $wrapper:ident, $inner:ident, $intoinner:ident
    ) => {
        impl<T> $op<$scalar> for $wrapper<T>
        where
            T: $intoinner,
            $inner<T>: $op<$scalar, Output=$inner<T>>
        {
            type Output = $wrapper<T>;
            fn $meth(self, rhs: $scalar) -> Self::Output {
                Poly::from_raw(self.into_inner().$meth(rhs))
            }

        }
        
        impl<T> $op<$wrapper<T>> for $scalar
        where
            T: $intoinner,
            $scalar: $op<$inner<T>, Output=$inner<T>>
        {
            type Output = $wrapper<T>;
            fn $meth(self, rhs: $wrapper<T>) -> Self::Output {
                Poly::from_raw(self.$meth(rhs.into_inner()))
            }

        }
    }
}

derive_wrapper_binops!{poly Integer, Add, add}
derive_wrapper_binops!{poly Integer, Mul, mul}
