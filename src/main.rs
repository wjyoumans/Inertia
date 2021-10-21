#![feature(min_specialization)]

use std::mem::MaybeUninit;
use std::sync::Arc;
use flint_sys::fmpz::fmpz;

pub trait Parent {
    type Data;
    type Element: Element;
}

pub trait Element {
    type Data;
    type Parent: Parent;
}

pub struct Wrap<T> {
    pub wrap: T,
}

impl<T> Drop for Wrap<T> {
    default fn drop(&mut self) {
        drop(self);
    }
}

pub struct Elem<T: Parent> {
    pub ctx: Arc<T::Data>,
    pub data: <T::Element as Element>::Data,
}

impl<T: Parent> Drop for Elem<T> {
    default fn drop(&mut self) {
        drop(self);
    }
}

// Integer //

pub struct IntegerRing {}
impl Parent for IntegerRing {
    type Data = ();
    type Element = Integer;
}

impl IntegerRing {
    pub fn init() -> Self {
        IntegerRing {}
    }
    
    pub fn new(&self, m: i32) -> Integer {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz::fmpz_init(z.as_mut_ptr());
            flint_sys::fmpz::fmpz_set_si(z.as_mut_ptr(), m as i64);
            Integer { 
                ctx: Arc::new(()), 
                data: z.assume_init(),
            }
        }
    }
}

pub type Integer = Elem<IntegerRing>;

impl Drop for Integer {
    fn drop(&mut self) {
        drop(&mut self.ctx);
        unsafe { flint_sys::fmpz::fmpz_clear(self.as_mut_ptr());}
    }
}

impl Element for Integer {
    type Data = fmpz;
    type Parent = IntegerRing;
}

impl Default for Integer {
    fn default() -> Self {
        let zz = IntegerRing::init();
        zz.new(0)
    }
}

impl Integer {
    pub fn as_ptr(&self) -> &fmpz {
        &self.data
    }
    
    pub fn as_mut_ptr(&mut self) -> &mut fmpz {
        &mut self.data
    }
}

fn main() {

    // the Integer is properly dropped (ref count is decremented)
    {
        let zz = IntegerRing::init();
        let x = zz.new(3);
        assert_eq!(1, Arc::<()>::strong_count(&x.ctx));
        {
            let _ = Integer {
                ctx: Arc::clone(&x.ctx),
                data: x.data.clone(),
            };
            assert_eq!(2, Arc::<()>::strong_count(&x.ctx));
        }
        assert_eq!(1, Arc::<()>::strong_count(&x.ctx));
    }
}
