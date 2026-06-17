use alloc::borrow::Cow;
use core::panic::Location;

use crate::{
    GErr, GErrSource, NoData, NoID, NoPrefix, Prefix, ResultExt, Source, report::GErrView,
};

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct DisplayJsonData {
    pub id: serde_json::Value,
    pub prefix: Option<String>,
    pub message: String,
    pub tags: Option<Vec<String>>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct JsonData {
    pub id: serde_json::Value,
    pub prefix: Option<String>,
    pub message: String,
    pub tags: Option<Vec<String>>,
    pub data: Option<serde_json::Value>,
    pub location: LocationJsonData,
    pub sources: Option<Vec<SourceJsonData>>,

    pub backtrace: Option<String>,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct LocationJsonData {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, Default)]
pub struct SourceJsonData {
    pub id: serde_json::Value,
    pub prefix: Option<String>,
    pub message: String,
    pub tags: Option<Vec<String>>,
    pub data: Option<serde_json::Value>,
    pub location: Option<LocationJsonData>,
    pub sources: Option<Vec<Box<SourceJsonData>>>,
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
            location: LocationJsonData {
                file: err.location.file().into(),
                line: err.location.line(),
                column: err.location.column(),
            },
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
        }
    }
}

impl<ID, P, D> TryFrom<JsonData> for crate::GErr<ID, P, D>
where
    ID: for<'a> ::serde::Deserialize<'a>,
    P: Prefix,
    D: for<'a> ::serde::Deserialize<'a>,
{
    type Error = GErr<NoID, NoPrefix, NoData>;

    fn try_from(value: JsonData) -> Result<Self, Self::Error> {
        let JsonData {
            id,
            prefix,
            message,
            tags,
            data,
            location: _,
            sources,
            backtrace: _,
        } = value;

        let de_id: ID = serde_json::from_value(id).context("converting id")?;

        let mut err = GErr::<ID, P, D>::new_with_id(de_id, message);

        if let Some(data) = data {
            err = err.with_data(serde_json::from_value(data).context("converting data")?);
        }

        if let Some(prefix) = prefix {
            err = err.set_prefix(prefix);
        }

        if let Some(tags) = tags {
            err = err.set_tags(tags);
        }

        if let Some(sources) = sources {
            let gerr_sources: Vec<Source> = sources.into_iter().map(Into::into).collect();
            err.sources = Some(gerr_sources);
        }

        Ok(err)
    }
}

impl<ID, P, D> TryFrom<DisplayJsonData> for crate::GErr<ID, P, D>
where
    ID: for<'a> ::serde::Deserialize<'a>,
    P: Prefix,
    D: for<'a> ::serde::Deserialize<'a>,
{
    type Error = GErr<NoID, NoPrefix, NoData>;

    fn try_from(value: DisplayJsonData) -> Result<Self, Self::Error> {
        let DisplayJsonData {
            id,
            prefix,
            message,
            tags,
            data,
        } = value;

        let de_id: ID = serde_json::from_value(id).context("converting id")?;

        let mut err = GErr::<ID, P, D>::new_with_id(de_id, message);

        if let Some(data) = data {
            err = err.with_data(serde_json::from_value(data).context("converting data")?);
        }

        if let Some(prefix) = prefix {
            err = err.set_prefix(prefix);
        }

        if let Some(tags) = tags {
            err = err.set_tags(tags);
        }

        Ok(err)
    }
}

impl From<&Source> for SourceJsonData {
    fn from(value: &Source) -> Self {
        match value {
            Source::Err(err) => Self {
                message: err.to_string(),
                ..Default::default()
            },
            Source::GErr(gerr) => Self {
                id: gerr.id_json.clone(),
                prefix: gerr.prefix.as_deref().map(|s| s.into()),
                message: gerr.message.to_string(),
                tags: gerr
                    .tags
                    .as_ref()
                    .map(|t| t.iter().map(|t| t.to_string()).collect()),
                data: gerr.data_json.clone(),
                sources: gerr
                    .sources
                    .as_ref()
                    .map(|s| s.iter().map(|s| Box::new(s.into())).collect()),
                location: Some(LocationJsonData {
                    file: gerr.location.file().into(),
                    line: gerr.location.line(),
                    column: gerr.location.column(),
                }),
            },
        }
    }
}

impl From<SourceJsonData> for GErrSource {
    fn from(value: SourceJsonData) -> Self {
        let SourceJsonData {
            id,
            prefix,
            message,
            tags,
            data,
            location: _,
            sources,
        } = value;

        Self {
            id: Box::new(id.to_string()),

            id_json: id,

            message: message.into(),

            prefix: prefix.map(|s| Cow::Owned(s)),

            sources: sources.map(|sources| {
                sources
                    .into_iter()
                    .map(|s| Source::GErr(Box::new((*s).into())))
                    .collect()
            }),

            tags: tags.map(|tags| tags.into_iter().map(Cow::Owned).collect()),

            data: data
                .as_ref()
                .map(|v| Box::new(v.to_string()) as Box<dyn core::fmt::Debug + Send + Sync>),

            data_json: data,

            location: Location::caller(),
        }
    }
}

impl From<SourceJsonData> for Source {
    fn from(value: SourceJsonData) -> Self {
        Source::GErr(Box::new(value.into()))
    }
}
