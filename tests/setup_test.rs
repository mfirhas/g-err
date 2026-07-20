// import:
// #[path = "setup_test.rs"]
// mod setup_test;
// use setup_test::*;
use g_err::{Config, NoID, SetField};

#[allow(dead_code)]
pub struct ErrAutoID;

impl Config for ErrAutoID {
    type Id = AutoID;

    #[inline]
    fn id() -> Option<Self::Id> {
        Some(AutoID)
    }
}

#[allow(dead_code)]
pub struct ErrAutoCode;

impl Config for ErrAutoCode {
    const CODE: Option<&'static str> = Some("AutoCode");
    type Id = NoID;
}

#[allow(dead_code)]
pub struct ErrAutoIDCode;

impl Config for ErrAutoIDCode {
    const CODE: Option<&'static str> = Some("AutoCode");

    type Id = AutoID;

    #[inline]
    fn id() -> Option<Self::Id> {
        Some(AutoID)
    }
}

#[allow(dead_code)]
pub struct ErrIDi32;

impl Config for ErrIDi32 {
    type Id = i32;
}

#[allow(dead_code)]
pub struct ErrIDi32AutoCode;

impl Config for ErrIDi32AutoCode {
    type Id = i32;
    const CODE: Option<&'static str> = Some("AutoCode");
}

#[allow(dead_code)]
pub struct ErrIDStr;

impl Config for ErrIDStr {
    type Id = &'static str;
}

#[allow(dead_code)]
pub struct ErrIDStrAutoCode;

impl Config for ErrIDStrAutoCode {
    const CODE: Option<&'static str> = Some("AutoCode");
    type Id = &'static str;
}

#[allow(dead_code)]
pub struct ErrIDBool;

impl Config for ErrIDBool {
    type Id = bool;
}

#[allow(dead_code)]
pub struct ErrIDBoolCode;

impl Config for ErrIDBoolCode {
    const CODE: Option<&'static str> = Some("AutoCode");
    type Id = bool;
}

#[allow(dead_code)]
pub struct ErrIDArrCode;

impl Config for ErrIDArrCode {
    const CODE: Option<&'static str> = Some("AutoCode");
    type Id = [i32; 2];
}

#[allow(dead_code)]
pub struct ErrIDDataCode;

impl Config for ErrIDDataCode {
    const CODE: Option<&'static str> = Some("AutoCode");
    type Id = Data;
}

#[allow(dead_code)]
pub struct ErrIDCodeTags;

impl Config for ErrIDCodeTags {
    type Id = AutoID;
    fn id() -> Option<Self::Id> {
        Some(AutoID)
    }

    const CODE: Option<&'static str> = Some("AutoCode");
    const TAGS: Option<&'static [&'static str]> = Some(&["tag1", "tag2", "tag3"]);
}

#[allow(dead_code)]
pub struct ErrTags;

impl Config for ErrTags {
    type Id = NoID;
    const TAGS: Option<&'static [&'static str]> = Some(&["http"]);
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct AutoID;

impl core::fmt::Display for AutoID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AutoID")
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for AutoID {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("AutoID")
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for AutoID {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AutoIDVisitor;

        impl<'de> serde::de::Visitor<'de> for AutoIDVisitor {
            type Value = AutoID;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str("the string \"AutoID\"")
            }

            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v == "AutoID" {
                    Ok(AutoID)
                } else {
                    Err(E::invalid_value(serde::de::Unexpected::Str(v), &self))
                }
            }
        }

        deserializer.deserialize_str(AutoIDVisitor)
    }
}

#[allow(dead_code)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Data {
    pub user_id: u64,
    pub user_name: String,
}

impl core::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}; user_name: {}", self.user_id, self.user_name)
    }
}

impl SetField<&'static str, u64> for Data {
    fn set_field(&mut self, key: &'static str, value: u64) {
        if key == "user_id" {
            self.user_id = value;
        }
    }
}

impl SetField<&'static str, String> for Data {
    fn set_field(&mut self, key: &'static str, value: String) {
        if key == "user_name" {
            self.user_name = value;
        }
    }
}
