
macro_rules! derive_cmp {
    (
        eq
        $t:ident
    ) => {
        impl_cmp! {
            eq
            $t
            {
                fn eq(&self, rhs: &$t) -> bool {
                    unimplemented!()
                }
            }
        }
    };
    (
        partial_eq
        $t1:ident, $t2:ident
    ) => {
        impl_cmp! {
            partial_eq
            $t1, $t2
            {
                fn eq(&self, rhs: &$t2) -> bool {
                    unimplemented!()
                }
            }
        }
        impl_cmp! {
            partial_eq
            $t2, $t1
            {
                fn eq(&self, rhs: &$t1) -> bool {
                    unimplemented!()
                }
            }
        }
    };
    (
        partial_eq
        $t1:ident, $cast:ident {$($t2:ident)+}
    ) => ($(
        impl_cmp! {
            partial_eq
            $t1, $t2
            {
                fn eq(&self, rhs: &$t2) -> bool {
                    unimplemented!()
                }
            }
        }
        impl_cmp! {
            partial_eq
            $t2, $t1
            {
                fn eq(&self, rhs: &$t1) -> bool {
                    unimplemented!()
                }
            }
        }
    )+);
    (
        ord
        $t:ident
    ) => {
        impl_cmp! {
            ord
            $t
            {
                fn cmp(&self, rhs: &$t) -> Ordering {
                    unimplemented!()
                    let cmp = unsafe { $func(self.as_ptr(), rhs.as_ptr()) };
                    if cmp == 0 {
                        Equal
                    } else if cmp < 0 {
                        Less
                    } else {
                        Greater
                    }
                }
            }
        }
    };
    (
        partial_ord
        $t1:ident, $t2:ident
        $func:path
    ) => {
        impl_cmp! {
            partial_ord
            $t1, $t2
            {
                fn partial_cmp(&self, rhs: &$t2) -> Option<Ordering> {
                    unimplemented!()
                    let cmp = unsafe { $func(self.as_ptr(), rhs.as_ptr()) };
                    if cmp == 0 {
                        Some(Equal)
                    } else if cmp < 0 {
                        Some(Less)
                    } else {
                        Some(Greater)
                    }
                }
            }
        }
        impl_cmp! {
            partial_ord
            $t2, $t1
            {
                fn partial_cmp(&self, rhs: &$t1) -> Option<Ordering> {
                    unimplemented!()
                    let cmp = unsafe { $func(rhs.as_ptr(), self.as_ptr()) };
                    if cmp == 0 {
                        Some(Equal)
                    } else if cmp > 0 {
                        Some(Less)
                    } else {
                        Some(Greater)
                    }
                }
            }
        }
    };
    (
        partial_ord
        $t1:ident, $cast:ident {$($t2:ident)+}
        $func:path
    ) => ($(
        impl_cmp! {
            partial_ord
            $t1, $t2
            {
                fn partial_cmp(&self, rhs: &$t2) -> Option<Ordering> {
                    unimplemented!()
                    let cmp = unsafe { $func(self.as_ptr(), *rhs as $cast) };
                    if cmp == 0 {
                        Some(Equal)
                    } else if cmp < 0 {
                        Some(Less)
                    } else {
                        Some(Greater)
                    }
                }
            }
        }
        impl_cmp! {
            partial_ord
            $t2, $t1
            {
                fn partial_cmp(&self, rhs: &$t1) -> Option<Ordering> {
                    unimplemented!()
                    /*
                    let cmp = unsafe { $func(rhs.as_ptr(), *self as $cast) };
                    if cmp == 0 {
                        Some(Equal)
                    } else if cmp > 0 {
                        Some(Less)
                    } else {
                        Some(Greater)
                    }
                    */
                }
            }
        }
    )+)
}

