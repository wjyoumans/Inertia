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


use std::fmt;
//use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::sync::Arc;

use antic_sys::nf_struct;
use antic_sys::nf_elem_struct;

use crate::traits::*;
use crate::numfld::src::{NumFldElem, NumberField};


// NumberField //

pub struct NfCtx(pub nf_struct);

impl Drop for NfCtx {
    fn drop(&mut self) {
        unsafe { antic_sys::nf_clear(&mut self.0); }
    }
}

impl Parent for NumberField {
    type Data = Arc<NfCtx>;
    type Extra = ();
    type Element = NumFldElem;

    #[inline]
    fn default(&self) -> NumFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            antic_sys::nf_elem_init(z.as_mut_ptr(), self.as_ptr());
            NumFldElem { ctx: Arc::clone(&self.ctx), extra: (), data: z.assume_init() }
        }
    }

}

// NumFldElem //

impl Element for NumFldElem {
    type Data = nf_elem_struct;
    type Parent = NumberField;

    #[inline]
    fn parent(&self) -> NumberField {
        NumberField { ctx: Arc::clone(&self.ctx) }
    }
}

impl Clone for NumFldElem {
    fn clone(&self) -> Self {
        NumFldElem { ctx: Arc::clone(&self.ctx), extra: (), data: self.data.clone() }
    }
}

/*
impl fmt::Display for RatFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}*/

impl Drop for NumFldElem {
    fn drop(&mut self) {
        unsafe { antic_sys::nf_elem_clear(self.as_mut_ptr(), self.ctx_as_ptr());}
    }
}

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
