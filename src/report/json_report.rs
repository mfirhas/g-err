use alloc::borrow::Cow;

use crate::{
    Source,
    report::{GErrView, Report},
};

pub struct JsonReport;

impl Report for JsonReport {
    fn report<E, ID, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: std::fmt::Display + serde::Serialize,
        D: std::fmt::Debug + serde::Serialize,
    {
        let err = &err.into();
        let resp: JsonReportData = err.into();
        serde_json::to_string_pretty(&resp).unwrap_or("<invalid json format>".into())
    }
}

pub struct DisplayJsonReport;

impl Report for DisplayJsonReport {
    fn report<E, ID, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: std::fmt::Display + serde::Serialize,
        D: std::fmt::Debug + serde::Serialize,
    {
        let err = &err.into();
        let resp: DisplayJsonReportData = err.into();
        serde_json::to_string_pretty(&resp).unwrap_or("<invalid json format>".into())
    }
}

#[derive(serde::Serialize)]
pub struct DisplayJsonReportData<'a> {
    pub id: Option<serde_json::Value>,
    pub prefix: Option<&'static str>,
    pub message: &'a str,
    pub tags: Option<&'a [Cow<'static, str>]>,
    pub data: Option<serde_json::Value>,
}

#[derive(serde::Serialize)]
pub struct JsonReportData<'a> {
    pub id: Option<serde_json::Value>,
    pub prefix: Option<&'static str>,
    pub message: &'a str,
    pub tags: Option<&'a [Cow<'static, str>]>,
    pub data: Option<serde_json::Value>,
    pub location: LocationJson<'a>,
    pub sources: Option<Vec<SourceJson<'a>>>,

    #[cfg(feature = "backtrace")]
    pub backtrace: String,
}

#[derive(serde::Serialize)]
pub struct LocationJson<'a> {
    pub file: &'a str,
    pub line: u32,
    pub column: u32,
}

#[derive(serde::Serialize)]
pub struct SourceJson<'a> {
    pub id: Option<&'a serde_json::Value>,
    pub prefix: Option<&'static str>,
    pub message: String,
    pub tags: Option<&'a [Cow<'static, str>]>,
    pub data: Option<&'a serde_json::Value>,
    pub location: Option<LocationJson<'a>>,
    pub sources: Option<Vec<Box<SourceJson<'a>>>>,
}

impl<'a, ID, D> From<&'a GErrView<'a, ID, D>> for DisplayJsonReportData<'a>
where
    ID: serde::Serialize,
    D: serde::Serialize,
{
    fn from(value: &'a GErrView<ID, D>) -> Self {
        Self {
            id: serde_json::to_value(value.id).ok(),
            prefix: value.prefix,
            message: value.message,
            tags: value.tags,
            data: serde_json::to_value(value.data).ok(),
        }
    }
}

impl<'a, ID, D> From<&'a GErrView<'a, ID, D>> for JsonReportData<'a>
where
    ID: serde::Serialize,
    D: serde::Serialize,
{
    fn from(value: &'a GErrView<ID, D>) -> Self {
        Self {
            id: serde_json::to_value(value.id).ok(),
            prefix: value.prefix,
            message: value.message,
            tags: value.tags,
            data: serde_json::to_value(value.data).ok(),
            location: LocationJson {
                file: value.location.file(),
                line: value.location.line(),
                column: value.location.column(),
            },
            sources: value.sources.map(|s| s.iter().map(|v| v.into()).collect()),

            #[cfg(feature = "backtrace")]
            backtrace: format!("{:#?}", value.backtrace),
        }
    }
}

impl<'a> From<&'a Source> for SourceJson<'a> {
    fn from(value: &'a Source) -> Self {
        match value {
            Source::Err(err) => Self {
                id: None,
                prefix: None,
                message: err.to_string(),
                tags: None,
                data: None,
                location: None,
                sources: None,
            },
            Source::GErr(gerr) => Self {
                id: Some(&gerr.id_json),
                prefix: gerr.prefix,
                message: gerr.message.to_string(),
                tags: gerr.tags.as_deref(),
                data: gerr.data_json.as_ref(),
                location: Some(LocationJson {
                    file: gerr.location.file(),
                    line: gerr.location.line(),
                    column: gerr.location.column(),
                }),
                sources: gerr
                    .source
                    .as_deref()
                    .map(|ref s| s.iter().map(|src| Box::new(src.into())).collect()),
            },
        }
    }
}
