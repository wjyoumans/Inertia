
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


/* TODO: use op/div guard?
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
}*/

/*
macro_rules! incomplete {
    (
        $inc:ident
        $t1:ident
        $t2:ident
        $out:ident
        {$($code_assign:tt)*}
        {$($code_from:tt)*}
    ) => {
        pub struct $inc<'a> {
            lhs: &'a $t1,
            rhs: &'a $t2,
        }

        incomplete! {@incomplete $inc $out {$($code_assign)*} {$($code_from)*}}
    };
    // abc lhs owned
    (
        lhs_owned
        $inc:ident
        $t1:ident
        $t2:ident
        $out:ident
        {$($code_assign:tt)*}
        {$($code_from:tt)*}
    ) => {
        pub struct $inc<'a> {
            lhs: $t1,
            rhs: &'a $t2,
        }

        incomplete! {@incomplete $inc $out {$($code_assign)*} {$($code_from)*}}
    };
    // abc rhs owned
    (
        rhs_owned
        $inc:ident
        $t1:ident
        $t2:ident
        $out:ident
        {$($code_assign:tt)*}
        {$($code_from:tt)*}
    ) => {
        pub struct $inc<'a> {
            lhs: &'a $t1,
            rhs: $t2,
        }

        incomplete! {@incomplete $inc $out {$($code_assign)*} {$($code_from)*}}
    };
    (@incomplete 
        $inc:ident
        $out:ident
        {$($code_assign:tt)*}
        {$($code_from:tt)*}
    ) => {
        impl Assign<$inc<'_>> for $out {
            #[inline]
            $($code_assign)*
        }
        
        impl From<$inc<'_>> for $out {
            #[inline]
            $($code_from)*
        }

        impl Complete for $inc<'_> {
            type Completed = $out;
            #[inline]
            fn complete(self) -> $out {
                <$out>::from(self)
            }
        }
    }
}

macro_rules! impl_incomplete {
    (
        $t1:ident, $t2:ident, $out:ident
        $($func:path; $inc:ident)*
    ) => ($(
        incomplete! {
            $inc
            $t1
            $t2
            $out
            {
                fn assign(&mut self, src: $inc<'_>) {
                    unsafe { $func(self.as_mut_ptr(), src.lhs.as_ptr(), src.rhs.as_ptr());}
                }
            }
            {
                fn from(src: $inc<'_>) -> $out {
                    let mut res = $out::default();
                    res.assign(src);
                    res
                }
            }
        }
    )*);
    (
        lprim 
        $cast:ident, $t2:ident, $out:ident
        $func:path {
            $($inc:ident, $t1:ident)*
        }
    ) => ($(
        incomplete! {
            lhs_owned
            $inc
            $t1
            $t2
            $out
            {
                fn assign(&mut self, src: $inc<'_>) {
                    unsafe { $func(self.as_mut_ptr(), src.lhs as $cast, src.rhs.as_ptr());}
                }
            }
            {
                fn from(src: $inc<'_>) -> $out {
                    let mut res = $out::default();
                    res.assign(src);
                    res
                }
            }
        }
    )*);
    (
        rprim 
        $t1:ident, $cast:ident, $out:ident
        $func:path {
            $($inc:ident, $t2:ident)*
        }
    ) => ($(
        incomplete! {
            rhs_owned
            $inc
            $t1
            $t2
            $out
            {
                fn assign(&mut self, src: $inc<'_>) {
                    unsafe { $func(self.as_mut_ptr(), src.lhs.as_ptr(), src.rhs as $cast);}
                }
            }
            {
                fn from(src: $inc<'_>) -> $out {
                    let mut res = $out::default();
                    res.assign(src);
                    res
                }
            }
        }
    )*);
    (
        lhs_owned
        $t1:ident, $t2:ident, $out:ident
        $($func:path; $inc:ident)*
    ) => ($(
        incomplete! {
            lhs_owned
            $inc
            $t1
            $t2
            $out
            {
                fn assign(&mut self, src: $inc<'_>) {
                    unsafe { $func(self.as_mut_ptr(), src.lhs.as_ptr(), src.rhs.as_ptr());}
                }
            }
            {
                fn from(src: $inc<'_>) -> $out {
                    let mut res = $out::default();
                    res.assign(src);
                    res
                }
            }
        }
    )*);
    (
        rhs_owned
        $t1:ident, $t2:ident, $out:ident
        $($func:path; $inc:ident)*
    ) => ($(
        incomplete! {
            rhs_owned
            $inc
            $t1
            $t2
            $out
            {
                fn assign(&mut self, src: $inc<'_>) {
                    unsafe { $func(self.as_mut_ptr(), src.lhs.as_ptr(), src.rhs.as_ptr());}
                }
            }
            {
                fn from(src: $inc<'_>) -> $out {
                    let mut res = $out::default();
                    res.assign(src);
                    res
                }
            }
        }
    )*);
}

// aba, aaa
macro_rules! arith_assign {
    (
        $kw:ident // either None, rhs_prim, or OwnedIncomplete
        $inc:ident
        $t1:ident
        $t2:ident
        $op:ident {$meth:ident}
        $op_assign:ident {$meth_assign:ident}
        $op_from:ident {$meth_from:ident};
        {$($code_op_assign_own:tt)*}
        {$($code_op_assign:tt)*}
        {$($code_op_from_own:tt)*}
        {$($code_op_from:tt)*}
    ) => {
        impl $op_assign<$t2> for $t1 {
            $($code_op_assign_own)*
        }

        impl $op_assign<&$t2> for $t1 {
            $($code_op_assign)*
        }
        
        impl $op_from<$t2> for $t1 {
            $($code_op_from_own)*
        }
        
        impl $op_from<&$t2> for $t1 {
            $($code_op_from)*
        }

        impl $op<$t2> for $t1 {
            type Output = $t1;
            fn $meth(mut self, rhs: $t2) -> $t1 {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl $op<&$t2> for $t1 {
            type Output = $t1;
            fn $meth(mut self, rhs: &$t2) -> $t1 {
                self.$meth_assign(rhs);
                self
            }
        }

        arith_assign! {
            @incomplete 
            $kw 
            $inc 
            $op 
            $meth 
            $meth_from 
            $t1 
            $t2 
        }

    };// aaa
    (@incomplete 
        None 
        $inc:ident 
        $op:ident 
        $meth:ident 
        $meth_from:ident 
        $t1:ident 
        $t2:ident
    ) => {
        impl<'a> $op<$t2> for &'a $t1 {
            type Output = $t1;
            fn $meth(self, mut rhs: $t2) -> $t1 {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl<'a> $op<&'a $t2> for &'a $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $inc { lhs: self, rhs: rhs }
            }
        }
    };//aba, b prim
    (@incomplete 
        rhs_prim 
        $inc:ident 
        $op:ident 
        $meth:ident 
        $meth_from:ident 
        $t1:ident 
        $t2:ident
    ) => {
        impl<'a> $op<$t2> for &'a $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: $t2) -> $inc<'a> {
                $inc { lhs: self, rhs: rhs }
            }
        }
        
        impl<'a> $op<&'a $t2> for &'a $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $inc { lhs: self, rhs: *rhs }
            }
        }
    };// aba
    (@incomplete 
        $rhs_owned_inc:ident 
        $inc:ident 
        $op:ident 
        $meth:ident 
        $meth_from:ident 
        $t1:ident 
        $t2:ident
    ) => {
        impl<'a> $op<$t2> for &'a $t1 {
            type Output = $rhs_owned_inc<'a>;
            fn $meth(self, rhs: $t2) -> $rhs_owned_inc<'a> {
                $rhs_owned_inc { lhs: self, rhs: rhs }
            }
        }
        
        impl<'a> $op<&'a $t2> for &'a $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $inc { lhs: self, rhs: rhs }
            }
        }
    }

}

macro_rules! impl_arith_assign {
    (
        $t1:ident, $t2:ident
        $op:ident {$meth:ident}
        $op_assign:ident {$meth_assign:ident}
        $op_from:ident {$meth_from:ident}
        $func:path; $inc:ident
    ) => {
        arith_assign! {
            None
            $inc
            $t1
            $t2
            $op {$meth}
            $op_assign {$meth_assign}
            $op_from {$meth_from};
            {
                fn $meth_assign(&mut self, rhs: $t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                }
            }
            {
                fn $meth_assign(&mut self, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                }
            }
            {
                fn $meth_from(&mut self, lhs: $t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), self.as_ptr()); }
                }
            }
            {
                fn $meth_from(&mut self, lhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), lhs.as_ptr(), self.as_ptr()); }
                }
            }
        }
    };
    (
        $t1:ident, $t2:ident
        $op:ident {$meth:ident}
        $op_assign:ident {$meth_assign:ident}
        $op_from:ident {$meth_from:ident}
        $func:path; $inc:ident
        $func_from:path; $owned_inc:ident
    ) => {
        arith_assign! {
            $owned_inc
            $inc
            $t1
            $t2
            $op {$meth}
            $op_assign {$meth_assign}
            $op_from {$meth_from};
            {
                fn $meth_assign(&mut self, rhs: $t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                }
            }
            {
                fn $meth_assign(&mut self, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), rhs.as_ptr()); }
                }
            }
            {
                fn $meth_from(&mut self, lhs: $t2) {
                    unsafe { $func_from(self.as_mut_ptr(), lhs.as_ptr(), self.as_ptr()); }
                }
            }
            {
                fn $meth_from(&mut self, lhs: &$t2) {
                    unsafe { $func_from(self.as_mut_ptr(), lhs.as_ptr(), self.as_ptr()); }
                }
            }
        }

    };
    (
        $t1:ident, $cast:ident
        $op:ident {$meth:ident}
        $op_assign:ident {$meth_assign:ident}
        $op_from:ident {$meth_from:ident}
        $func:path;
        $func_from:path {
            $($inc:ident, $t2:ident)*
        }
    ) => ($(
        arith_assign! {
            rhs_prim
            $inc
            $t1
            $t2
            $op {$meth}
            $op_assign {$meth_assign}
            $op_from {$meth_from};
            {
                fn $meth_assign(&mut self, rhs: $t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), rhs as $cast); }
                }
            }
            {
                fn $meth_assign(&mut self, rhs: &$t2) {
                    unsafe { $func(self.as_mut_ptr(), self.as_ptr(), *rhs as $cast); }
                }
            }
            {
                fn $meth_from(&mut self, lhs: $t2) {
                    unsafe { $func_from(self.as_mut_ptr(), lhs as $cast, self.as_ptr()); }
                }
            }
            {
                fn $meth_from(&mut self, lhs: &$t2) {
                    unsafe { $func_from(self.as_mut_ptr(), *lhs as $cast, self.as_ptr()); }
                }
            }
        }
    )*)
}

macro_rules! arith_rhs_assign {
    (
        $kw:ident // either lhs_prim or (LhsOwned)Incomplete
        $inc:ident
        $t1:ident
        $t2:ident
        $op:ident {$meth:ident}
        $op_from:ident {$meth_from:ident}
    ) => {
        impl $op<$t2> for $t1 {
            type Output = $t2;
            fn $meth(self, mut rhs: $t2) -> $t2 {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl $op<$t2> for &$t1 {
            type Output = $t2;
            fn $meth(self, mut rhs: $t2) -> $t2 {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        arith_rhs_assign! {@incomplete 
            $kw
            $inc
            $t1
            $t2
            $op {$meth}
            $op_from {$meth_from}
        }
    };
    (@incomplete 
        lhs_prim
        $inc:ident
        $t1:ident
        $t2:ident
        $op:ident {$meth:ident}
        $op_from:ident {$meth_from:ident}
    ) => {
        impl<'a> $op<&'a $t2> for $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $inc { lhs: self, rhs: rhs}
            }
        }
        
        impl<'a> $op<&'a $t2> for &'a $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $inc { lhs: *self, rhs: rhs}
            }
        }


    };
    (@incomplete 
        $lhs_owned_inc:ident
        $inc:ident
        $t1:ident
        $t2:ident
        $op:ident {$meth:ident}
        $op_from:ident {$meth_from:ident}
    ) => {
        
        impl<'a> $op<&'a $t2> for $t1 {
            type Output = $lhs_owned_inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $lhs_owned_inc<'a> {
                $lhs_owned_inc { lhs: self, rhs: rhs}
            }
        }
        
        impl<'a> $op<&'a $t2> for &'a $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $inc { lhs: self, rhs: rhs}
            }
        }

    }
}


macro_rules! impl_arith_rhs_assign {
    (
        $t2:ident
        $op:ident {$meth:ident}
        $op_from:ident {$meth_from:ident}
        {
            $($inc:ident, $t1:ident)*
        }
    ) => ($(
        arith_rhs_assign! {
            lhs_prim
            $inc
            $t1
            $t2
            $op {$meth}
            $op_from {$meth_from}
        }
    )*);
    (
        $t1:ident, $t2:ident
        $op:ident {$meth:ident}
        $op_from:ident {$meth_from:ident}
        $inc:ident, $owned_inc:ident
    ) => (
        arith_rhs_assign! {
            $owned_inc
            $inc
            $t1
            $t2
            $op {$meth}
            $op_from {$meth_from}
        }
    )
}

macro_rules! arith_no_assign {
    (
        $inc:ident
        $lhs_owned_inc:ident
        $rhs_owned_inc:ident
        $both_owned_inc:ident
        $t1:ident
        $t2:ident
        $op:ident {$meth:ident}
    ) => {
        impl $op<$t2> for $t1 {
            type Output = $t2;
            fn $meth(self, mut rhs: $t2) -> $t2 {
                $both_owned_inc { lhs: self, rhs: rhs }
            }
        }
        
        arith_no_assign! {@incomplete 
            $inc
            $lhs_owned_inc
            $rhs_owned_inc
            $both_owned_inc
            $t1
            $t2
            $op {$meth}
        }
    };
    (@incomplete 
        None
        $lhs_owned_inc:ident
        None
        $both_owned_inc:ident
        $t1:ident
        $t2:ident
        $op:ident {$meth:ident}
    ) => {
        
        impl $op<$t2> for &$t1 {
            type Output = $t2;
            fn $meth(self, mut rhs: $t2) -> $t2 {
                $both_owned_inc { lhs: *self, rhs: rhs }
            }
        }

        impl<'a> $op<&'a $t2> for $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $lhs_owned_inc { lhs: self, rhs: rhs}
            }
        }
        
        impl<'a> $op<&'a $t2> for &'a $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $lhs_owned_inc { lhs: *self, rhs: rhs}
            }
        }
    };
    (@incomplete 
        None
        None
        $rhs_owned_inc:ident
        $both_owned_inc:ident
        $t1:ident
        $t2:ident
        $op:ident {$meth:ident}
    ) => {
        
        impl $op<$t2> for &$t1 {
            type Output = $t2;
            fn $meth(self, mut rhs: $t2) -> $t2 {
                $rhs_owned_inc { lhs: self, rhs: rhs }
            }
        }

        impl<'a> $op<&'a $t2> for $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $both_owned_inc { lhs: self, rhs: *rhs}
            }
        }
        
        impl<'a> $op<&'a $t2> for &'a $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $rhs_owned_inc { lhs: self, rhs: *rhs}
            }
        }
    };
    (@incomplete 
        $inc:ident
        $lhs_owned_inc:ident
        $rhs_owned_inc:ident
        $both_owned_inc:ident
        $t1:ident
        $t2:ident
        $op:ident {$meth:ident}
    ) => {
        impl $op<$t2> for &$t1 {
            type Output = $t2;
            fn $meth(self, mut rhs: $t2) -> $t2 {
                $rhs_owned_inc { lhs: self, rhs: rhs }
            }
        }

        impl<'a> $op<&'a $t2> for $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $lhs_owned_inc { lhs: self, rhs: rhs}
            }
        }
        
        impl<'a> $op<&'a $t2> for &'a $t1 {
            type Output = $inc<'a>;
            fn $meth(self, rhs: &'a $t2) -> $inc<'a> {
                $inc { lhs: self, rhs: rhs}
            }
        }
    }
}
macro_rules! from_assign {
    ($src:ty => $dst:ty) => {
    };

    /*
    ($src:ty => $dst1:ty, $dst2:ty) => {
        impl From<$Src> for ($Dst1, $Dst2) {
            #[inline]
            fn from(src: $Src) -> Self {
                let mut dst = Self::default();
                Assign::assign(&mut (&mut dst.0, &mut dst.1), src);
                dst
            }
        }

        impl Complete for $Src {
            type Completed = ($Dst1, $Dst2);
            #[inline]
            fn complete(self) -> ($Dst1, $Dst2) {
                <($Dst1, $Dst2)>::from(self)
            }
        }
    };

    ($src:ty => $dst1:ty, $dst2:ty, $dst3:ty) => {
        impl From<$Src> for ($Dst1, $Dst2, $Dst3) {
            #[inline]
            fn from(src: $Src) -> Self {
                let mut dst = Self::default();
                Assign::assign(&mut (&mut dst.0, &mut dst.1, &mut dst.2), src);
                dst
            }
        }

        impl Complete for $Src {
            type Completed = ($Dst1, $Dst2, $Dst3);
            #[inline]
            fn complete(self) -> ($Dst1, $Dst2, $Dst3) {
                <($Dst1, $Dst2, $Dst3)>::from(self)
            }
        }
    };*/
}

