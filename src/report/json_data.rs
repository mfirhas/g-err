use alloc::borrow::Cow;
use core::panic::Location;

use crate::{
    Config, GErrDefault, IdSource, gerr,
    gerr::{ErrorLocation, GErr, Source},
    gerr_source::{DataSource, GErrSource},
    gerr_view::GErrView,
};

/// JSON data for public display.
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct DisplayJsonData {
    /// Error ID: can be in form of Number or String.
    pub id: Option<serde_json::Value>,
    /// Error code.
    pub code: Option<String>,
    /// Error message.
    pub message: String,
    /// Error tags.
    pub tags: Option<Vec<String>>,
    /// Error data: can be in any JSON values.
    pub data: Option<serde_json::Value>,
    /// Error help hint.
    pub help: Option<String>,
    /// Error causes
    pub causes: Option<Vec<DisplayCausesJsonData>>,
}

/// Display JSON data causes
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct DisplayCausesJsonData {
    /// Cause message
    pub message: String,
    /// caused by
    pub caused_by: Option<Vec<DisplayCausesJsonData>>,
}

/// JSON data for internal display.
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct JsonData {
    /// Error ID: can be in form of Number or String
    pub id: Option<serde_json::Value>,
    /// Error code
    pub code: Option<String>,
    /// Error message
    pub message: String,
    /// Error tags
    pub tags: Option<Vec<String>>,
    /// Error data: can be in any JSON values.
    pub data: Option<serde_json::Value>,
    /// Error location
    pub location: Option<LocationJsonData>,
    /// Error sources
    pub sources: Option<Vec<SourceJsonData>>,
    /// Error help hint
    pub help: Option<String>,

    /// Error stack trace
    pub backtrace: Option<String>,
}

/// JSON data for error location.
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct LocationJsonData {
    /// File where error happen
    pub file: String,
    /// Line where error happen
    pub line: u32,
    /// Column where error happen
    pub column: u32,
}

/// JSON data for error sources
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, Default)]
pub struct SourceJsonData {
    /// Error ID: can be in form of Number or String
    pub id: Option<serde_json::Value>,
    /// Error code
    pub code: Option<String>,
    /// Error message
    pub message: String,
    /// Error tags
    pub tags: Option<Vec<String>>,
    /// Error data: can be in any JSON values.
    pub data: Option<serde_json::Value>,
    /// Error location
    pub location: Option<LocationJsonData>,
    /// Error sources
    pub sources: Option<Vec<SourceJsonData>>,
    /// Error help hint
    pub help: Option<String>,
}

impl<'a, C: Config, D> From<&GErrView<'a, C, D>> for JsonData
where
    C::Id: ::serde::Serialize,
    D: ::serde::Serialize,
{
    fn from(err: &GErrView<'a, C, D>) -> Self {
        JsonData {
            id: err
                .id
                .map(|id| ::serde_json::to_value(id).unwrap_or_default()),
            code: err.code.map(|s| s.into()),
            message: err.message.into(),
            tags: err.tags.map(|t| t.iter().map(|s| s.to_string()).collect()),
            data: err
                .data
                .map(|d| serde_json::to_value(d).unwrap_or_default()),
            help: err.help.map(Into::into),
            location: Some(LocationJsonData {
                file: err.location.file.to_string(),
                line: err.location.line,
                column: err.location.column,
            }),
            sources: err.sources.map(|s| s.iter().map(|i| i.into()).collect()),
            #[cfg(not(feature = "backtrace"))]
            backtrace: None,
            #[cfg(feature = "backtrace")]
            backtrace: match err.backtrace.status() {
                std::backtrace::BacktraceStatus::Disabled => Some("<disabled>".into()),
                std::backtrace::BacktraceStatus::Captured => Some(err.backtrace.to_string()),
                _ => Some("<unsupported>".into()),
            },
        }
    }
}

impl<'a, C: Config, D> From<&GErrView<'a, C, D>> for DisplayJsonData
where
    C::Id: ::serde::Serialize,
    D: ::serde::Serialize,
{
    fn from(err: &GErrView<'a, C, D>) -> Self {
        DisplayJsonData {
            id: err
                .id
                .map(|id| ::serde_json::to_value(id).unwrap_or_default()),
            code: err.code.map(|s| s.into()),
            message: err.message.into(),
            tags: err.tags.map(|t| t.iter().map(|s| s.to_string()).collect()),
            data: err
                .data
                .map(|d| serde_json::to_value(d).unwrap_or_default()),
            help: err.help.map(Into::into),
            causes: err
                .sources
                .map(|sources| sources.iter().map(|src| src.into()).collect::<Vec<_>>()),
        }
    }
}

