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
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;

use serde::ser::{Serialize, Serializer, SerializeSeq};
//use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess};

use crate::*;


impl AsRef<Integer> for Integer {
    fn as_ref(&self) -> &Integer {
        self
    }
}

impl fmt::Display for IntegerRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Integer ring")
    }
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_init_set(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Integer")
            .field("data", &self.data)
            .finish()
    }
}

impl Default for Integer {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz::fmpz_init(z.as_mut_ptr());
            Integer { data: IntegerData { elem: z.assume_init() } }
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ui_vector().hash(state);
    }
}

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer
    {
        let vec = self.get_ui_vector();
        println!("{:?}, len = {}", vec, vec.len());
        let mut seq = serializer.serialize_seq(Some(vec.len()))?;
        for e in vec.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

/*
impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de>
    {
        enum Field { Data }
        
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error> where 
                D: Deserializer<'de>
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`data`")
                    }

                    fn visit_str<E: de::Error>(self, value: &str) -> Result<Field, E> {
                        match value {
                            "data" => Ok(Field::Data),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct IntegerVisitor;

        impl<'de> Visitor<'de> for IntegerVisitor {
            type Value = Integer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Integer")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Integer, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut ui_vec = Vec::<u64>::new();    
                while let Some(x) = seq.next_element::<u64>()? {
                    ui_vec.push(x);
                }
                //println!("{:?}", ui_vec);
                let mut res = Integer::default();
                res.set_ui_vector(ui_vec);
                Ok(res)
            }
        }

        const FIELDS: &'static [&'static str] = &["data"];
        deserializer.deserialize_struct("Integer", FIELDS, IntegerVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        let x = Integer::from("18446744073709551616");
        let ser = bincode::serialize(&x).unwrap();
        let y: Integer = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
*/
