use crate::{
    Source,
    report::{GErrView, Report},
};
use alloc::borrow::Cow;

pub struct JsonReport;

impl JsonReport {
    pub fn data<E, ID, D>(err: &E) -> pub_json_data::JsonData
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: std::fmt::Display + serde::Serialize,
        D: std::fmt::Debug + serde::Serialize,
    {
        let err = &err.into();
        err.into()
    }
}

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

impl DisplayJsonReport {
    pub fn data<E, ID, D>(err: &E) -> pub_json_data::DisplayJsonData
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: std::fmt::Display + serde::Serialize,
        D: std::fmt::Debug + serde::Serialize,
    {
        let err = &err.into();
        err.into()
    }
}

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

pub mod pub_json_data {
    use alloc::borrow::Cow;
    use core::panic::Location;

    use crate::{
        GErr, GErrSource, IdSource, NoData, NoID, NoPrefix, Prefix, ResultExt, Source,
        report::GErrView,
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
                err = err.set_prefix("TODO");
            }

            if let Some(tags) = tags {
                err = err.set_tags(tags);
            }

            if let Some(sources) = sources {
                let gerr_sources: Vec<Source> = sources.into_iter().map(Into::into).collect();
                err.source = Some(gerr_sources);
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
                err = err.set_prefix("TODO");
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
                    prefix: gerr.prefix.map(|s| s.into()),
                    message: gerr.message.to_string(),
                    tags: gerr
                        .tags
                        .as_ref()
                        .map(|t| t.iter().map(|t| t.to_string()).collect()),
                    data: gerr.data_json.clone(),
                    sources: gerr
                        .source
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

            let de_id = {
                let ret = match id.clone() {
                    ::serde_json::Value::Null => Box::new(NoID) as Box<dyn IdSource>,
                    ::serde_json::Value::Bool(_) => Box::new(NoID) as Box<dyn IdSource>,
                    ::serde_json::Value::Number(n) => {
                        Box::new(n.as_i64().unwrap_or_default()) as Box<dyn IdSource>
                    }
                    ::serde_json::Value::String(s) => Box::new(s) as Box<dyn IdSource>,
                    ::serde_json::Value::Array(_) => Box::new(NoID) as Box<dyn IdSource>,
                    ::serde_json::Value::Object(_) => Box::new(NoID) as Box<dyn IdSource>,
                };
                ret
            };
            let de_data = {
                if let Some(d) = data.clone() {
                    let ret = match d {
                        ::serde_json::Value::Null => {
                            Some(Box::new(NoID) as Box<dyn core::fmt::Debug + Send + Sync>)
                        }
                        ::serde_json::Value::Bool(b) => {
                            Some(Box::new(b) as Box<dyn core::fmt::Debug + Send + Sync>)
                        }
                        ::serde_json::Value::Number(n) => {
                            Some(Box::new(n.as_i64().unwrap_or_default())
                                as Box<dyn core::fmt::Debug + Send + Sync>)
                        }
                        ::serde_json::Value::String(s) => {
                            Some(Box::new(s) as Box<dyn core::fmt::Debug + Send + Sync>)
                        }
                        ::serde_json::Value::Array(a) => {
                            Some(Box::new(a) as Box<dyn core::fmt::Debug + Send + Sync>)
                        }
                        ::serde_json::Value::Object(obj) => {
                            Some(Box::new(obj) as Box<dyn core::fmt::Debug + Send + Sync>)
                        }
                    };
                    ret
                } else {
                    None
                }
            };

            Self {
                id: de_id,

                id_json: id,

                message: message.into(),

                prefix: Some("TODO"),

                source: sources.map(|sources| {
                    sources
                        .into_iter()
                        .map(|s| Source::GErr(Box::new((*s).into())))
                        .collect()
                }),

                tags: tags.map(|tags| tags.into_iter().map(Cow::Owned).collect()),

                data: de_data,

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
}

#[derive(serde::Serialize)]
struct DisplayJsonReportData<'a> {
    pub id: Option<serde_json::Value>,
    pub prefix: Option<&'static str>,
    pub message: &'a str,
    pub tags: Option<&'a [Cow<'static, str>]>,
    pub data: Option<serde_json::Value>,
}

#[derive(serde::Serialize)]
struct JsonReportData<'a> {
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
struct LocationJson<'a> {
    pub file: &'a str,
    pub line: u32,
    pub column: u32,
}

#[derive(serde::Serialize)]
struct SourceJson<'a> {
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