macro_rules! arith_unary {
    (
        $op:ident { $meth:ident }
        impl $op_assign:ident for $t:ident {
            fn $meth_assign:ident(&mut $self:ident) {
                $($code:tt)*
            }
        }
    ) => {
        impl $op for $t {
            type Output = $t;
            #[inline]
            fn $meth(mut $self) -> $t {
                $self.$meth_assign();
                $self
            }
        }

        /* TODO
        impl<'a> $op for &'a $t {
            type Output = Expr<'a, $t>;
            #[inline]
            fn $meth($self) -> Expr<'a, $t> {
                Expr {
                    phantom: std::marker::PhantomData::<$t>,
                    expr: ExprNode::UnaryExpr {
                        op: UnaryOp::$op,
                        child: Box::new(ExprNode::$t($self))
                    }
                }
            }
        }*/
        
        impl $op_assign for $t {
            #[inline]
            fn $meth_assign(&mut $self) {
                $($code)*
            }
        }
        
        impl $op_assign for &mut $t {
            #[inline]
            fn $meth_assign(&mut $self) {
                $($code)*
            }
        }
    };
    // no assignment
    (
        impl $op:ident for $t:ident {
            type Output = $out:ty;
            fn $meth:ident($self:ident) -> $ret:ty {
                $($code:tt)*
            }
        }
    ) => {
        impl $op for $t {
            type Output = $t;
            #[inline]
            fn $meth(mut $self) -> $t {
                $self.$meth_assign();
                $self
            }
        }
        
        /* TODO
        impl<'a> $op for &'a $t {
            type Output = Expr<'a, $t>;
            #[inline]
            fn $meth($self) -> Expr<'a, $t> {
                Expr {
                    phantom: std::marker::PhantomData::<$t>,
                    expr: ExprNode::UnaryExpr {
                        op: UnaryOp::$op,
                        child: Box::new(ExprNode::$t($self))
                    }
                }
            }
        }*/
    };
}