/// Convert `JsonData` into `GErr<C, D>`.
///
/// If id and code are empty from JsonData,
/// C: Config will auto-generate them.
impl<C: Config, D> TryFrom<JsonData> for GErr<C, D>
where
    C::Id: for<'a> ::serde::Deserialize<'a>,
    D: for<'a> ::serde::Deserialize<'a>,
{
    type Error = GErrDefault;

    fn try_from(value: JsonData) -> Result<Self, Self::Error> {
        let JsonData {
            id,
            code,
            message,
            tags,
            data,
            help,
            location,
            sources,
            backtrace: _,
        } = value;

        let de_id: Option<C::Id> = if let Some(the_id) = id {
            serde_json::from_value(the_id).map_err(|err| {
                gerr!(
                    "failed converting id to ID = {}",
                    core::any::type_name::<C::Id>()
                )
                .add_source(err)
            })?
        } else {
            C::id()
        };

        let mut err = GErr::<C, D>::new_with_id_untracked(de_id, message, Location::caller());

        if let Some(data) = data {
            err = err.set_data(serde_json::from_value(data).map_err(|err| {
                gerr!(
                    "failed converting data into D = {}",
                    core::any::type_name::<D>()
                )
                .add_source(err)
            })?);
        }

        if let Some(code) = code {
            err = err.set_code(code);
        } else if let Some(const_code) = C::CODE {
            err = err.set_code(const_code);
        }

        if let Some(tags) = tags {
            err = err.add_tags(tags);
        }

        if let Some(sources) = sources {
            let gerr_sources: Vec<Source> = sources.into_iter().map(|s| s.into_source()).collect();
            err = err.set_sources(gerr_sources);
        }

        if let Some(help) = help {
            err = err.set_help(help);
        }

        if let Some(loc) = location {
            err = err.set_location(ErrorLocation {
                file: Cow::Owned(loc.file),
                line: loc.line,
                column: loc.column,
            });
        }

        Ok(err)
    }
}

impl From<&Source> for SourceJsonData {
    fn from(gerr: &Source) -> Self {
        match gerr {
            Source::Err(err) => Self {
                message: err.to_string(),
                ..Default::default()
            },
            Source::GErr(gerr) => Self {
                id: serde_json::from_value({
                    match &gerr.id_json {
                        Some(::serde_json::Value::Number(num)) => {
                            ::serde_json::Value::from(num.as_i64().unwrap_or_default())
                        }
                        Some(::serde_json::Value::String(s)) => {
                            ::serde_json::Value::from(s.as_str())
                        }
                        Some(::serde_json::Value::Bool(b)) => ::serde_json::Value::from(*b),
                        Some(::serde_json::Value::Array(arr)) => {
                            ::serde_json::Value::from(arr.as_slice())
                        }
                        Some(::serde_json::Value::Object(obj)) => {
                            ::serde_json::Value::from(obj.clone())
                        }
                        _ => ::serde_json::Value::Null,
                    }
                })
                .unwrap_or_default(),
                code: gerr.code.as_deref().map(|s| s.into()),
                message: gerr.message.to_string(),
                tags: gerr
                    .tags
                    .as_ref()
                    .map(|t| t.iter().map(|t| t.to_string()).collect()),
                data: serde_json::from_value({
                    if let Some(json) = &gerr.data_json {
                        match json {
                            ::serde_json::Value::Bool(b) => ::serde_json::Value::from(*b),
                            ::serde_json::Value::Number(num) => {
                                ::serde_json::Value::from(num.as_i64().unwrap_or_default())
                            }
                            ::serde_json::Value::String(s) => ::serde_json::Value::from(s.as_str()),
                            ::serde_json::Value::Array(arr) => {
                                let slice: &[::serde_json::Value] = arr.as_ref();
                                ::serde_json::Value::from(slice)
                            }
                            serde_json::Value::Object(obj) => {
                                ::serde_json::Value::Object(obj.clone())
                            }
                            _ => ::serde_json::Value::Null,
                        }
                    } else {
                        ::serde_json::Value::Null
                    }
                })
                .unwrap_or_default(),
                help: gerr.help.as_deref().map(|s| s.into()),
                sources: gerr
                    .sources
                    .as_ref()
                    .map(|s| s.iter().map(|s| s.into()).collect()),
                location: gerr.location.as_ref().map(|loc| LocationJsonData {
                    file: loc.file.to_string(),
                    line: loc.line,
                    column: loc.column,
                }),
            },
        }
    }
}

impl From<&Source> for DisplayCausesJsonData {
    fn from(value: &Source) -> Self {
        match value {
            Source::Err(err) => Self {
                message: err.to_string(),
                caused_by: None,
            },
            Source::GErr(gerr) => Self {
                message: gerr.to_string(),
                caused_by: gerr
                    .sources
                    .as_deref()
                    .map(|s| s.iter().map(|i| i.into()).collect::<Vec<_>>()),
            },
        }
    }
}

impl SourceJsonData {
    fn into_source(self) -> Source {
        let SourceJsonData {
            id,
            code,
            message,
            tags,
            data,
            help,
            location,
            sources,
        } = self;

        let gerr_source = GErrSource {
            id: match id {
                Some(serde_json::Value::Bool(b)) => Some(Box::new(b)),
                Some(serde_json::Value::Number(ref num)) => {
                    Some(Box::new(num.as_i64().unwrap_or_default()))
                }
                _ => id
                    .as_ref()
                    .map(|id| Box::new(id.to_string()) as Box<dyn IdSource>),
            },

            id_json: id,

            message: message.into(),

            code: code.map(Cow::Owned),

            sources: sources.map(|s| s.into_iter().map(|sj| sj.into_source()).collect()),

            tags: tags.map(|tags| tags.into_iter().map(Cow::Owned).collect()),

            data: data
                .as_ref()
                .map(|v| Box::new(v.to_string()) as Box<dyn DataSource>),

            data_json: data,

            help: help.map(Cow::Owned),

            location: location.map(|loc| ErrorLocation {
                file: Cow::Owned(loc.file),
                line: loc.line,
                column: loc.column,
            }),
        };

        Source::GErr(Box::new(gerr_source))
    }
}
