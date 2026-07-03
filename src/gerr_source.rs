extern crate alloc;

use alloc::borrow::Cow;

use core::{
    any::Any,
    error::Error,
    fmt::{Debug, Display},
    panic::Location,
};

use crate::gerr::Source;

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
    pub sources: Option<Vec<Source>>,

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

impl Debug for GErrSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = f.debug_struct("GErrSource");

        debug.field("id", &self.id);

        #[cfg(feature = "serde")]
        debug.field("id_json", &self.id_json);

        debug
            .field("prefix", &self.prefix)
            .field("message", &self.message)
            .field("sources", &self.sources)
            .field("tags", &self.tags)
            .field("data", &self.data);

        #[cfg(feature = "serde")]
        debug.field("data_json", &self.data_json);

        debug
            .field("help", &self.help)
            .field("location", &self.location);

        debug.finish()
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
            return sources.first().map(|s| match s {
                Source::Err(e) => &**e as &(dyn Error + 'static),
                Source::GErr(e) => &**e as &(dyn Error + 'static),
            });
        }
        None
    }
}
