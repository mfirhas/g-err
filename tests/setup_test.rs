// import:
// #[path = "setup_test.rs"]
// mod setup_test;
// use setup_test::*;

use g_err::{Id, Prefix, SetField};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct AutoID;

impl Id for AutoID {
    #[inline]
    fn id() -> Self {
        Self
    }
}

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

pub struct AutoPrefix;

impl Prefix for AutoPrefix {
    const PREFIX: Option<&'static str> = Some("AutoPrefix");
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Data {
    pub user_id: u64,
    pub user_name: String,
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
