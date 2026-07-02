use core::fmt::{Debug, Display};

use crate::gerr::{Id, Prefix};

/// Type for generic parameter `ID` for GErr without id.
#[derive(Debug, PartialEq, Eq)]
pub struct NoID;

impl Display for NoID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NoID")
    }
}

impl Id for NoID {
    #[inline]
    fn id() -> NoID {
        NoID
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for NoID {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit()
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for NoID {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <()>::deserialize(deserializer)?;
        Ok(NoID)
    }
}

/// Type for generic parameter `P` for GErr without prefix.
pub struct NoPrefix;

impl Prefix for NoPrefix {}

/// Type for generic parameter `D` for GErr without data.
#[derive(Debug, PartialEq, Eq)]
pub struct NoData;

#[cfg(feature = "serde")]
impl ::serde::Serialize for NoData {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit()
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for NoData {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <()>::deserialize(deserializer)?;
        Ok(NoData)
    }
}
