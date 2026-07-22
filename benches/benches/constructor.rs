use g_err::{Config, GErrBox, GErrDefault, gerr};
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
pub fn default_box() -> GErrBox {
    GErrDefault::new("default error message").boxed()
}

#[inline(always)]
pub fn default_box_macro() -> GErrBox {
    gerr!("default error message").boxed()
}

#[inline(always)]
pub fn default_fmt() -> GErrDefault {
    GErrDefault::new(format!("default error message: {}, {}", "asd", 123))
}

#[inline(always)]
pub fn default_fmt_macro() -> GErrDefault {
    gerr!("default error message: {}, {}", "asd", 123)
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

#[inline(always)]
pub fn default_with_metadata_source() -> GErrDefault {
    let err = "qwe".parse::<i32>().unwrap_err();
    GErrDefault::new("error with some metadata")
        .set_code("CODE")
        .add_tag("tag1")
        .set_help("help message here")
        .add_source(err)
        .add_source_gerr(GErrDefault::new("gerr source"))
}

#[inline(always)]
pub fn default_with_metadata_source_macro() -> GErrDefault {
    let err = "qwe".parse::<i32>().unwrap_err();
    gerr!("error with some metadata"; code="CODE", tag="tag1", help="help message here", source=err, gerr=gerr!("gerr source"))
}

// ########### anyhow ###########
#[inline(always)]
pub fn anyhow_builder() -> anyhow::Error {
    anyhow::Error::msg("anyhow error message")
}

#[inline(always)]
pub fn anyhow_macro() -> anyhow::Error {
    anyhow::anyhow!("anyhow error message")
}

#[inline(always)]
pub fn anyhow_fmt_builder() -> anyhow::Error {
    anyhow::Error::msg(format!("anyhow error message: {}, {}", "asd", 123))
}

#[inline(always)]
pub fn anyhow_fmt_macro() -> anyhow::Error {
    anyhow::anyhow!("anyhow error message: {}, {}", "asd", 123)
}
