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

    pub fn from_cstring_lossy(cstring: CString) -> Self {
        Self::from_cstring(cstring).unwrap_or_else(|NullStringFromUtf8Error { cstring, .. }| {
            // bytes without null byte
            let bytes = cstring.into_bytes();

            // This would replace any invalid utf-8 sequence with
            // `std::char::REPLACEMENT_CHARACTER`, which does not
            // contain null byte.
            let string = String::from_utf8_lossy(&bytes).into_owned();

            // Convert it back into bytes
            let bytes = string.into_bytes();

            // from_vec_unchecked appends the trailing '\0'
            //
            // Safety:
            //
            // The string cannot have any null byte.
            Self(unsafe { CString::from_vec_unchecked(bytes) })
        })
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

impl From<NullTerminatedString> for String {
    fn from(s: NullTerminatedString) -> String {
        // bytes without trailing null byte
        let bytes = s.0.into_bytes();
        // Safety:
        //
        // NullTerminatedString contains valid utf-8 string,
        // excluding the trailing null byte.
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

impl From<NullTerminatedString> for CString {
    fn from(s: NullTerminatedString) -> CString {
        s.0
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
