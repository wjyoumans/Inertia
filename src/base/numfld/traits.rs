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


//use std::hash::{Hash, Hasher};
use std::sync::Arc;


use crate::*;


impl Clone for NumFldElem {
    fn clone(&self) -> Self {
        NumFldElem { 
            data: NumFldElemData {
                ctx: Arc::clone(&self.data.ctx), 
                x: Arc::clone(&self.data.x), 
                elem: self.data.elem.clone() 
            }
        }
    }
}

/*
impl fmt::Display for RatFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}*/

/* TODO: need num/den
impl Hash for RatFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numerator().coefficients().hash(state);
        self.denominator().coefficients().hash(state);
    }
}
*/

/*
impl<T> Evaluate<T> for RatFunc where
    T: Into<Rational>
{
    type Output = Rational;
    #[inline]
    fn evaluate(&self, x: T) -> Self::Output {
        self.evaluate(&x.into())
    }
}

impl Evaluate<&Rational> for RatPol {
    type Output = Rational;
    #[inline]
    fn evaluate(&self, x: &Rational) -> Self::Output {
        let mut res = Rational::default();
        unsafe {
            flint_sys::fmpq_poly::fmpq_poly_evaluate_fmpq(
                res.as_mut_ptr(),
                self.as_ptr(),
                x.as_ptr()
            );
        }
        res
    }
}*/
