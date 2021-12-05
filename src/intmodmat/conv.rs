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



/*
impl_from! {
    IntPol, IntModPol
    {
        fn from(x: &IntModPol) -> IntPol {
            let mut res = IntPol::default();
            unsafe { 
                flint_sys::fmpz_mod_poly::fmpz_mod_poly_get_fmpz_poly(
                    res.as_mut_ptr(),
                    x.as_ptr(),
                    x.ctx_as_ptr(),
                );
            }
            res
        }
    }
}

impl_from! {
    String, IntModPol
    {
        fn from(x: &IntModPol) -> String {
            String::from(IntPol::from(x))
        }
    }
}*/
