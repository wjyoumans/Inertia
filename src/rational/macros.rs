use crate::rational::src::Rational;

#[macro_export]
macro_rules! rat {
    ($arg:expr) => {
        Rational::from($arg)
    };
    ($num:expr, $den:expr) => {
        Rational::from(vec![$num, $den].as_slice())
    }
}
