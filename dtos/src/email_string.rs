//! variant of <https://stackoverflow.com/questions/74482350/adding-length-limit-when-deserializing-a-string-a-vec-or-an-array>

use regex::Regex;
use serde::de;
use std::{ops::Deref, sync::LazyLock};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct EmailString(String);

impl Deref for EmailString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<EmailString> for String {
    fn from(v: EmailString) -> Self {
        v.0
    }
}

pub static EMAIL_RX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").expect("Regex should compile")
});

impl<'de> de::Deserialize<'de> for EmailString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        <String as de::Deserialize>::deserialize(deserializer).and_then(|inner| {
            if EMAIL_RX.is_match(&inner) {
                Ok(EmailString(inner))
            } else {
                Err(de::Error::custom("The string is not an email string"))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_email<S: ToString + ?Sized>(email: &S) -> bool {
        let email = email.to_string();
        EMAIL_RX.is_match(&email)
    }

    #[test]
    fn regex_compiles() {
        let _ = LazyLock::<_>::force(&EMAIL_RX);
    }

    #[test]
    fn test_email() {
        assert!(is_email("kangoedin@gmail.com"));
    }

    #[test]
    fn double_dot() {
        assert!(is_email("abrit@mail.co.uk"));
    }

    #[test]
    fn not_email() {
        assert!(!is_email("not an email"));
    }
}
