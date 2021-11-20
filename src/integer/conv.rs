use crate::traits::Assign;
use crate::integer::src::Integer;


macro_rules! impl_from_prim {
    ($cast:ident $func:path; $($t:ident)*) => ($(
        impl_from! {
            impl From<&$t> for Integer {
                fn from(src: &$t) -> Integer {
                    let mut res = Integer::default();
                    unsafe { $func(res.as_mut_ptr(), *src as $cast); }
                    res
                }
            }
        }

    )*)
}

impl_from_prim! {u64 flint_sys::fmpz::fmpz_set_ui; usize u64 u32 u16 u8 }
impl_from_prim! {i64 flint_sys::fmpz::fmpz_set_si; isize i64 i32 i16 i8 }

impl From<Integer> for String {
    fn from(x: Integer) -> String {
        String::from(&x)
    }
}

impl From<&Integer> for String {
    fn from(x: &Integer) -> String {
        format!("{}", &x.to_str_radix(10))
    }
}

impl Assign<&Integer> for Integer {
    fn assign(&mut self, other: &Integer) {
        unsafe { flint_sys::fmpz::fmpz_set(self.as_mut_ptr(), other.as_ptr()); }
    }
}

