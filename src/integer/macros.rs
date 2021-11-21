
#[macro_export]
macro_rules! int {
    ($arg:expr) => {
        Integer::from($arg)
    }
}