macro_rules! derive_unop {
    (
        $kw:ident
        $in:ident
        $op:ident, $meth:ident
        $op_assign:ident, $meth_assign:ident
    ) => {
        impl_unop! {
            $in
            $op {$meth}
            {
                fn $meth(self) -> $in {
                    derive_unop!(@call $kw, $meth, self, $in)
                }
            }
            $op_assign {$meth_assign}
            {
                fn $meth_assign(&mut self) {
                    self.inner_mut().$meth_assign();
                }
            }
        }
    };
    (
        $kw:ident
        $in:ident, $out:ident
        $op:ident, $meth:ident
    ) => {
        impl_unop! {
            $in, $out
            $op {$meth}
            {
                fn $meth(self) -> $out {
                    derive_unop!(@call $kw, $meth, self, $out)
                }
            }
        }
    };
    (@call ctx, $meth:ident, $in:ident, $out:ident) => {
        $out::from_raw($in.inner().$meth(), $in.context().clone())
    };
    (@call $kw:ident, $meth:ident, $in:ident, $out:ident) => {
        $out::from_raw($in.inner().$meth())
    };
}

macro_rules! derive_binop {
    (
        $kw:ident
        $lhs:ident, $rhs:ident, $out:ident
        $(
            $op:ident, $meth:ident
            $op_assign:ident, $meth_assign:ident
            $op_from:ident, $meth_from:ident
            $assign_op:ident, $assign_meth:ident
        )*
    ) => ($(
        impl_binop! {
            $lhs, $rhs, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$rhs) -> $out {
                    derive_binop!(@call $kw, $meth, self, rhs, $out)
                }
            }
            $op_assign {$meth_assign}
            { 
                fn $meth_assign(&mut self, rhs: &$rhs) {
                    self.inner_mut().$meth_assign(rhs.inner());
                }
            }
            $op_from {$meth_from}
            {
                fn $meth_from(&mut self, lhs: &$lhs) {
                    self.inner_mut().$meth_from(lhs.inner());
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$lhs, rhs: &$rhs) {
                    self.inner_mut().$assign_meth(lhs.inner(), rhs.inner());
                }
            }
        }
    )*);
    (
        // a + b = a
        $kw:ident
        op_assign
        $lhs:ident, $rhs:ident, $out:ident
        $(
            $op:ident, $meth:ident
            $op_assign:ident, $meth_assign:ident
            $assign_op:ident, $assign_meth:ident
        )+
    ) => ($(
        impl_binop! {
            op_assign
            $lhs, $rhs, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$rhs) -> $out {
                    derive_binop!(@call $kw, $meth, self, rhs, $out)
                }
            }
            $op_assign {$meth_assign}
            {
                fn $meth_assign(&mut self, rhs: &$rhs) {
                    self.inner_mut().$meth_assign(rhs.inner());
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$lhs, rhs: &$rhs) {
                    self.inner_mut().$assign_meth(lhs.inner(), rhs.inner());
                }
            }
        }
    )*);
    (
        // a + b = b
        $kw:ident
        op_from
        $lhs:ident, $rhs:ident, $out:ident
        $(
            $op:ident, $meth:ident
            $op_from:ident, $meth_from:ident
            $assign_op:ident, $assign_meth:ident
        )+
    ) => ($(
        impl_binop! {
            op_from
            $lhs, $rhs, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$rhs) -> $out {
                    derive_binop!(@call $kw, $meth, self, rhs, $out)
                }
            }
            $op_from {$meth_from}
            {
                fn $meth_from(&mut self, lhs: &$lhs) {
                    self.inner_mut().$meth_from(lhs.inner());
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$lhs, rhs: &$rhs) {
                    self.inner_mut().$assign_meth(lhs.inner(), rhs.inner());
                }
            }
        }
    )+);
    (
        // a + b = c
        $kw:ident
        $lhs:ident, $rhs:ident, $out:ident
        $(
            $op:ident, $meth:ident
            $assign_op:ident, $assign_meth:ident
        )+
    ) => ($(
        impl_binop! {
            $lhs, $rhs, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$rhs) -> $out {
                    derive_binop!(@call $kw, $meth, self, rhs, $out)
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$lhs, rhs: &$rhs) {
                    self.inner_mut().$assign_meth(lhs.inner(), rhs.inner());
                }
            }
        }
    )+);
    (@call ctx_rhs, $meth:ident, $lhs:ident, $rhs:ident, $out:ident) => {
        //assert_eq!($lhs.context(), $rhs.context())
        $out::from_raw($lhs.inner().$meth($rhs.inner()), $rhs.context().clone())
    };
    (@call ctx, $meth:ident, $lhs:ident, $rhs:ident, $out:ident) => {
        //assert_eq!($lhs.context(), $rhs.context())
        $out::from_raw($lhs.inner().$meth($rhs.inner()), $lhs.context().clone())
    };
    (@call $kw:ident, $meth:ident, $lhs:ident, $rhs:ident, $out:ident) => {
        $out::from_raw($lhs.inner().$meth($rhs.inner()))
    };
}

