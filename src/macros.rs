
/*
macro_rules! op_guard {
    (Add, matrix, $lhs: expr, $rhs: expr) => {
        if ($lhs.ncols() != $rhs.ncols()) || ($lhs.nrows() != $rhs.nrows()) {
            panic!("Incompatible matrix dimensions.")
    	}
    };
	(Sub, matrix, $lhs: expr, $rhs: expr) => {
        if ($lhs.ncols() != $rhs.ncols()) || ($lhs.nrows() != $rhs.nrows()) {
            panic!("Incompatible matrix dimensions.")
        }
    };
    (Mul, matrix, $lhs: expr, $rhs: expr) => {
        if $lhs.ncols() != $rhs.nrows() {
            panic!("Incompatible matrix dimensions.")
    	}
    };
    (Div, $arg_kw: ident, $lhs: expr, $rhs: expr) => {
        if !$rhs.is_invertible() {
            panic!("Divisor must be nonzero and invertible.")
    	}
    };
	(Rem, $arg_kw: ident, $lhs: expr, $rhs: expr) => {
        if $rhs.is_zero() {
            panic!("Modulus must be nonzero (and positive if applicable).")
        }
    };
    ($op: ident, ctx, $lhs: expr, $rhs: expr) => {
        if $lhs.ctx() != $rhs.ctx() {
            panic!("Incompatible contexts.")
        }
    };
    ($op: ident, $arg_kw: ident, $lhs: expr, $rhs: expr) => {}
}
*/

macro_rules! impl_cmp {
    // a = a
    (
        eq
        $t:ident
        {
            $($code:tt)*
        }
    ) => {
        impl Eq for $t {}
        
        impl Eq for &$t {}

        impl PartialEq for $t {
            #[inline]
            $($code)*
        }

        impl PartialEq<&$t> for $t {
            #[inline]
            fn eq(&self, rhs: &&$t) -> bool {
                rhs.eq(&self)
            }
        }
        
        impl PartialEq<$t> for &$t {
            #[inline]
            fn eq(&self, rhs: &$t) -> bool {
                self.eq(&rhs)
            }
        }
    };
    // a > a
    (
        ord
        $t:ident
        {
            $($code:tt)*
        }
    ) => {
        impl Ord for $t {
            #[inline]
            $($code)*
        }
        
        impl PartialOrd for $t {
            #[inline]
            fn partial_cmp(&self, rhs: &$t) -> Option<Ordering> {
                Some(self.cmp(rhs))
            }
        }
    };
    // a = b

}

macro_rules! impl_cmp_unsafe {
    (
        eq
        $t:ident
        $func:path
    ) => {
        impl_cmp! {
            eq
            $t
            {
                fn eq(&self, rhs: &$t) -> bool {
                    unsafe { $func(self.as_ptr(), rhs.as_ptr()) == 1 }
                }
            }
        }
    };
    (
        ord
        $t:ident
        $func:path
    ) => {
        impl_cmp! {
            ord
            $t
            {
                fn cmp(&self, rhs: &$t) -> Ordering {
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
    }
}

macro_rules! op_guard {
    (Div, $rhs:expr) => {
        if !$rhs.is_invertible() {
            panic!("Invalid divisor (zero or not invertible).")
        }
    };
    (Rem, $rhs:expr) => {
        // TODO: check if negative (doesn't make sense for polynomials)
        if $rhs.is_zero() {
            panic!("Invalid modulus (zero or negative).")
        }
    };
    ($op:ident, $rhs:expr) => {}
}


macro_rules! impl_unop {
    (
        // assign
        $t:ident
        $op:ident {$meth:ident}
        {
            $($code:tt)*
        }
        $op_assign:ident {$meth_assign:ident}
        {
            $($code_assign:tt)*
        }
    ) => {
        impl $op for $t {
            type Output = $t;
            #[inline]
            fn $meth(mut self) -> $t {
                self.$meth_assign();
                self
            }
        }
        
        impl $op for &$t {
            type Output = $t;
            #[inline]
            $($code)*
        }

        impl $op_assign for $t {
            #[inline]
            $($code_assign)*
        }
    };
    (
        // no assign
        $t:ident, $out:ident
        $op:ident {$meth:ident}
        {
            $($code:tt)*
        }
    ) => {
        impl $op for $t {
            type Output = $out;
            #[inline]
            $($code)*
        }
        
        impl $op for &$t {
            type Output = $out;
            #[inline]
            $($code)*
        }
    };
}

macro_rules! impl_unop_unsafe {
    (
        $t:ident
        $op:ident {$meth:ident}
        $op_assign:ident {$meth_assign:ident}
        $func:path
    ) => {

        impl_unop! {
            $t
            $op {$meth}
            {
                fn $meth(self) -> $t {
                    let mut res = $t::default();
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr()); }
                    res
                }
            }
            $op_assign {$meth_assign}
            {
                fn $meth_assign(&mut self) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr()); }
                }
            }
        }
    };
    (
        $t:ident, $out:ident
        $op:ident {$meth:ident}
        $func:path
    ) => {

        impl_unop! {
            $t, $out
            $op {$meth}
            {
                fn $meth(self) -> $out {
                    let mut res = $out::default();
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr()); }
                    res
                }
            }
        }
    }
}

