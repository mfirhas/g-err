use alloc::borrow::Cow;
use core::panic::Location;
use core::str::FromStr;

use crate::{
    ResultExt,
    gerr::{GErr, Prefix, Source},
    gerr_source::{DataSource, GErrSource},
    gerr_view::GErrView,
    types::{NoData, NoID, NoPrefix},
};

/// JSON data for public display.
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct DisplayJsonData {
    /// Error ID: can be in form of Number or String.
    pub id: serde_json::Value,
    /// Error prefix.
    pub prefix: Option<String>,
    /// Error message.
    pub message: String,
    /// Error tags.
    pub tags: Option<Vec<String>>,
    /// Error data: can be in any JSON values.
    pub data: Option<serde_json::Value>,
    /// Error help hint.
    pub help: Option<String>,
}

/// JSON data for internal display.
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct JsonData {
    /// Error ID: can be in form of Number or String
    pub id: serde_json::Value,
    /// Error prefix
    pub prefix: Option<String>,
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
    pub id: serde_json::Value,
    /// Error prefix
    pub prefix: Option<String>,
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

impl<'a, ID, D> From<&GErrView<'a, ID, D>> for JsonData
where
    ID: ::serde::Serialize,
    D: ::serde::Serialize,
{
    fn from(err: &GErrView<'a, ID, D>) -> Self {
        JsonData {
            id: serde_json::to_value(err.id)
                .unwrap_or(serde_json::to_value(NoID).unwrap_or_default()),
            prefix: err.prefix.map(|s| s.into()),
            message: err.message.into(),
            tags: err.tags.map(|t| t.iter().map(|s| s.to_string()).collect()),
            data: err
                .data
                .map(|d| serde_json::to_value(d).unwrap_or_default()),
            help: err.help.map(Into::into),
            location: Some(LocationJsonData {
                file: err.location.file().into(),
                line: err.location.line(),
                column: err.location.column(),
            }),
            sources: err.sources.map(|s| s.iter().map(|i| i.into()).collect()),
            #[cfg(not(feature = "backtrace"))]
            backtrace: None,
            #[cfg(feature = "backtrace")]
            backtrace: Some(err.backtrace.to_string()),
        }
    }
}

impl<'a, ID, D> From<&GErrView<'a, ID, D>> for DisplayJsonData
where
    ID: ::serde::Serialize,
    D: ::serde::Serialize,
{
    fn from(err: &GErrView<'a, ID, D>) -> Self {
        DisplayJsonData {
            id: serde_json::to_value(err.id)
                .unwrap_or(serde_json::to_value(NoID).unwrap_or_default()),
            prefix: err.prefix.map(|s| s.into()),
            message: err.message.into(),
            tags: err.tags.map(|t| t.iter().map(|s| s.to_string()).collect()),
            data: err
                .data
                .map(|d| serde_json::to_value(d).unwrap_or_default()),
            help: err.help.map(Into::into),
        }
    }
}

impl<ID, P, D> TryFrom<JsonData> for GErr<ID, P, D>
where
    ID: for<'a> ::serde::Deserialize<'a>,
    P: Prefix,
    D: for<'a> ::serde::Deserialize<'a>,
{
    type Error = GErr<NoID, NoPrefix, NoData>;

    #[track_caller]
    fn try_from(value: JsonData) -> Result<Self, Self::Error> {
        let JsonData {
            id,
            prefix,
            message,
            tags,
            data,
            help,
            location: _,
            sources,
            backtrace: _,
        } = value;

        let de_id: ID = serde_json::from_value(id).context_auto("converting id")?;

        let mut err = GErr::<ID, P, D>::with_id_untracked(de_id, message, Location::caller());

        if let Some(data) = data {
            err = err.with_data(serde_json::from_value(data).context_auto("converting data")?);
        }

        if let Some(prefix) = prefix {
            err = err.set_prefix(prefix);
        }

        if let Some(tags) = tags {
            err = err.add_tags(tags);
        }

        if let Some(sources) = sources {
            let gerr_sources: Vec<Source> = sources
                .into_iter()
                .map(|s| s.into_source(Location::caller()))
                .collect();
            err = err.set_sources(gerr_sources);
        }

        if let Some(help) = help {
            err = err.set_help(help);
        }

        Ok(err)
    }
}

impl<ID, P, D> TryFrom<DisplayJsonData> for GErr<ID, P, D>
where
    ID: for<'a> ::serde::Deserialize<'a>,
    P: Prefix,
    D: for<'a> ::serde::Deserialize<'a>,
{
    type Error = GErr<NoID, NoPrefix, NoData>;

    #[track_caller]
    fn try_from(value: DisplayJsonData) -> Result<Self, Self::Error> {
        let DisplayJsonData {
            id,
            prefix,
            message,
            tags,
            data,
            help,
        } = value;

        let de_id: ID = serde_json::from_value(id).context_auto("converting id")?;

        let mut err = GErr::<ID, P, D>::with_id_untracked(de_id, message, Location::caller());

        if let Some(data) = data {
            err = err.with_data(serde_json::from_value(data).context_auto("converting data")?);
        }

        if let Some(prefix) = prefix {
            err = err.set_prefix(prefix);
        }

        if let Some(tags) = tags {
            err = err.add_tags(tags);
        }

        if let Some(help) = help {
            err = err.set_help(help);
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
                        ::serde_json::Value::Number(num) => {
                            serde_json::Value::from(num.as_i64().unwrap_or_default())
                        }
                        ::serde_json::Value::String(s) => {
                            serde_json::Value::from_str(s).unwrap_or_default()
                        }
                        _ => serde_json::Value::Null,
                    }
                })
                .unwrap_or_default(),
                prefix: gerr.prefix.as_deref().map(|s| s.into()),
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
                            ::serde_json::Value::String(s) => {
                                ::serde_json::Value::from_str(s).unwrap_or_default()
                            }
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
                help: gerr.prefix.as_deref().map(|s| s.into()),
                sources: gerr
                    .sources
                    .as_ref()
                    .map(|s| s.iter().map(|s| s.into()).collect()),
                location: gerr.location.map(|loc| LocationJsonData {
                    file: loc.file().into(),
                    line: loc.line(),
                    column: loc.column(),
                }),
            },
        }
    }
}

impl SourceJsonData {
    fn into_source(self, location: &'static Location<'static>) -> Source {
        let SourceJsonData {
            id,
            prefix,
            message,
            tags,
            data,
            help,
            location: _,
            sources,
        } = self;

        let gerr_source = GErrSource {
            id: Box::new(id.to_string()),

            id_json: id,

            message: message.into(),

            prefix: prefix.map(Cow::Owned),

            sources: sources.map(|s| s.into_iter().map(|sj| sj.into_source(location)).collect()),

            tags: tags.map(|tags| tags.into_iter().map(Cow::Owned).collect()),

            data: data
                .as_ref()
                .map(|v| Box::new(v.to_string()) as Box<dyn DataSource>),

            data_json: data,

            help: help.map(Cow::Owned),

            location: Some(location),
        };

        Source::GErr(Box::new(gerr_source))
    }
}
