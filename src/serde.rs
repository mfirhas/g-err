use crate::GErr;
use crate::Prefix;
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

/// Serialize into json.
#[allow(unused)]
pub fn json<S, ID, P, D>(err: &GErr<ID, P, D>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    ID: core::fmt::Display + serde::Serialize,
    P: Prefix,
    D: core::fmt::Debug + serde::Serialize,
    GErr<ID, P, D>: Error + 'static,
{
    err.report_as::<crate::DisplayJsonReport>()
        .serialize(serializer)
}
