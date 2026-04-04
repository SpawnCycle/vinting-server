//! based on <https://stackoverflow.com/questions/74482350/adding-length-limit-when-deserializing-a-string-a-vec-or-an-array>

use serde::{de, ser};
use std::ops::Deref;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct LimitedString<const MAX_LENGTH: usize = 1024, const MIN_LENGTH: usize = 0>(String);

impl<const MAX_LENGTH: usize, const MIN_LENGTH: usize> Deref
    for LimitedString<MAX_LENGTH, MIN_LENGTH>
{
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MAX_LENGTH: usize, const MIN_LENGTH: usize> From<LimitedString<MAX_LENGTH, MIN_LENGTH>>
    for String
{
    fn from(v: LimitedString<MAX_LENGTH, MIN_LENGTH>) -> Self {
        v.0
    }
}

impl<'de, const MAX_LENGTH: usize, const MIN_LENGTH: usize> de::Deserialize<'de>
    for LimitedString<MAX_LENGTH, MIN_LENGTH>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        <String as de::Deserialize>::deserialize(deserializer).and_then(|inner| {
            let char_count = inner.chars().count();
            if char_count < MIN_LENGTH {
                Err(de::Error::invalid_length(
                    inner.len(),
                    &"The given string was shorter than the allowed length",
                ))
            } else if char_count > MAX_LENGTH {
                Err(de::Error::invalid_length(
                    inner.len(),
                    &"The given string was longer than the allowed length",
                ))
            } else {
                Ok(Self(inner))
            }
        })
    }
}

impl<const MAX_LENGTH: usize, const MIN_LENGTH: usize> ser::Serialize
    for LimitedString<MAX_LENGTH, MIN_LENGTH>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        <String as ser::Serialize>::serialize(&self.0, serializer)
    }
}