/*
 * arith_binary_assign
 *  op_assign(a, b)
 *  op_assign(a, &b)
 *  op(a, b) = op_assign(a, b); a
 *  op(a, &b) = op_assign(a, &b); a
 *  op(&a, b) = inc(&a, b)
 *      self then op(&a, b) = op_from(b, &a); b
 *  op(&a, &b) = inc(&a, &b)
 *
 *  incomplete(&a, &b)
 *
 */

macro_rules! arith_binary {
    (
        $op:ident { $meth:ident }
        impl $op_assign_own:ident<$t2_assign_own:ident> for $t1_assign_own:ident {
            fn $meth_assign_own:ident(&mut $self_assign_own:ident, $rhs_own:ident : $rhs_own_ty:ty) {
                $($code_assign_own:tt)*
            }
        }
        impl $op_assign:ident<&$t2_assign:ident> for $t1_assign:ident {
            fn $meth_assign:ident(&mut $self_assign:ident, $rhs:ident : &$rhs_ty:ty) {
                $($code_assign:tt)*
            }
        }
        impl $op_from_own:ident<$t2_from_own:ident> for $t1_from_own:ident {
            fn $meth_from_own:ident(&mut $self_from_own:ident, $lhs_own:ident : $lhs_own_ty:ty) {
                $($code_from_own:tt)*
            }
        }
        impl $op_from:ident<&$t2_from:ident> for $t1_from:ident {
            fn $meth_from:ident(&mut $self_from:ident, $lhs:ident : &$lhs_ty:ty) {
                $($code_from:tt)*
            }
        }
    ) => {
        impl $op<$t2_assign_own> for $t1_assign_own {
            type Output = $t1_assign_own;
            #[inline]
            fn $meth(mut $self_assign_own, $rhs_own: $rhs_own_ty) -> $t1_assign_own {
                $self_assign_own.$meth_assign_own($rhs_own);
                $self_assign_own
            }
        }
        
        impl $op_assign_own<$t2_assign_own> for $t1_assign_own {
            #[inline]
            fn $meth_assign_own(&mut $self_assign_own, $rhs_own: $rhs_own_ty) {
                $($code_assign_own)*
            }
        }
        
        impl $op<&$t2_assign> for $t1_assign {
            type Output = $t1_assign;
            #[inline]
            fn $meth(mut $self_assign, $rhs: &$rhs_ty) -> $t1_assign {
                $self_assign.$meth_assign($rhs);
                $self_assign
            }
        }

        impl $op_assign<&$t2_assign> for $t1_assign {
            #[inline]
            fn $meth_assign(&mut $self_assign, $rhs: &$rhs_ty) {
                $($code_assign)*
            }
        }
        
        /*
        impl $op_assign<$t2_assign> for &mut $t1_assign {
            #[inline]
            fn $meth_assign(&mut $self_assign, $rhs: $rhs_ty) {
                $self_assign.$meth_assign(&$rhs);
            }
        }
        
        impl $op_assign<&$t2_assign> for &mut $t1_assign {
            #[inline]
            fn $meth_assign(&mut $self_assign, $rhs: &$rhs_ty) {
                $($code_assign)*
            }
        }*/
        
        // TODO: move?
        impl $op_from_own<$t2_from_own> for $t1_from_own {
            #[inline]
            fn $meth_from_own(&mut $self_from_own, $lhs_own: $lhs_own_ty) {
                $($code_from_own)*
            }
        }
        
        impl $op_from<&$t2_from> for $t1_from {
            #[inline]
            fn $meth_from(&mut $self_from, $lhs: &$lhs_ty) {
                $($code_from)*
            }
        }
    };
    (
        $op:ident { $meth:ident }
        impl $op_assign:ident<&$t2_assign:ident> for $t1_assign:ident {
            fn $meth_assign:ident(&mut $self_assign:ident, $rhs:ident : &$rhs_ty:ty) {
                $($code_assign:tt)*
            }
        }
        impl $op_from:ident<&$t2_from:ident> for $t1_from:ident {
            fn $meth_from:ident(&mut $self_from:ident, $lhs:ident : &$lhs_ty:ty) {
                $($code_from:tt)*
            }
        }
    ) => {
        arith_binary! {
            $op { $meth }
            impl $op_assign<$t2_assign> for $t1_assign {
                fn $meth_assign(&mut $self_assign, $rhs: $rhs_ty) {
                    $self_assign.$meth_assign(&$rhs);
                }
            }
            impl $op_assign<&$t2_assign> for $t1_assign {
                fn $meth_assign(&mut $self_assign, $rhs: &$rhs_ty) {
                    $($code_assign)*
                }
            }
            impl $op_from<$t2_from> for $t1_from {
                fn $meth_from(&mut $self_from, $lhs: $lhs_ty) {
                    $self_from.$meth_from(&$lhs);
                }
            }
            impl $op_from<&$t2_from> for $t1_from {
                fn $meth_from(&mut $self_from, $lhs: &$lhs_ty) {
                    $($code_from)*
                }
            }
        }
    }

}

