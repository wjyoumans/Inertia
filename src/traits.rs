use std::sync::Arc;

pub trait Assign<T> {
    fn assign(&mut self, src: T);

    // alias for assign
    #[inline]
    fn set(&mut self, src: T) {
        self.assign(src)
    }
}

impl<T> Assign<T> for T {
    fn assign(&mut self, src: T) {
        drop(std::mem::replace(self, src));
    }
}

pub trait AssignAdd<T, U> {
    fn assign_add(&mut self, lhs: T, rhs: U);
}

pub trait AssignSub<T, U> {
    fn assign_sub(&mut self, lhs: T, rhs: U);
}

pub trait AssignMul<T, U> {
    fn assign_mul(&mut self, lhs: T, rhs: U);
}

pub trait AssignDiv<T, U> {
    fn assign_div(&mut self, lhs: T, rhs: U);
}

pub trait AssignRem<T, U> {
    fn assign_rem(&mut self, lhs: T, rhs: U);
}

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
    default fn drop(&mut self) {}
}

pub struct Elem<T: Parent> {
    pub ctx: Arc<T::Data>,
    pub data: <T::Element as Element>::Data,
}

impl<T: Parent> Drop for Elem<T> {
    default fn drop(&mut self) {}
}

pub trait Inv {
    type Output;
    fn inv(self) -> Self::Output;
}

pub trait InvAssign {
    fn inv_assign(&mut self);
}
