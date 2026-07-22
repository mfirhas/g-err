use g_err::{Config, GErrDefault, gerr};
use uuid::Uuid;

pub const MSG: &str = "database connection failed";

pub struct ErrI32Code;

impl Config for ErrI32Code {
    type Id = i32;
    const CODE: Option<&'static str> = Some("HTTP");
    const TAGS: Option<&'static [&'static str]> = Some(&["tag1", "tag2"]);
}

pub struct ErrAutoUUIDCode;

impl Config for ErrAutoUUIDCode {
    type Id = uuid::Uuid;

    #[inline(always)]
    fn id() -> Option<Self::Id> {
        Some(Uuid::new_v4())
    }

    const CODE: Option<&'static str> = Some("HTTP");
    const TAGS: Option<&'static [&'static str]> = Some(&["tag1", "tag2"]);
}

#[derive(Debug)]
pub struct Data {
    user_id: i32,
    user_name: String,
}

#[inline(always)]
pub fn default() -> GErrDefault {
    GErrDefault::new("default error message")
}

#[inline(always)]
pub fn default_macro() -> GErrDefault {
    gerr!("default error message")
}

#[inline(always)]
pub fn default_with_metadata() -> GErrDefault {
    GErrDefault::new("error with some metadata")
        .set_code("CODE")
        .add_tag("tag1")
        .set_help("help message here")
}

#[inline(always)]
pub fn default_with_metadata_macro() -> GErrDefault {
    gerr!("error with some metadata"; code="CODE", tag="tag1", help="help message here")
}