macro_rules! impl_binop {
    (
        // a + a = a
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            {
                $($code:tt)*
            }
            $op_assign:ident {$meth_assign:ident}
            {
                $($code_assign:tt)*
            }
            $op_from:ident {$meth_from:ident}
            {
                $($code_from:tt)*
            }
            $assign_op:ident {$assign_meth:ident}
            {
                $($assign_code:tt)*
            }
        )*
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
       
        impl_binop! {@op_assign
            $t1, $t2, $out
            $op_assign {$meth_assign}
            {
                $($code_assign)*
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
    )*);
    (
        // a + b = a
        op_assign
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            {
                $($code:tt)*
            }
            $op_assign:ident {$meth_assign:ident}
            {
                $($code_assign:tt)*
            }
            $assign_op:ident {$assign_meth:ident}
            {
                $($assign_code:tt)*
            }
        )*
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
    )*);
    (
        // a + b = b
        op_from
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            {
                $($code:tt)*
            }
            $op_from:ident {$meth_from:ident}
            {
                $($code_from:tt)*
            }
            $assign_op:ident {$assign_meth:ident}
            {
                $($assign_code:tt)*
            }
        )*
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
    )*);
    (
        // a + b = c
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            {
                $($code:tt)*
            }
            $assign_op:ident {$assign_meth:ident}
            {
                $($assign_code:tt)*
            }
        )*
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
            fn $meth(self, rhs: &$t2) -> $out {
                (&self).$meth(rhs)
            }
        }
        
        impl $op<$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth(self, rhs: $t2) -> $out {
                self.$meth(&rhs)
            }
        }
        
        impl_binop! {@assign_op
            $t1, $t2, $out
            $assign_op {$assign_meth}
            {
                $($assign_code)*
            }
        }
    )*);
    (
        @op_assign
        $t1:ident, $t2:ident, $out:ident
        $op_assign:ident {$meth_assign:ident}
        {
            $($code_assign:tt)*
        }
    ) => {
        impl $op_assign<&$t2> for $t1 {
            #[inline]
            $($code_assign)*
        }

        impl $op_assign<$t2> for $t1 {
            #[inline]
            fn $meth_assign(&mut self, rhs: $t2) {
                self.$meth_assign(&rhs);
            }
        }
    };
    (
        @op_from
        $t1:ident, $t2:ident, $out:ident
        $op_from:ident {$meth_from:ident}
        {
            $($code_from:tt)*
        }
    ) => {
        impl $op_from<&$t1> for $t2 {
            #[inline]
            $($code_from)*
        }

        impl $op_from<$t1> for $t2 {
            #[inline]
            fn $meth_from(&mut self, lhs: $t1) {
                self.$meth_from(&lhs);
            }
        }
    };
    (
        @assign_op
        $t1:ident, $t2:ident, $out:ident
        $assign_op:ident {$assign_meth:ident}
        {
            $($assign_code:tt)*
        }
    ) => {
        impl $assign_op<&$t1, &$t2> for $out {
            #[inline]
            $($assign_code)*
        }
        
        impl $assign_op<$t1, &$t2> for $out {
            #[inline]
            fn $assign_meth(&mut self, lhs: $t1, rhs: &$t2) {
                self.$assign_meth(&lhs, rhs);
            }
        }
        
        impl $assign_op<&$t1, $t2> for $out {
            #[inline]
            fn $assign_meth(&mut self, lhs: &$t1, rhs: $t2) {
                self.$assign_meth(lhs, &rhs);
            }
        }
        
        impl $assign_op<$t1, $t2> for $out {
            #[inline]
            fn $assign_meth(&mut self, lhs: $t1, rhs: $t2) {
                self.$assign_meth(&lhs, &rhs);
            }
        }
    };
}