macro_rules! arith_binary_self {
    (
        $op:ident { $meth:ident }
        impl $op_assign:ident<&$t2_op_assign:ident> for $t1_op_assign:ident {
            fn $meth_assign:ident(&mut $self_op_assign:ident, $rhs:ident : &$rhs_ty:ty) {
                $($code_op_assign:tt)*
            }
        }
        impl $op_from:ident<&$t2_op_from:ident> for $t1_op_from:ident {
            fn $meth_from:ident(&mut $self_op_from:ident, $lhs:ident : &$lhs_ty:ty) {
                $($code_op_from:tt)*
            }
        }
        impl Assign<$inc_assign:ident<'_>> for $t_assign:ident {
            fn assign(&mut $self_assign:ident, $inc:ident: $inc_ty:ident<'_>) {
                $($code_assign:tt)*
            }
        }
        impl From<$inc_from:ident<'_>> for $t_from:ident {
            fn from($src:ident: $src_ty:ident<'_>) -> $out:ident {
                $($code_from:tt)*
            }
        }
    ) => { 
        arith_binary! {
            $op { $meth }
            impl $op_assign<&$t2_op_assign> for $t1_op_assign {
                fn $meth_assign(&mut $self_op_assign, $rhs: &$rhs_ty) {
                    $($code_op_assign)*
                }
            }
            impl $op_from<&$t2_op_from> for $t1_op_from {
                fn $meth_from(&mut $self_op_from, $lhs: &$lhs_ty) {
                    $($code_op_from)*
                }
            }
        }

        impl $op<$t2_op_assign> for &$t1_op_assign {
            type Output = $t1_op_assign;
            #[inline]
            fn $meth($self_op_assign, mut $rhs: $rhs_ty) -> $t1_op_assign {
                $rhs.$meth_from($self_op_assign);
                $rhs
            }
        }

        impl<'a> $op<&'a $t2_op_assign> for &'a $t1_op_assign {
            type Output = $inc_assign<'a>;
            #[inline]
            fn $meth($self_op_assign, $rhs: &'a $rhs_ty) -> $inc_assign<'a> {
                $inc_assign { lhs: $self_op_assign, rhs: $rhs }
            }
        }
        
        #[derive(Debug)]
        pub struct $inc_assign<'a> {
            lhs: &'a $t1_op_assign,
            rhs: &'a $t2_op_assign,
        }

        impl Assign<$inc_assign<'_>> for $t_assign {
            #[inline]
            fn assign(&mut $self_assign, $inc: $inc_ty<'_>) {
                $($code_assign)*
            }
        }
        
        impl From<$inc_from<'_>> for $t_from {
            #[inline]
            fn from($src: $src_ty<'_>) -> $out {
                $($code_from)*
            }
        }

        impl Complete for $inc_from<'_> {
            type Completed = $out;
            #[inline]
            fn complete(self) -> $out {
                <$out>::from(self)
            }
        }
    };
    /*
    // no assignment
    (
        impl $op:ident<$t2:ident> for $t1:ident {
            type Output = $out:ident;
            fn $meth:ident($self:ident, $rhs:ident : $rhs_ty:ty) -> $out_ty:ident {
                $($code:tt)*
            }
        }
        impl Assign<$inc_assign:ident<'_>> for $t_assign:ident {
            fn assign(&mut $self_assign:ident, $inc:ident: $inc_ty:ident<'_>) {
                $($code_assign:tt)*
            }
        }
        impl From<$inc_from:ident<'_>> for $t_from:ident {
            fn from($src:ident: $src_ty:ident<'_>) -> $out:ident {
                $($code_from:tt)*
            }
        }
    ) => {}*/
}

