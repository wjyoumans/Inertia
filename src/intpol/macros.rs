
#[macro_export]
macro_rules! intpol {
    ($($coeff:expr),+) => (
        IntPol::from(vec![$($coeff),+].as_slice())
    )
}
