use std::{
    ffi::{CStr, CString},
    ops::Deref,
    str::{from_utf8, from_utf8_unchecked, Utf8Error},
};

#[derive(Debug)]
#[repr(transparent)]
pub struct NullTerminatedStr(CStr);

impl NullTerminatedStr {
    pub fn as_c_str(&self) -> &CStr {
        &self.0
    }

    /// # Safety
    ///
    /// The `CStr` (excluding the null byte)
    /// must be valid utf-8 str.
    pub const unsafe fn from_cstr_unchecked(cstr: &CStr) -> &Self {
        // Safety: NullTerminatedStr is transparent
        // newtype of CStr
        &*(cstr as *const CStr as *const Self)
    }

    pub fn from_cstr(cstr: &CStr) -> Result<&Self, Utf8Error> {
        from_utf8(cstr.to_bytes())?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }
}

const fn report_err() -> u8 {
    const EMPTY_ARR: [u8; 0] = [];
    #[allow(unconditional_panic)]
    EMPTY_ARR[0]
}

impl NullTerminatedStr {
    /// This function creates a `NullTerminatedStr`
    /// from `s` which must have only one null byte
    /// at the end of the string.
    ///
    /// If not, then this function would panic.
    pub const fn from_const_str(s: &str) -> &Self {
        let bytes = s.as_bytes();

        if bytes.is_empty() {
            report_err();
        }

        let mut i = 0;
        let n = bytes.len() - 1;

        // Check last byte is null byte
        if bytes[n] != b'\0' {
            report_err();
        }

        // Ensure there is no internal null byte.
        while i < n {
            if bytes[i] == b'\0' {
                report_err();
            }
            i += 1;
        }

        unsafe { Self::from_cstr_unchecked(CStr::from_bytes_with_nul_unchecked(bytes)) }
    }
}

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

impl Deref for NullTerminatedStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { from_utf8_unchecked(self.0.to_bytes()) }
    }
}

#[derive(Clone, Debug)]
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
