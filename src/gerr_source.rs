extern crate alloc;

use alloc::borrow::Cow;

use core::{
    any::Any,
    error::Error,
    fmt::{Debug, Display},
    panic::Location,
};

use crate::NoID;

/// Dyn-compatible trait for error id.
pub trait IdSource: Any + Debug + Display + Send + Sync {}

impl<T> IdSource for T where T: Any + Debug + Display + Send + Sync {}

/// Dyn-compatible trait for error data.
pub trait DataSource: Any + Debug + Send + Sync {}

impl<T> DataSource for T where T: Any + Debug + Send + Sync {}

/// GErrSource - GErr as source of error.
///
/// This will be the type of GErr's sources,
/// be it from general error or from GErr itself.
///
/// You can add more detail attributes like GErr to other error types,
/// by implementing conversion from those errors to GErrSource.
///
/// Then, you can add it as detailed GErrSource into method [`crate::GErr::add_source_gerr`].
#[derive(Debug)]
pub struct GErrSource {
    /// Error id, must implement Debug and Display.
    pub id: Box<dyn IdSource>,

    /// Error id as json value, support for numeric and string only.
    ///
    /// Passing other than numeric and string, will be ignored at serde.
    #[cfg(feature = "serde")]
    pub id_json: serde_json::Value,

    /// Error message.
    pub message: Cow<'static, str>,

    /// Error prefix.
    pub prefix: Option<Cow<'static, str>>,

    /// Error sources.
    pub sources: Option<Vec<GErrSource>>,

    /// Error tags.
    pub tags: Option<Vec<Cow<'static, str>>>,

    /// Error data.
    pub data: Option<Box<dyn DataSource>>,

    /// Error help.
    pub help: Option<Cow<'static, str>>,

    /// Error data as json values. Supports all json values.
    #[cfg(feature = "serde")]
    pub data_json: Option<serde_json::Value>,

    /// Error location.
    pub location: Option<&'static Location<'static>>,
}

impl GErrSource {
    /// Constructs GErrSource from any error implementing [`Error`].
    ///
    /// If you pass GErrSource itself, it will be parsed with the attributes.
    pub fn from_error<E>(err: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let msg = err.to_string();
        let any = &err as &dyn Any;

        if any.downcast_ref::<GErrSource>().is_some() {
            let any: Box<dyn Any> = Box::new(err);
            if let Ok(owned_gerr_source) = any.downcast::<GErrSource>() {
                return Self {
                    id: owned_gerr_source.id,
                    #[cfg(feature = "serde")]
                    id_json: owned_gerr_source.id_json,
                    message: msg.into(),
                    prefix: owned_gerr_source.prefix,
                    sources: owned_gerr_source.sources,
                    tags: owned_gerr_source.tags,
                    data: owned_gerr_source.data,
                    help: owned_gerr_source.help,
                    #[cfg(feature = "serde")]
                    data_json: owned_gerr_source.data_json,
                    location: owned_gerr_source.location,
                };
            }
        }

        Self {
            id: Box::new(NoID),
            #[cfg(feature = "serde")]
            id_json: serde_json::Value::Null,
            message: msg.into(),
            prefix: None,
            sources: None,
            tags: None,
            data: None,
            help: None,
            #[cfg(feature = "serde")]
            data_json: None,
            location: None,
        }
    }
}

impl Display for GErrSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(prefix) = self.prefix.as_deref() {
            write!(f, "{prefix} {}", self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl Error for GErrSource {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(ref sources) = self.sources
            && !sources.is_empty()
        {
            return sources.first().map(|s| s as &(dyn Error + 'static));
        }
        None
    }
}
