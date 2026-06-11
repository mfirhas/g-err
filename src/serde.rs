use crate::Err;
use crate::Prefix;
use core::error::Error;
use serde::Serialize;

/// Serialize into string.
impl<ID, P: Prefix, D> serde::Serialize for Err<ID, P, D>
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

#[allow(unused)]
/// Serialize into json.
pub fn json<S, ID, P, D>(err: &Err<ID, P, D>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    ID: serde::Serialize,
    D: serde::Serialize,
    P: Prefix,
    Err<ID, P, D>: Error + 'static,
{
    err.display_json_data().serialize(serializer)
}
