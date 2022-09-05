use std::{ffi::CString, ops::Deref};

mod borrowed;
pub use borrowed::NullTerminatedStr;

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullTerminatedString(CString);

impl From<&str> for NullTerminatedString {
    fn from(s: &str) -> Self {
        let buf = s.bytes().filter(|byte| *byte != b'\0').collect::<Vec<_>>();
        // from_vec_unchecked appends the trailing '\0'
        //
        // Safety:
        // All '\0' is removed before passing in.
        Self(unsafe { CString::from_vec_unchecked(buf) })
    }
}

impl From<String> for NullTerminatedString {
    fn from(s: String) -> Self {
        let mut buf = s.into_bytes();
        buf.retain(|byte| *byte != b'\0');
        // from_vec_unchecked appends the trailing '\0'
        //
        // Safety:
        // All '\0' is removed before passing in.
        Self(unsafe { CString::from_vec_unchecked(buf) })
    }
}

impl Deref for NullTerminatedString {
    type Target = NullTerminatedStr;

    fn deref(&self) -> &Self::Target {
        unsafe { NullTerminatedStr::from_cstr_unchecked(self.0.as_c_str()) }
    }
}
