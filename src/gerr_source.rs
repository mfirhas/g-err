extern crate alloc;

use alloc::borrow::Cow;

use core::{
    any::Any,
    error::Error,
    fmt::{Debug, Display},
    panic::Location,
};

use crate::gerr::Source;

pub trait IdSource: Any + Debug + Display + Send + Sync {}

impl<T> IdSource for T where T: Any + Debug + Display + Send + Sync {}

pub trait DataSource: Any + Debug + Send + Sync {}

impl<T> DataSource for T where T: Any + Debug + Send + Sync {}

// --- GErrSource, before converting GErr to dyn Err
#[derive(Debug)]
pub struct GErrSource {
    pub id: Box<dyn IdSource>,

    #[cfg(feature = "serde")]
    pub id_json: serde_json::Value,

    pub message: Cow<'static, str>,

    pub prefix: Option<Cow<'static, str>>,

    pub sources: Option<Vec<Source>>,

    pub tags: Option<Vec<Cow<'static, str>>>,

    pub data: Option<Box<dyn DataSource>>,

    pub help: Option<Cow<'static, str>>,

    #[cfg(feature = "serde")]
    pub data_json: Option<serde_json::Value>,

    pub location: &'static Location<'static>,
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
            return sources.first().and_then(|s| match s {
                Source::Err(e) => Some(&**e as &(dyn Error + 'static)),
                Source::GErr(e) => Some(&**e as &(dyn Error + 'static)),
            });
        }
        None
    }
}