macro_rules! impl_binop_unsafe {
    (
        // a + a = a
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_assign:ident {$meth_assign:ident}
            $op_from:ident {$meth_from:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )+
    ) => ($(
        impl_binop! {
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = $out::default();
                    //op_guard!($op, rhs);
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                    res
                }
            }
            $op_assign {$meth_assign}
            {
                fn $meth_assign(&mut self, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                }
            }
            $op_from {$meth_from}
            {
                fn $meth_from(&mut self, lhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), self.as_ptr()); }
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), rhs.as_ptr()); }
                }
            }
        }
    )+);
    (
        // a + b = a
        op_assign
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_assign:ident {$meth_assign:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )+
    ) => ($(
        impl_binop! {
            op_assign
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = $out::default();
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                    res
                }
            }
            $op_assign {$meth_assign}
            {
                fn $meth_assign(&mut self, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), rhs.as_ptr()); }
                }
            }
        }
    )+);
    (
        // a + b = a, b primitive
        op_assign    
        $t1:ident, $cast:ty {$($t2:ident)+}, $out:ident
        
        $op:ident {$meth:ident}
        $op_assign:ident {$meth_assign:ident}
        $assign_op:ident {$assign_meth:ident}
        $func:path;

        $($next:tt)*
    ) => ($(
        impl_binop_unsafe! {@inner
            op_assign
            $t1, $cast {$t2}, $out
            
            $op {$meth}
            $op_assign {$meth_assign}
            $assign_op {$assign_meth}
            $func;
        })+

        impl_binop_unsafe! {
            op_assign
            $t1, $cast {$($t2)+}, $out
            $($next)*
        }
    );
    (@inner
        op_assign
        $t1:ident, $cast:ty {$t2:ident}, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_assign:ident {$meth_assign:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )*
    ) => ($(
        impl_binop! {
            op_assign
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = $out::default();
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), *rhs as $cast); }
                    res
                }
            }
            $op_assign {$meth_assign}
            {
                fn $meth_assign(&mut self, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), *rhs as $cast); }
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), *rhs as $cast); }
                }
            }
        }
    )*);
    (
        op_assign
        $t1:ident, $cast:ty {$($t2:ident)+}, $out:ident
    ) => {};
    (
        // a + b = b
        op_from
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_from:ident {$meth_from:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )+
    ) => ($(
        impl_binop! {
            op_from
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = $out::default();
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                    res
                }
            }
            $op_from {$meth_from}
            {
                fn $meth_from(&mut self, lhs: &$t1) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), self.as_ptr()); }
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), rhs.as_ptr()); }
                }
            }
        }
    )+);
    (
        // a + b = b, a primitive
        op_from
        $cast:ty {$($t1:ident)+}, $t2:ident, $out:ident
        
        $op:ident {$meth:ident}
        $op_from:ident {$meth_from:ident}
        $assign_op:ident {$assign_meth:ident}
        $func:path;

        $($next:tt)*
    ) => ($(
        impl_binop_unsafe! {@inner
            op_from
            $cast {$t1}, $t2, $out
            
            $op {$meth}
            $op_from {$meth_from}
            $assign_op {$assign_meth}
            $func;
        })+

        impl_binop_unsafe! {
            op_from
            $cast {$($t1)+}, $t2, $out
            $($next)*
        }
    );
    (@inner
        op_from
        $cast:ty {$t1:ident}, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $op_from:ident {$meth_from:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )*
    ) => ($(
        impl_binop! {
            op_from
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = $out::default();
                    unsafe { $func(res.as_mut_ptr(), *self as $cast, rhs.as_ptr()); }
                    res
                }
            }
            $op_from {$meth_from}
            {
                fn $meth_from(&mut self, lhs: &$t1) {
                    unsafe { $func(self.as_mut_ptr(), *lhs as $cast, self.as_ptr()); }
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), *lhs as $cast, rhs.as_ptr()); }
                }
            }
        }
    )*);
    (
        op_from
        $cast:ty {$($t1:ident)+}, $t2:ident, $out:ident
    ) => {};
    (
        // a + b = c
        $t1:ident, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )+
    ) => ($(
        impl_binop! {
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = $out::default();
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                    res
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), rhs.as_ptr()); }
                }
            }
        }
    )+);
    (
        // a + b = c, a primitive
        $cast:ty {$($t1:ident)+}, $t2:ident, $out:ident
        
        $op:ident {$meth:ident}
        $assign_op:ident {$assign_meth:ident}
        $func:path;

        $($next:tt)*
    ) => ($(
        impl_binop_unsafe! {@inner
            $cast {$t1}, $t2, $out
            
            $op {$meth}
            $assign_op {$assign_meth}
            $func;
        })+

        impl_binop_unsafe! {
            $cast {$($t1)+}, $t2, $out
            $($next)*
        }
    );
    (@inner
        $cast:ty {$t1:ident}, $t2:ident, $out:ident
        $(
            $op:ident {$meth:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )*
    ) => ($(
        impl_binop! {
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = $out::default();
                    unsafe { $func(res.as_mut_ptr(), *self as $cast, rhs.as_ptr()); }
                    res
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), *lhs as $cast, rhs.as_ptr()); }
                }
            }
        }
    )*);
    (
        $cast:ty {$($t1:ident)+}, $t2:ident, $out:ident
    ) => {};
    (
        // a + b = c, b primitive
        $t1:ident, $cast:ty {$($t2:ident)+}, $out:ident
        
        $op:ident {$meth:ident}
        $assign_op:ident {$assign_meth:ident}
        $func:path;

        $($next:tt)*
    ) => ($(
        impl_binop_unsafe! {@inner
            $t1, $cast {$t2}, $out
            
            $op {$meth}
            $assign_op {$assign_meth}
            $func;
        })+

        impl_binop_unsafe! {
            $t1, $cast {$($t2)+}, $out
            $($next)*
        }
    );
    (@inner
        $t1:ident, $cast:ty {$t2:ident}, $out:ident
        $(
            $op:ident {$meth:ident}
            $assign_op:ident {$assign_meth:ident}
            $func:path;
        )*
    ) => ($(
        impl_binop! {
            $t1, $t2, $out
            $op {$meth}
            {
                fn $meth(self, rhs: &$t2) -> $out {
                    let mut res = $out::default();
                    unsafe { $func(res.as_mut_ptr(), self.as_ptr(), *rhs as $cast); }
                    res
                }
            }
            $assign_op {$assign_meth}
            {
                fn $assign_meth(&mut self, lhs: &$t1, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), *rhs as $cast); }
                }
            }
        }
    )*);
    (
        $t1:ident, $cast:ty {$($t2:ident)+}, $out:ident
    ) => {};
}

macro_rules! impl_from {
    (
        impl From<&$src:ident> for $dst:ident {
            fn from($arg:ident : &$arg_ty:ident) -> $dst_ty:ident {
                $($code:tt)*
            }
        }
    ) => {
        impl From<$src> for $dst {
            #[inline]
            fn from($arg: $arg_ty) -> $dst_ty {
                $dst::from(&$arg)
            }
        }
        
        impl From<&$src> for $dst {
            #[inline]
            fn from($arg: &$arg_ty) -> $dst_ty {
                $($code)*
            }
        }
        
        impl From<&mut $src> for $dst {
            #[inline]
            fn from($arg: &mut $arg_ty) -> $dst_ty {
                $($code)*
            }
        }
    }
}
