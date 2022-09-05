mod borrowed;
pub use borrowed::NullTerminatedStr;

mod owned;
pub use owned::{NullStringFromUtf8Error, NullTerminatedString};

mod into_null_terminated_string;
pub use into_null_terminated_string::IntoNullTerminatedString;

/// Create a null-terminated utf-8 str as an rvalue.
/// Appends a NUL byte to the passed string.
///
/// ```rust
/// use null_terminated_str::{
///     const_null_terminated_str,
///     NullTerminatedStr
/// };
/// use std::ops::Deref;
///
/// const S: &NullTerminatedStr = const_null_terminated_str!("Hello, World!");
/// assert_eq!(S.deref(), "Hello, World!");
/// ```
///
/// If the `str` contains NULL bytes, then the compilation
/// would fail.
///
/// ```rust,compile_fail
/// use null_terminated_str::{
///     const_null_terminated_str,
///     NullTerminatedStr
/// };
/// use std::ops::Deref;
///
/// const S: &NullTerminatedStr = const_null_terminated_str!("Hello,\0 World!");
/// assert_eq!(S.deref(), "Hello, World!");
/// ```
#[macro_export]
macro_rules! const_null_terminated_str {
    ($strval:expr) => {
        $crate::NullTerminatedStr::from_const_str(concat!($strval, "\0"))
    };
}