macro_rules! arith_binary_rprim {
    (
        $op:ident { $meth:ident }
        impl $op_assign_own:ident<$t2_assign_own:ident> for $t1_assign_own:ident {
            fn $meth_assign_own:ident(&mut $self_op_assign_own:ident, $rhs_own:ident: $rhs_own_ty:ty) {
                $($code_op_assign_own:tt)*
            }
        }
        impl $op_assign:ident<&$t2_assign:ident> for $t1_assign:ident {
            fn $meth_assign:ident(&mut $self_op_assign:ident, $rhs:ident : &$rhs_ty:ty) {
                $($code_op_assign:tt)*
            }
        }
        impl $op_from_own:ident<$t2_from_own:ident> for $t1_from_own:ident {
            fn $meth_from_own:ident(&mut $self_from_own:ident, $lhs_own:ident : $lhs_own_ty:ty) {
                $($code_op_from_own:tt)*
            }
        }
        impl $op_from:ident<&$t2_from:ident> for $t1_from:ident {
            fn $meth_from:ident(&mut $self_from:ident, $lhs:ident : &$lhs_ty:ty) {
                $($code_op_from:tt)*
            }
        }
        impl Assign<$inc_assign:ident<'_>> for $t_assign:ident {
            fn assign(&mut $self_assign:ident, $inc:ident: $inc_ty:ident<'_>) {
                $($code_assign:tt)*
            }
        }
        impl From<$inc_from:ident<'_>> for $t_from:ident {
            fn from($src:ident: $src_ty:ident<'_>) -> $out:ident {
                $($code_from:tt)*
            }
        }
    ) => { 
        arith_binary! {
            $op { $meth }
            impl $op_assign_own<$t2_assign_own> for $t1_assign_own {
                fn $meth_assign_own(&mut $self_op_assign_own, $rhs_own: $rhs_own_ty) {
                    $($code_op_assign_own)*
                }
            }
            impl $op_assign<&$t2_assign> for $t1_assign {
                fn $meth_assign(&mut $self_op_assign, $rhs: &$rhs_ty) {
                    $($code_op_assign)*
                }
            }
            impl $op_from_own<$t2_from_own> for $t1_from_own {
                fn $meth_from_own(&mut $self_from_own, $lhs_own: $lhs_own_ty) {
                    $($code_op_from_own)*
                }
            }
            impl $op_from<&$t2_from> for $t1_from {
                fn $meth_from(&mut $self_from, $lhs: &$lhs_ty) {
                    $($code_op_from)*
                }
            }
        }
        
        impl<'a> $op<$t2_assign_own> for &'a $t1_assign_own {
            type Output = $inc_assign<'a>;
            #[inline]
            fn $meth($self_op_assign_own, $rhs_own: $rhs_own_ty) -> $inc_assign<'a> {
                $inc_assign { lhs: $self_op_assign_own, rhs: $rhs_own }
            }
        }

        impl<'a> $op<&'a $t2_assign> for &'a $t1_assign {
            type Output = $inc_assign<'a>;
            #[inline]
            fn $meth($self_op_assign, $rhs: &'a $rhs_ty) -> $inc_assign<'a> {
                $inc_assign { lhs: $self_op_assign, rhs: *$rhs }
            }
        }
        
        #[derive(Debug)]
        pub struct $inc_assign<'a> {
            lhs: &'a $t1_assign,
            rhs: $t2_assign,
        }

        impl Assign<$inc_assign<'_>> for $t_assign {
            #[inline]
            fn assign(&mut $self_assign, $inc: $inc_ty<'_>) {
                $($code_assign)*
            }
        }
        
        impl From<$inc_from<'_>> for $t_from {
            #[inline]
            fn from($src: $src_ty<'_>) -> $out {
                $($code_from)*
            }
        }

        impl Complete for $inc_from<'_> {
            type Completed = $out;
            #[inline]
            fn complete(self) -> $out {
                <$out>::from(self)
            }
        }
       
    }
}

