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

use flint_sys::qadic::qadic_ctx_struct;
use flint_sys::qadic::qadic_struct;
use libc::c_long;

use crate::*;


#[derive(Debug)]
pub struct QadicCtx(pub qadic_ctx_struct);

impl Drop for QadicCtx {
    fn drop(&mut self) {
        unsafe { flint_sys::qadic::qadic_ctx_clear(&mut self.0); }
    }
}

/// An unramified extension of the p-adic numbers.
pub struct QadicField {
    ctx: <Self as Parent>::Data,
}

impl Parent for QadicField {
    type Data = Arc<QadicCtx>;
    type Extra = ();
    type Element = QadicElem;

    #[inline]
    fn default(&self) -> QadicElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::qadic::qadic_init(z.as_mut_ptr());
            QadicElem { 
                ctx: Arc::clone(&self.ctx), 
                extra: (), 
                data: z.assume_init() 
            }
        }
    }
}

impl Additive for QadicField {
    #[inline]
    fn zero(&self) -> QadicElem {
        self.default()
    }
}

impl Multiplicative for QadicField {
    #[inline]
    fn one(&self) -> QadicElem {
        let mut res = self.default();
        unsafe { flint_sys::qadic::qadic_one(res.as_mut_ptr()); }
        res
    }
}

impl AdditiveGroup for QadicField {}

impl MultiplicativeGroup for QadicField {}
/*
impl Ring for QadicField {}

impl Field for QadicField {}
*/
impl<T> Init4<&Integer, T, T, &str> for QadicField where
    T: TryInto<c_long>
{
    fn init(p: &Integer, k: T, deg: T, var: &str) -> Self {
        match k.try_into() {
            Ok(kk) => match deg.try_into() {
                Ok(d) => {
                    let tmp = CString::new(var).unwrap();
                    let mut z = MaybeUninit::uninit();
                    unsafe {
                        flint_sys::qadic::qadic_ctx_init(
                            z.as_mut_ptr(), 
                            p.as_ptr(), 
                            d,
                            0,
                            kk,
                            tmp.as_ptr(),
                            PADIC_DEFAULT_PRINT_MODE
                        );
                        QadicField { ctx: Arc::new(QadicCtx(z.assume_init())) }
                    }
                },
                Err(_) => panic!("Input cannot be converted into a signed long!"),
            },
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }
}

impl<T, U> Init4<T, U, U, &str> for QadicField where
    T: Into<Integer>,
    U: TryInto<c_long>
{
    #[inline]
    fn init(p: T, k: U, deg: U, var: &str) -> Self {
        Self::init(&p.into(), k, deg, var)
    }
}

impl New<&IntPol> for QadicField {
    fn new(&self, x: &IntPol) -> QadicElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::qadic::qadic_init(z.as_mut_ptr());
            flint_sys::qadic::qadic_set_fmpz_poly(z.as_mut_ptr(), x.as_ptr(), self.as_ptr());
            QadicElem { 
                ctx: Arc::clone(&self.ctx), 
                extra: (), 
                data: z.assume_init() 
            }
        }
    }
}

impl<T> New<T> for QadicField where
    T: Into<IntPol>,
{
    fn new(&self, x: T) -> QadicElem {
        self.new(&x.into())
    }
}

impl QadicField {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &qadic_ctx_struct {
        &self.ctx.0
    }
}

/// An element of a q-adic field.
pub type QadicElem = Elem<QadicField>;

impl Element for QadicElem {
    type Data = qadic_struct;
    type Parent = QadicField;

    #[inline]
    fn parent(&self) -> QadicField {
        QadicField { ctx: Arc::clone(&self.ctx) }
    }
}

impl AdditiveElement for QadicElem {
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::qadic::qadic_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for QadicElem {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::qadic::qadic_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for QadicElem {}

impl MultiplicativeGroupElement for QadicElem {}

/*
impl RingElement for QadicElem {}

impl FieldElement for QadicElem {}
*/

impl QadicElem {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &qadic_struct {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut qadic_struct {
        &mut self.data
    }

    /// A reference to the struct holding context information. This is only needed to interface
    /// directly with FLINT via the FFI.
    pub fn ctx_as_ptr(&self) -> &qadic_ctx_struct {
        &self.ctx.0
    }

    /*
    /// Return a [String] representation of a qadic number.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::padic::padic_get_str(
                std::ptr::null(), 
                self.as_ptr(), 
                self.ctx_as_ptr()
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }*/

}
