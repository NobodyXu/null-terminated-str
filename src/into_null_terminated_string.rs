use std::borrow::Cow;

use super::{const_null_terminated_str, NullTerminatedStr, NullTerminatedString};

pub trait IntoNullTerminatedString<'a> {
    fn into_null_terminated_string(self) -> Cow<'a, NullTerminatedStr>;
}

impl<'a> IntoNullTerminatedString<'a> for &'a str {
    fn into_null_terminated_string(self) -> Cow<'a, NullTerminatedStr> {
        if self.is_empty() {
            Cow::Borrowed(const_null_terminated_str!(""))
        } else {
            NullTerminatedStr::try_from_str(self)
                .map(Cow::Borrowed)
                .unwrap_or_else(|| Cow::Owned(NullTerminatedString::from(self)))
        }
    }
}

impl IntoNullTerminatedString<'static> for String {
    fn into_null_terminated_string(self) -> Cow<'static, NullTerminatedStr> {
        Cow::Owned(NullTerminatedString::from(self))
    }
}

impl<'a> IntoNullTerminatedString<'a> for &'a NullTerminatedStr {
    fn into_null_terminated_string(self) -> Cow<'a, NullTerminatedStr> {
        Cow::Borrowed(self)
    }
}

impl<'a> IntoNullTerminatedString<'a> for NullTerminatedString {
    fn into_null_terminated_string(self) -> Cow<'a, NullTerminatedStr> {
        Cow::Owned(self)
    }
}
