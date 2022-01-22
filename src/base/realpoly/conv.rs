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

use crate::*;

macro_rules! impl_from_ui {
    (
        $($t:ident)*
    ) => ($(
        impl From<&$t> for RealPoly {
            #[inline]
            fn from(src: &$t) -> RealPoly {
                RealPoly::from(IntPoly::from(src))
            }
        }

        impl From<$t> for RealPoly {
            #[inline]
            fn from(src: $t) -> RealPoly {
                RealPoly::from(&src)
            }
        }
    )*)
}

impl_from_ui! {
    usize u64 u32 u16 u8
}

impl_from_unsafe! {
    None
    RealPoly, i64 {isize i64 i32 i16 i8} 
    arb_sys::arb_poly::arb_poly_set_si
}

impl_from! {
    RealPoly, Integer
    {
        fn from(x: &Integer) -> RealPoly {
            RealPoly::from(IntPoly::from(x))
        }
    }
}

impl_from! {
    RealPoly, IntMod
    {
        fn from(x: &IntMod) -> RealPoly {
            RealPoly::from(IntPoly::from(x))
        }
    }
}

impl_from! {
    RealPoly, Rational
    {
        fn from(x: &Rational) -> RealPoly {
            RealPoly::from(RatPoly::from(x))
        }
    }
}

impl_from_unsafe! {
    None
    RealPoly, IntPoly, ARB_DEFAULT_PREC;
    arb_sys::arb_poly::arb_poly_set_fmpz_poly
}

impl_from! {
    RealPoly, IntModPoly
    {
        fn from(x: &IntModPoly) -> RealPoly {
            RealPoly::from(IntPoly::from(x))
        }
    }
}

impl_from_unsafe! {
    None
    RealPoly, RatPoly, ARB_DEFAULT_PREC;
    arb_sys::arb_poly::arb_poly_set_fmpq_poly
}

impl_from! {
    RealPoly, NumFldElem
    {
        fn from(x: &NumFldElem) -> RealPoly {
            RealPoly::from(RatPoly::from(x))
        }
    }
}

impl_from! {
    String, RealPoly
    {
        fn from(x: &RealPoly) -> String {
            x.get_str_pretty()
        }
    }
}


impl_from! {
    pol
    RealPoly, Real {u64 u32 u16 u8 i64 i32 i16 i8 Integer IntMod Rational PadicElem}
}

impl From<&[Real]> for RealPoly {
    #[inline]
    fn from(src: &[Real]) -> RealPoly {
        let mut res = RealPoly::default();
        for (i, x) in src.iter().enumerate() {
            res.set_coeff(i, x);
        }
        res
    }
}

impl From<Vec<Real>> for RealPoly {
    #[inline]
    fn from(src: Vec<Real>) -> RealPoly {
        RealPoly::from(src.as_slice())
    }
}
