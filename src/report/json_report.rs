use crate::gerr::Source;
use crate::report::json_data::{DisplayJsonData, JsonData};
use crate::types::NoID;
use crate::{gerr_view::GErrView, report::Report};
use alloc::borrow::Cow;
use core::fmt::{Debug, Display};

/// GErr's reporting as JSON for internal reports.
pub struct JsonReport;

impl JsonReport {
    /// Internal JSON report data.
    pub fn data<E, ID, D>(err: &E) -> JsonData
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: serde::Serialize,
        D: serde::Serialize,
    {
        let err = &err.into();
        err.into()
    }

    /// Public JSON report data.
    pub fn display_data<E, ID, D>(err: &E) -> DisplayJsonData
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: serde::Serialize,
        D: serde::Serialize,
    {
        let err = &err.into();
        err.into()
    }
}

impl Report for JsonReport {
    fn report<E, ID, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: Display + serde::Serialize,
        D: Debug + serde::Serialize,
    {
        let err = &err.into();
        let resp: JsonReportData = err.into();
        serde_json::to_string_pretty(&resp).unwrap_or("<invalid json format>".into())
    }
}

#[derive(serde::Serialize)]
struct JsonReportData<'a> {
    pub id: serde_json::Value,
    pub prefix: Option<&'a str>,
    pub message: &'a str,
    pub tags: Option<&'a [Cow<'static, str>]>,
    pub data: Option<serde_json::Value>,
    pub location: LocationJson<'a>,
    pub sources: Option<Vec<SourceJson<'a>>>,
    pub help: Option<&'a str>,

    #[cfg(feature = "backtrace")]
    pub backtrace: String,
}

#[derive(serde::Serialize)]
struct LocationJson<'a> {
    pub file: &'a str,
    pub line: u32,
    pub column: u32,
}

#[derive(serde::Serialize)]
struct SourceJson<'a> {
    pub id: &'a serde_json::Value,
    pub prefix: Option<&'a str>,
    pub message: String,
    pub tags: Option<&'a [Cow<'static, str>]>,
    pub data: Option<&'a serde_json::Value>,
    pub location: Option<LocationJson<'a>>,
    pub sources: Option<Vec<SourceJson<'a>>>,
    pub help: Option<&'a str>,
}

impl<'a, ID, D> From<&'a GErrView<'a, ID, D>> for JsonReportData<'a>
where
    ID: serde::Serialize,
    D: serde::Serialize,
{
    fn from(value: &'a GErrView<ID, D>) -> Self {
        Self {
            id: serde_json::to_value(value.id)
                .unwrap_or(serde_json::to_value(NoID).unwrap_or_default()),
            prefix: value.prefix,
            message: value.message,
            tags: value.tags,
            data: serde_json::to_value(value.data).ok(),
            location: LocationJson {
                file: &value.location.file,
                line: value.location.line,
                column: value.location.column,
            },
            sources: value.sources.map(|s| s.iter().map(|v| v.into()).collect()),
            help: value.help,

            #[cfg(feature = "backtrace")]
            backtrace: format!("{:#?}", value.backtrace),
        }
    }
}

pub static NO_ID_JSON: serde_json::Value = serde_json::Value::Null;

impl<'a> From<&'a Source> for SourceJson<'a> {
    fn from(source: &'a Source) -> Self {
        match source {
            Source::Err(err) => Self {
                id: &NO_ID_JSON,
                prefix: None,
                message: err.to_string(),
                tags: None,
                data: None,
                location: None,
                sources: None,
                help: None,
            },
            Source::GErr(gerr) => Self {
                id: &gerr.id_json,
                prefix: gerr.prefix.as_deref(),
                message: gerr.message.to_string(),
                tags: gerr.tags.as_deref(),
                data: gerr.data_json.as_ref(),
                location: gerr.location.as_ref().map(|loc| LocationJson {
                    file: &loc.file,
                    line: loc.line,
                    column: loc.column,
                }),
                sources: gerr
                    .sources
                    .as_deref()
                    .map(|s| s.iter().map(|src| src.into()).collect()),
                help: gerr.help.as_deref(),
            },
        }
    }
}