macro_rules! derive_from {
    ($out_ty:ident, $($in:ident)*) => ($(
        impl_from! {
            $out_ty, $in
            {
                fn from(src: &$in) -> $out_ty {
                    $out_ty::from_raw(core::$out_ty::from(src))
                }
            }
        }
    )*);
    ($out_ty:ident, $out_ctx:ident, $($in:ident)*) => ($(
        impl_from! {
            $out_ty, $in
            {
                fn from(src: &$in) -> $out_ty {
                    $out_ty::from_raw(core::$out_ty::from(src), $out_ctx::default())
                }
            }
        }
    )*);
}

//macro_rules! derive_new {}

//macro_rules! derive_new_element {}

/*
macro_rules! derive_scalar_binops {
    (
        $ident:ident, $scalar:ident
        $op:ident, $meth:ident
        $op_assign:ident, $meth_assign:ident
        $op_from:ident, $meth_from:ident
        $assign_op:ident, $assign_meth:ident
    ) => {
    
        impl $op_assign<OwnedElement<$scalar>> for $ident {
            #[inline]
            fn $meth_assign(&mut self, rhs: OwnedElement<$scalar>) {
                self.inner_mut().$meth_assign(rhs.into_inner().into_inner());
            }
        }
        
        impl $op_assign<BorrowedElement<'_, $scalar>> for $ident {
            #[inline]
            fn $meth_assign(&mut self, rhs: BorrowedElement<$scalar>) {
                self.inner_mut().$meth_assign(rhs.into_inner().inner());
            }
        }
        
        impl $op_from<OwnedElement<$scalar>> for $ident {
            #[inline]
            fn $meth_from(&mut self, lhs: OwnedElement<$scalar>) {
                self.inner_mut().$meth_from(lhs.into_inner().into_inner());
            }
        }
        
        impl $op_from<BorrowedElement<'_, $scalar>> for $ident {
            #[inline]
            fn $meth_from(&mut self, lhs: BorrowedElement<$scalar>) {
                self.inner_mut().$meth_from(lhs.into_inner().inner());
            }
        }

        impl $op<OwnedElement<$scalar>> for $ident {
            type Output = $ident;
            fn $meth(mut self, rhs: OwnedElement<$scalar>) -> Self::Output {
                self.inner_mut().$meth_assign(rhs.into_inner().into_inner());
                self
            }
        }
        
        impl $op<BorrowedElement<'_, $scalar>> for $ident {
            type Output = $ident;
            fn $meth(mut self, rhs: BorrowedElement<$scalar>) -> Self::Output {
                self.inner_mut().$meth_assign(rhs.into_inner().inner());
                self
            }
        }
        
        impl $op<OwnedElement<$scalar>> for &$ident {
            type Output = $ident;
            fn $meth(self, rhs: OwnedElement<$scalar>) -> Self::Output {
                let mut res = self.clone();
                res.inner_mut().$meth_assign(rhs.into_inner().into_inner());
                res
            }
        }
        
        impl $op<BorrowedElement<'_, $scalar>> for &$ident {
            type Output = $ident;
            fn $meth(self, rhs: BorrowedElement<$scalar>) -> Self::Output {
                let mut res = self.clone();
                res.inner_mut().$meth_assign(rhs.into_inner().inner());
                res
            }
        }

        impl $op<$ident> for OwnedElement<$scalar> {
            type Output = $ident;
            #[inline]
            fn $meth(self, mut rhs: $ident) -> Self::Output {
                rhs.inner_mut().$meth_from(self.into_inner().into_inner());
                rhs
            }
        }
        
        impl $op<$ident> for BorrowedElement<'_, $scalar> {
            type Output = $ident;
            #[inline]
            fn $meth(self, mut rhs: $ident) -> Self::Output {
                rhs.inner_mut().$meth_from(self.into_inner().inner());
                rhs
            }
        }
        
        impl $op<&$ident> for OwnedElement<$scalar> {
            type Output = $ident;
            #[inline]
            fn $meth(self, rhs: &$ident) -> Self::Output {
                let mut res = rhs.clone();
                res.inner_mut().$meth_from(self.into_inner().into_inner());
                res
            }
        }
        
        impl $op<&$ident> for BorrowedElement<'_, $scalar> {
            type Output = $ident;
            #[inline]
            fn $meth(self, rhs: &$ident) -> Self::Output {
                let mut res = rhs.clone();
                res.inner_mut().$meth_from(self.into_inner().inner());
                res
            }
        }
    }
}
*/

