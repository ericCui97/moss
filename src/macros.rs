#[macro_export]
macro_rules! is_newline {
    ($ch:expr) => {
        ($ch == '\n') || ($ch == '\r')
    };
}