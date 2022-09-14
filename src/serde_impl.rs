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
        self.as_c_str().serialize(serializer)
    }
}

impl Serialize for NullTerminatedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_c_str().serialize(serializer)
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