/*
macro_rules! derive_binop {
    (
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_assign:ident {$meth_assign:ident}
            $op_from:ident {$meth_from:ident}
            $assign_op:ident {$assign_meth:ident}
        )+
    ) => ($(
        impl $op<&$t2> for &$t1 {
            type Output = $out;
            #[inline]
            $($code)*
        }
       
        impl $op<$t2> for &$t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, mut rhs: $t2) -> $out {
                rhs.$meth_from(self);
                rhs
            }
        }

        impl $op<&$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(mut self, rhs: &$t2) -> $out {
                self.$meth_assign(rhs);
                self
            }
        }

        impl $op<$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(mut self, rhs: $t2) -> $out {
                self.$meth_assign(&rhs);
                self
            }
        }

        derive_binop! {@op_assign
            $t1, $t2, $out
            $op_assign {$meth_assign}
        }

        derive_binop! {@op_from
            $t1, $t2, $out
            $op_from {$meth_from}
        }

        derive_binop! {@assign_op
            $t1, $t2, $out
            $assign_op {$assign_meth}
        }
    )+);
    (
        op_assign
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_assign:ident {$meth_assign:ident}
            $assign_op:ident {$assign_meth:ident}
        )+
    ) => ($(

        impl $op<&$t2> for &$t1 {
            type Output = $out;
            #[inline]
            $($code)*
        }

        impl $op<$t2> for &$t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, rhs: $t2) -> $out {
                self.$meth(&rhs)
            }
        }

        impl $op<&$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(mut self, rhs: &$t2) -> $out {
                self.$meth_assign(rhs);
                self
            }
        }

        impl $op<$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(mut self, rhs: $t2) -> $out {
                self.$meth_assign(&rhs);
                self
            }
        }

        impl_binop! {@op_assign
            $t1, $t2, $out
            $op_assign {$meth_assign}
            {
                $($code_assign)*
            }
        }

        impl_binop! {@assign_op
            $t1, $t2, $out
            $assign_op {$assign_meth}
            {
                $($assign_code)*
            }
        }
    )+);
    (
        op_from
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_from:ident {$meth_from:ident}
            $assign_op:ident {$assign_meth:ident}
        )+
    ) => ($(
        impl $op<&$t2> for &$t1 {
            type Output = $out;
            #[inline]
            $($code)*
        }

        impl $op<$t2> for &$t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, mut rhs: $t2) -> $out {
                rhs.$meth_from(self);
                rhs
            }
        }

        impl $op<&$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, rhs: &$t2) -> $out {
                (&self).$meth(rhs)
            }
        }

        impl $op<$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, mut rhs: $t2) -> $out {
                rhs.$meth_from(self);
                rhs
            }
        }

        impl_binop! {@op_from
            $t1, $t2, $out
            $op_from {$meth_from}
            {
                $($code_from)*
            }
        }
        impl_binop! {@assign_op
            $t1, $t2, $out
            $assign_op {$assign_meth}
            {
                $($assign_code)*
            }
        }
    )+);
}*/
