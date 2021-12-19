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


use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::sync::Arc;

use antic_sys::nf_struct;
use antic_sys::nf_elem_struct;

use crate::*;


pub struct NfCtx(pub nf_struct);

impl Drop for NfCtx {
    fn drop(&mut self) {
        unsafe { antic_sys::nf_clear(&mut self.0); }
    }
}

/// A number field.
#[derive(Clone)]
pub struct NumberField {
    pub ctx: <Self as Parent>::Data,
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

impl Additive for NumberField {
    #[inline]
    fn zero(&self) -> NumFldElem {
        self.default()
    }
}

impl Multiplicative for NumberField {
    #[inline]
    fn one(&self) -> NumFldElem {
        let mut res = self.default();
        unsafe {
            antic_sys::_nf_elem_set_coeff_num_fmpz(
                res.as_mut_ptr(), 
                0,
                Integer::from(1).as_ptr(),
                self.as_ptr()
            );
        }
        res
    }
}

impl AdditiveGroup for NumberField {}

impl MultiplicativeGroup for NumberField {}

impl Ring for NumberField {}

impl Field for NumberField {
    type BaseField = RationalField;

    #[inline]
    fn base_field(&self) -> RationalField {
        RationalField {}
    }
}

impl NumberField {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// Antic via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &nf_struct {
        &self.ctx.0
    }
}

// NumFldElem //

/// A number field element.
pub type NumFldElem = Elem<NumberField>;

impl Element for NumFldElem {
    type Data = nf_elem_struct;
    type Parent = NumberField;

    #[inline]
    fn parent(&self) -> NumberField {
        NumberField { ctx: Arc::clone(&self.ctx) }
    }
}

/*
impl AdditiveElement for NumFldElem {
    #[inline]
    fn is_zero(&self) -> bool {
        self == &0
    }
}

impl MultiplicativeElement for NumFldElem {
    #[inline]
    fn is_one(&self) -> bool {
        self == &1
    }
}

impl AdditiveGroupElement for NumFldElem {}

impl MultiplicativeGroupElement for NumFldElem {}

impl RingElement for NumFldElem {}

impl FieldElement for NumFldElem {}
*/

impl NumFldElem {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// Antic via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &nf_elem_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with Antic via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut nf_elem_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with Antic via the FFI.
    #[inline]
    pub fn ctx_as_ptr(&self) -> &nf_struct {
        &self.ctx.0
    }
    
    /*
    /// Return a [String] representation of a number field element.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::fmpz_poly_q::fmpz_poly_q_get_str(self.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return a pretty-printed [String] representation of a rational function.
    #[inline]
    pub fn get_str_pretty(&self, var: &str) -> String {
        let v = CString::new(var).unwrap();
        unsafe {
            let s = flint_sys::fmpz_poly_q::fmpz_poly_q_get_str_pretty(self.as_ptr(), v.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    */
}
