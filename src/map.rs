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

use crate::Parent;
use rustc_hash::FxHashMap;
use std::rc::Rc;

/// A generic map between [Parents](Parent) backed by closures.
pub struct Map<'a, D: Parent, C: Parent> {
    domain: Rc<D>,
    codomain: Rc<C>,
    image: Box<dyn 'a + Fn(D::Element) -> Result<C::Element, ()>>,
    preimage: Option<Box<dyn 'a + Fn(C::Element) -> Result<D::Element, ()>>>,
}

impl<'a, D: Parent, C: Parent> Map<'a, D, C> {
    #[inline]
    pub fn new(
        domain: &Rc<D>,
        codomain: &Rc<C>,
        image: Box<dyn Fn(D::Element) -> Result<C::Element, ()>>,
        preimage: Option<Box<dyn Fn(C::Element) -> Result<D::Element, ()>>>,
    ) -> Self {
        Map {
            domain: Rc::clone(domain),
            codomain: Rc::clone(codomain),
            image,
            preimage,
        }
    }

    #[inline]
    pub fn domain(&self) -> Rc<D> {
        Rc::clone(&self.domain)
    }

    #[inline]
    pub fn codomain(&self) -> Rc<C> {
        Rc::clone(&self.codomain)
    }

    #[inline]
    pub fn image(&self, x: D::Element) -> Result<C::Element, ()> {
        (self.image)(x)
    }

    #[inline]
    pub fn map(&self, x: D::Element) -> Result<C::Element, ()> {
        self.image(x)
    }