macro_rules! arith_binary_lprim {
    (
        $op_from:ident {$meth_from:ident}
        /*
        impl $op:ident<$t2:ident> for $t1:ident {
            type Output = $out:ident;
            fn $meth:ident($self:ident, $rhs:ident : $rhs_ty:ty) -> $out_ty:ident {
                $($code:tt)*
            }
        }*/
        impl Assign<$inc_assign:ident<'_>> for $t_assign:ident {
            fn assign(&mut $self_assign:ident, $assign_arg:ident: $assign_arg_ty:ident<'_>) {
                $($code_assign:tt)*
            }
        }
        impl From<$inc_from:ident<'_>>  for $t_from:ident {
            fn from($from_arg:ident: $from_arg_ty:ident<'_>) -> $out_from_ty:ident {
                $($code_from:tt)*
            }
        }
    ) => { 
        
        impl $op<$t2> for $t1 {
            type Output = $out;
            #[inline]
            fn $meth($self, mut $rhs: $rhs_ty) -> $out_ty {
                $rhs.$meth_from($self);
                $rhs
            }
        }
        
        impl<'a> $op<&'a $t2> for $t1 {
            type Output = $inc<'a>;
            #[inline]
            fn $meth($self, $rhs: &'a $rhs_ty) -> $inc<'a> {
                $inc { lhs: $self, rhs: $rhs }
            }
        }
        
        impl $op<$t2> for &$t1 {
            type Output = $out;
            #[inline]
            fn $meth($self, mut $rhs: $rhs_ty) -> $out_ty {
                $rhs.$meth_from($self);
                $rhs
            }
        }

        impl<'a> $op<&'a $t2> for &'a $t1 {
            type Output = $inc<'a>;
            #[inline]
            fn $meth($self, $rhs: &'a $rhs_ty) -> $inc<'a> {
                $inc { lhs: *$self, rhs: $rhs }
            }
        }
        
        #[derive(Debug)]
        pub struct $inc<'a> {
            lhs: $t1,
            rhs: &'a $t2,
        }

        impl Assign<$inc<'_>> for $out {
            #[inline]
            fn assign(&mut $self, $inc: $inc_ty<'_>) {
                $($code_assign)*
            }
        }
        
        impl From<$inc_from<'_>> for $t_from {
            #[inline]
            fn from($src: $src_ty<'_>) -> $out {
                $($code_from)*
            }
        }

        impl Complete for $inc<'_> {
            type Completed = $out;
            #[inline]
            fn complete(self) -> $out {
                <$out>::from(self)
            }
        }

    }
}

*/
