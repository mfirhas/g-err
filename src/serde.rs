//! Contains serde implementations for [`GErr`].
//!
//! For default serialization, goes into string.
//! No default deserialization.
//!
//! For JSON serde use `g_err::serde::json` for internal,
//! and `g_err::serde::display_json` for public.
//!
use crate::gerr::{GErr, Prefix};
use core::error::Error;
use serde::Serialize;

/// Serialize into string.
impl<ID, P: Prefix, D> serde::Serialize for GErr<ID, P, D>
where
    ID: serde::Serialize,
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
    pub fn serialize<S, ID, P, D>(err: &GErr<ID, P, D>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        ID: core::fmt::Display + serde::Serialize,
        P: Prefix,
        D: core::fmt::Debug + serde::Serialize,
        GErr<ID, P, D>: Error + 'static,
    {
        err.json_data().serialize(serializer)
    }

    /// deserialize GErr from JSON through [`JsonData`]
    pub fn deserialize<'de, De, ID, P, D>(deserializer: De) -> Result<GErr<ID, P, D>, De::Error>
    where
        De: serde::Deserializer<'de>,
        ID: for<'a> serde::Deserialize<'a>,
        P: Prefix,
        D: for<'a> serde::Deserialize<'a>,
    {
        <JsonData as serde::Deserialize>::deserialize(deserializer)?
            .try_into()
            .map_err(::serde::de::Error::custom)
    }
}

/// Serialize and deserialize into/from json for public.
///
/// Attribute:
///
/// `#[serde(with = "g_err::serde::display_json")]`
pub mod display_json {
    use crate::json::DisplayJsonData;

    use super::*;

    /// Serialize GErr into JSON through [`DisplayJsonData`]
    pub fn serialize<S, ID, P, D>(err: &GErr<ID, P, D>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        ID: core::fmt::Display + serde::Serialize,
        P: Prefix,
        D: core::fmt::Debug + serde::Serialize,
        GErr<ID, P, D>: Error + 'static,
    {
        err.display_json_data().serialize(serializer)
    }

    /// deserialize GErr from JSON through [`DisplayJsonData`]
    pub fn deserialize<'de, De, ID, P, D>(deserializer: De) -> Result<GErr<ID, P, D>, De::Error>
    where
        De: serde::Deserializer<'de>,
        ID: for<'a> serde::Deserialize<'a>,
        P: Prefix,
        D: for<'a> serde::Deserialize<'a>,
    {
        <DisplayJsonData as serde::Deserialize>::deserialize(deserializer)?
            .try_into()
            .map_err(::serde::de::Error::custom)
    }
}
