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
use std::fmt;
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
    pub ctx: Arc<NfCtx>,
    pub x: Arc<String>,
}

impl Parent for NumberField {
    type Element = NumFldElem;
    type Context = ();

    #[inline]
    fn default(&self) -> NumFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            antic_sys::nf_elem_init(z.as_mut_ptr(), self.as_ptr());
            NumFldElem { 
                data: NumFldElemData {
                    ctx: Arc::clone(&self.ctx), 
                    x: Arc::clone(&self.x),
                    elem: z.assume_init() 
                }
            }
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

pub struct NumFldElemData {
    pub elem: nf_elem_struct,
    pub ctx: Arc<NfCtx>,
    pub x: Arc<String>,
}

impl Drop for NumFldElemData {
    fn drop(&mut self) {
        unsafe { antic_sys::nf_elem_clear(&mut self.elem, &self.ctx.0);}
    }
}

impl fmt::Debug for NumFldElemData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rr = RatPolyRing::init(&*self.x);
        let mut tmp = rr.default();
        unsafe {
            antic_sys::nf_elem_get_fmpq_poly(
                tmp.as_mut_ptr(),
                &self.elem,
                &self.ctx.0
            );
            f.debug_struct("NumFldElemData")
                .field("elem", &tmp.get_str_pretty())
                .field("x", &self.x)
                .field("ctx.pol", &self.ctx.0.pol)
                .field("ctx.traces", &self.ctx.0.traces)
                .field("ctx.flag", &self.ctx.0.flag)
                .finish_non_exhaustive()
        }
    }
}

impl Element for NumFldElem {
    type Data = NumFldElemData;
    type Parent = NumberField;

    #[inline]
    fn parent(&self) -> NumberField {
        NumberField { ctx: Arc::clone(&self.data.ctx), x: Arc::clone(&self.data.x) }
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
        &self.data.elem
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with Antic via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut nf_elem_struct {
        &mut self.data.elem
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with Antic via the FFI.
    #[inline]
    pub fn ctx_as_ptr(&self) -> &nf_struct {
        &self.data.ctx.0
    }
    
    /// Return a [String] representation of a number field element.
    #[inline]
    pub fn get_str(&self) -> String {
        RatPoly::from(self).get_str()
    }
    
    /// Return a pretty-printed [String] representation of a number field element.
    #[inline]
    pub fn get_str_pretty(&self) -> String {
        let rr = RatPolyRing::init(&*self.data.x);
        rr.new(self).get_str()
    }
}
