use std::{error::Error, ffi::CString, fmt, ops::Deref, str::Utf8Error};

use super::NullTerminatedStr;

#[derive(Clone, Debug)]
pub struct NullStringFromUtf8Error {
    cstring: CString,
    utf8_err: Utf8Error,
}

impl NullStringFromUtf8Error {
    pub fn into_inner(self) -> (CString, Utf8Error) {
        (self.cstring, self.utf8_err)
    }
}

impl fmt::Display for NullStringFromUtf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.utf8_err.fmt(f)
    }
}

impl Error for NullStringFromUtf8Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.utf8_err)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullTerminatedString(CString);

impl NullTerminatedString {
    /// # Safety
    ///
    /// `cstring` (excluding the null byte) must be valid utf-8 str.
    pub const unsafe fn from_cstring_unchecked(cstring: CString) -> Self {
        Self(cstring)
    }

    pub fn from_cstring(cstring: CString) -> Result<Self, NullStringFromUtf8Error> {
        if let Err(utf8_err) = NullTerminatedStr::from_cstr(&cstring) {
            Err(NullStringFromUtf8Error { cstring, utf8_err })
        } else {
            Ok(Self(cstring))
        }
    }
}

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

impl fmt::Display for NullTerminatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}
