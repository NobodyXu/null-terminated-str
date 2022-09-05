use std::{ffi::CString, ops::Deref};

use super::NullTerminatedStr;

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
