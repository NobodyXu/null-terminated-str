use std::{
    cmp::Ordering,
    ffi::CStr,
    fmt,
    ops::Deref,
    str::{from_utf8, from_utf8_unchecked, Utf8Error},
};

const fn report_err() -> u8 {
    const EMPTY_ARR: [u8; 0] = [];
    #[allow(unconditional_panic)]
    EMPTY_ARR[0]
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
