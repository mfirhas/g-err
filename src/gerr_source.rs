extern crate alloc;

use alloc::borrow::Cow;

use core::{
    any::Any,
    error::Error,
    fmt::{Debug, Display},
    panic::Location,
};

use crate::NoID;

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

    pub sources: Option<Vec<GErrSource>>,

    pub tags: Option<Vec<Cow<'static, str>>>,

    pub data: Option<Box<dyn DataSource>>,

    pub help: Option<Cow<'static, str>>,

    #[cfg(feature = "serde")]
    pub data_json: Option<serde_json::Value>,

    pub location: &'static Location<'static>,
}

impl GErrSource {
    #[track_caller]
    #[inline]
    pub fn from_error<E>(err: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let msg = err.to_string();
        let any = &err as &dyn Any;

        if let Some(_) = any.downcast_ref::<GErrSource>() {
            let any: Box<dyn Any> = Box::new(err);
            if let Ok(owned_gerr_source) = any.downcast::<GErrSource>() {
                return Self {
                    id: owned_gerr_source.id,
                    #[cfg(feature = "serde")]
                    id_json: owned_gerr_source.id_json,
                    message: owned_gerr_source.message,
                    prefix: owned_gerr_source.prefix,
                    sources: owned_gerr_source.sources,
                    tags: owned_gerr_source.tags,
                    data: owned_gerr_source.data,
                    help: owned_gerr_source.help,
                    #[cfg(feature = "serde")]
                    data_json: owned_gerr_source.data_json,
                    location: Location::caller(),
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
            location: Location::caller(),
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
            return sources
                .first()
                .and_then(|s| Some(&*s as &(dyn Error + 'static)));
        }
        None
    }
}