    #[inline]
    pub fn preimage(&self, y: C::Element) -> Result<D::Element, ()> {
        if let Some(ref f) = self.preimage {
            f(y)
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn inv(&self, y: C::Element) -> Result<D::Element, ()> {
        self.preimage(y)
    }

    pub fn compose<T: Parent>(&'a self, other: &'a Map<C, T>) -> Map<D, T> {
        if let Some(ref g1) = self.preimage {
            if let Some(ref g2) = other.preimage {
                let f = |x| other.image(self.image(x)?) as _;
                let g = move |x| g1(g2(x)?);

                return Map::<D, T> {
                    domain: Rc::clone(&self.domain),
                    codomain: Rc::clone(&other.codomain),
                    image: Box::new(f),
                    preimage: Some(Box::new(g)),
                };
            }
        }

        let f = |x| other.image(self.image(x)?) as _;
        Map::<D, T> {
            domain: Rc::clone(&self.domain),
            codomain: Rc::clone(&other.codomain),
            image: Box::new(f),
            preimage: None,
        }
    }
}

#[allow(unused_macros)]
macro_rules! map {
    ($domain:ident; |$x:ident| $im:expr) => {{
        let dom = Rc::new($domain.clone());
        let f = move |$x| $im;
        Map::new(&dom, &dom, Box::new(f), None)
    }};
    ($domain:ident; |$x:ident| $im:expr; |$y:ident| $pre:expr) => {{
        let dom = Rc::new($domain.clone());
        let f = move |$x| $im;
        let g = move |$y| $pre;
        Map::new(&dom, &dom, Box::new(f), Some(Box::new(g)))
    }};
    ($domain:ident -> $codomain:ident; |$x:ident| $im:expr) => {{
        let dom = Rc::new($domain.clone());
        let co = Rc::new($codomain.clone());
        let m = move |$x| $im;
        Map::new(&dom, &co, Box::new(m), None)
    }};
    ($domain:ident -> $codomain:ident; |$x:ident| $im:expr; |$y:ident| $pre:expr) => {{
        let dom = Rc::new($domain.clone());
        let co = Rc::new($codomain.clone());
        let f = move |$x| $im;
        let g = move |$y| $pre;
        Map::new(&dom, &co, Box::new(f), Some(Box::new(g)))
    }};
}

// NOTE: We could construct preimage hashmap from image map, or verify preimage map
/// A generic map between [Parents](Parent) backed by [FxHashMaps](FxHashMap).
pub struct HMap<D: Parent, C: Parent> {
    domain: Rc<D>,
    codomain: Rc<C>,
    image: FxHashMap<D::Element, C::Element>,
    preimage: Option<FxHashMap<C::Element, D::Element>>,
}

impl<D: Parent, C: Parent> HMap<D, C> {
    #[inline]
    pub fn new(
        domain: &Rc<D>,
        codomain: &Rc<C>,
        image: FxHashMap<D::Element, C::Element>,
        preimage: Option<FxHashMap<C::Element, D::Element>>,
    ) -> Self {
        HMap {
            domain: Rc::clone(domain),
            codomain: Rc::clone(codomain),
            image,
            preimage,
        }
    }

    #[inline]
    pub fn domain(&self) -> Rc<D> {
        Rc::clone(&self.domain)
    }

    #[inline]
    pub fn codomain(&self) -> Rc<C> {
        Rc::clone(&self.codomain)
    }

    #[inline]
    pub fn image(&self, x: D::Element) -> Result<C::Element, ()> {
        match self.image.get(&x) {
            Some(v) => Ok(v.clone()),
            None => Err(()),
        }
    }

    #[inline]
    pub fn map(&self, x: D::Element) -> Result<C::Element, ()> {
        self.image(x)
    }

    #[inline]
    pub fn preimage(&self, y: C::Element) -> Result<D::Element, ()> {
        if let Some(ref map) = self.preimage {
            match map.get(&y) {
                Some(v) => Ok(v.clone()),
                None => Err(()),
            }
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn inv(&self, y: C::Element) -> Result<D::Element, ()> {
        self.preimage(y)
    }

    pub fn compose<T: Parent>(&self, other: &HMap<C, T>) -> HMap<D, T> {
        let mut f = FxHashMap::default();
        for (k, v) in self.image.iter() {
            if let Some(u) = other.image.get(v) {
                f.insert(k.clone(), u.clone());
            }
        }

        if let Some(ref g1) = self.preimage {
            if let Some(ref g2) = other.preimage {
                let mut g = FxHashMap::default();

                for (k, v) in g2.into_iter() {
                    if let Some(u) = g1.get(v) {
                        g.insert(k.clone(), u.clone());
                    }
                }

                return HMap {
                    domain: Rc::clone(&self.domain),
                    codomain: Rc::clone(&other.codomain),
                    image: f,
                    preimage: Some(g),
                };
            }
        }

        HMap {
            domain: Rc::clone(&self.domain),
            codomain: Rc::clone(&other.codomain),
            image: f,
            preimage: None,
        }
    }
}

#[allow(unused_macros)]
macro_rules! hmap {
    ($domain:ident; [ $(($key:expr, $val:expr)),* $(,)? ]) => {
        {
            let dom = Rc::new($domain.clone());
            let mut f = FxHashMap::default();
            $(f.insert($key, $val);)*
            HMap::new(&dom, &dom, f, None)
        }
    };
    (
        $domain:ident;
        [ $(($k1:expr, $v1:expr)),* $(,)? ],
        [ $(($k2:expr, $v2:expr)),* $(,)? ]
    ) => {
        {
            let dom = Rc::new($domain.clone());
            let mut f = FxHashMap::default();
            $(f.insert($k1, $v1);)*

            let mut g = FxHashMap::default();
            $(g.insert($k2, $v2);)*
            HMap::new(&dom, &dom, f, Some(g))
        }
    };
    ($domain:ident -> $codomain:ident; [ $(($key:expr, $val:expr)),* $(,)? ]) => {
        {
            let dom = Rc::new($domain.clone());
            let co = Rc::new($codomain.clone());
            let mut f = FxHashMap::default();
            $(f.insert($key, $val);)*
            HMap::new(&dom, &co, f, None)
        }
    };
    (
        $domain:ident -> $codomain:ident;
        [ $(($k1:expr, $v1:expr)),* $(,)? ],
        [ $(($k2:expr, $v2:expr)),* $(,)? ]
    ) => {
        {
            let dom = Rc::new($domain.clone());
            let co = Rc::new($codomain.clone());
            let mut f = FxHashMap::default();
            $(f.insert($k1, $v1);)*

            let mut g = FxHashMap::default();
            $(g.insert($k2, $v2);)*
            HMap::new(&dom, &co, f, Some(g))
        }
    };
}

#[cfg(test)]
mod test {
    use super::{FxHashMap, HMap, Map};
    use inertia_core::*;
    use std::rc::Rc;

    #[test]
    fn map() {
        let zz = IntegerRing::init();
        let rr = RationalField::init();

        let m1 = map!(zz; |x| Ok(x+5));
        assert_eq!(15, m1.map(zz.new(10)).unwrap());

        let m2 = map!(zz; |x| Ok(x+5); |y| Ok(y-5));
        assert_eq!(9, m2.map(Integer::from(4)).unwrap());
        assert_eq!(0, m2.inv(Integer::from(5)).unwrap());

        let m3 = map!(
            zz -> rr;
            |x| Ok(x*rr.new([3,2]));
            |y| {
                let t = y*rr.new([2,3]);
                match Integer::try_from(t) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(()),
                }
            }
        );
        assert_eq!(rr.new([9, 2]), m3.map(Integer::from(3)).unwrap());
        assert_eq!(8, m3.inv(Rational::from(12)).unwrap());

        let m4 = m2.compose(&m3);
        assert_eq!(18, m4.map(Integer::from(7)).unwrap());
        assert_eq!(-1, m4.inv(Rational::from(6)).unwrap());
    }

    #[test]
    fn hmap() {
        let zz = IntegerRing::init();
        //let rr = RationalField::init();

        let m1 = hmap!(zz; [
            (Integer::from(1), Integer::from(2)),
            (Integer::from(2), Integer::from(4))
        ]);
        assert_eq!(2, m1.map(zz.new(1)).unwrap());

        let m2 = hmap!(zz;
            [
                (Integer::from(1), Integer::from(2)),
                (Integer::from(2), Integer::from(4))
            ],
            [
                (Integer::from(2), Integer::from(1)),
                (Integer::from(4), Integer::from(2))
            ]
        );
        assert_eq!(1, m2.inv(zz.new(2)).unwrap());
    }
}
