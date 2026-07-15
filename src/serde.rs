//! Contains serde implementations for [`GErr`].
//!
//! For default serialization, goes into string.
//! No default deserialization.
//!
//! For JSON serde use `g_err::serde::json` for internal,
//! and `g_err::serde::display_json` for public.
//!
use crate::gerr::{Config, GErr};
use core::error::Error;
use serde::Serialize;

/// Serialize into string.
impl<C: Config, D> serde::Serialize for GErr<C, D>
where
    C::Id: serde::Serialize,
    D: serde::Serialize,
    Self: Error + 'static,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

/// Serialize and deserialize into/from json for internal.
///
/// Attribute:
///
/// `#[serde(with = "g_err::serde::json")]`
pub mod json {
    use crate::json::JsonData;

    use super::*;

    /// Serialize GErr into JSON through [`JsonData`]
    pub fn serialize<S, C: Config, D>(err: &GErr<C, D>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        C::Id: core::fmt::Display + serde::Serialize,
        D: core::fmt::Debug + serde::Serialize,
        GErr<C, D>: Error + 'static,
    {
        err.json_data().serialize(serializer)
    }

    /// deserialize GErr from JSON through [`JsonData`]
    pub fn deserialize<'de, De, C: Config, D>(deserializer: De) -> Result<GErr<C, D>, De::Error>
    where
        De: serde::Deserializer<'de>,
        C::Id: for<'a> serde::Deserialize<'a>,
        D: for<'a> serde::Deserialize<'a>,
    {
        <JsonData as serde::Deserialize>::deserialize(deserializer)?
            .try_into()
            .map_err(::serde::de::Error::custom)
    }
}

/// Serialize into json for public.
///
/// Attribute:
///
/// `#[serde(serialize_with = "g_err::serde::display_json::serialize")]`
pub mod display_json {
    use super::*;

    /// Serialize GErr into JSON through [`crate::json::DisplayJsonData`]
    pub fn serialize<S, C: Config, D>(err: &GErr<C, D>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        C::Id: core::fmt::Display + serde::Serialize,
        D: core::fmt::Debug + serde::Serialize,
        GErr<C, D>: Error + 'static,
    {
        err.display_json_data().serialize(serializer)
    }
}
