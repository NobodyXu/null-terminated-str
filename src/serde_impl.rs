use std::ops::Deref;

use serde::{
    de::{Deserialize, Deserializer, Error, Unexpected},
    Serialize, Serializer,
};

use super::{NullTerminatedStr, NullTerminatedString};

impl Serialize for NullTerminatedStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.as_c_str().to_bytes_with_nul())
    }
}

impl Serialize for NullTerminatedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.deref().serialize(serializer)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a NullTerminatedStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        NullTerminatedStr::try_from_str(s).ok_or_else(|| {
            D::Error::invalid_value(Unexpected::Str(s), &"Expected null terminated utf-8 str")
        })
    }
}

impl<'de> Deserialize<'de> for NullTerminatedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        Ok(NullTerminatedString::from(s))
    }
}

#[cfg(test)]
mod tests {
    use crate::const_null_terminated_str;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_de_ser_null_terminated_str() {
        macro_rules! def_str_test {
            ($s:expr) => {
                assert_tokens(
                    &const_null_terminated_str!($s),
                    &[Token::BorrowedBytes(concat!($s, "\0").as_bytes())],
                );

                assert_tokens(
                    &(const_null_terminated_str!($s)).to_owned(),
                    &[Token::BorrowedBytes(concat!($s, "\0").as_bytes())],
                );
            };
        }

        def_str_test!("");
        def_str_test!("abcde");
    }
}
