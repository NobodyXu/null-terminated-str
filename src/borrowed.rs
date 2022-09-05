use std::{
    cmp::Ordering,
    ffi::CStr,
    fmt,
    ops::Deref,
    str::{from_utf8, from_utf8_unchecked, Utf8Error},
};

use super::NullTerminatedString;

const fn report_err() -> &'static NullTerminatedStr {
    const EMPTY_ARR: [&NullTerminatedStr; 0] = [];
    #[allow(unconditional_panic)]
    EMPTY_ARR[0]
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NullTerminatedStr(CStr);

impl NullTerminatedStr {
    pub const fn as_c_str(&self) -> &CStr {
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

    /// Return `true` if the str has and only has one null byte
    /// at the end of the string.
    const fn is_null_terminated(s: &str) -> bool {
        let bytes = s.as_bytes();

        if bytes.is_empty() {
            return false;
        }

        let mut i = 0;
        let n = bytes.len() - 1;

        // Check last byte is null byte
        if bytes[n] != b'\0' {
            return false;
        }

        // Ensure there is no internal null byte.
        while i < n {
            if bytes[i] == b'\0' {
                return false;
            }
            i += 1;
        }

        true
    }

    /// This function creates a `NullTerminatedStr`
    /// from `s` which must have only one null byte
    /// at the end of the string.
    ///
    /// If not, then this function would panic.
    pub const fn from_const_str(s: &str) -> &Self {
        if let Some(null_str) = Self::try_from_str(s) {
            null_str
        } else {
            report_err()
        }
    }

    /// This function tries creates a `NullTerminatedStr`
    /// from `s` which must have only one null byte
    /// at the end of the string.
    ///
    /// If not, then this function would return `None`.
    ///
    /// ```rust
    /// use null_terminated_str::NullTerminatedStr;
    /// use std::ops::Deref;
    ///
    /// // Empty string is rejected
    /// assert_eq!(
    ///     NullTerminatedStr::try_from_str(""),
    ///     None,
    /// );
    ///
    /// // String without null byte is rejected
    /// assert_eq!(
    ///     NullTerminatedStr::try_from_str("ha"),
    ///     None,
    /// );
    ///
    /// // String with internal null byte is rejected
    /// assert_eq!(
    ///     NullTerminatedStr::try_from_str("h\0a\0"),
    ///     None,
    /// );
    ///
    /// // String without trailing null byte is also rejected
    /// assert_eq!(
    ///     NullTerminatedStr::try_from_str("h\0a"),
    ///     None,
    /// );
    /// ```
    pub const fn try_from_str(s: &str) -> Option<&Self> {
        if Self::is_null_terminated(s) {
            Some(unsafe {
                Self::from_cstr_unchecked(CStr::from_bytes_with_nul_unchecked(s.as_bytes()))
            })
        } else {
            None
        }
    }
}

impl Deref for NullTerminatedStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { from_utf8_unchecked(self.0.to_bytes()) }
    }
}

impl PartialEq<str> for NullTerminatedStr {
    fn eq(&self, other: &str) -> bool {
        self.deref().eq(other)
    }
}

impl PartialEq<NullTerminatedStr> for str {
    fn eq(&self, other: &NullTerminatedStr) -> bool {
        self.eq(other.deref())
    }
}

impl PartialOrd<str> for NullTerminatedStr {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        self.deref().partial_cmp(other)
    }
}

impl PartialOrd<NullTerminatedStr> for str {
    fn partial_cmp(&self, other: &NullTerminatedStr) -> Option<Ordering> {
        self.partial_cmp(other.deref())
    }
}

impl fmt::Display for NullTerminatedStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl<'a> TryFrom<&'a CStr> for &'a NullTerminatedStr {
    type Error = Utf8Error;

    fn try_from(cstr: &'a CStr) -> Result<Self, Self::Error> {
        NullTerminatedStr::from_cstr(cstr)
    }
}

impl ToOwned for NullTerminatedStr {
    type Owned = NullTerminatedString;

    fn to_owned(&self) -> Self::Owned {
        let cstring = self.as_c_str().to_owned();
        unsafe { NullTerminatedString::from_cstring_unchecked(cstring) }
    }
}

impl<'a> From<&'a NullTerminatedStr> for &'a CStr {
    fn from(null_str: &'a NullTerminatedStr) -> Self {
        null_str.as_c_str()
    }
}

impl AsRef<NullTerminatedStr> for NullTerminatedStr {
    fn as_ref(&self) -> &NullTerminatedStr {
        self
    }
}

impl AsRef<str> for NullTerminatedStr {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl AsRef<CStr> for NullTerminatedStr {
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}
