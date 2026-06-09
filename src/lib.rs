#![no_std]

extern crate alloc;

use alloc::{borrow::Cow, boxed::Box, string::ToString, vec::Vec};
use core::{
    error::Error,
    fmt::{self, Debug, Display},
    panic::Location,
};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

#[cfg(feature = "uuid")]
pub type Id = uuid::Uuid;

#[cfg(not(feature = "uuid"))]
pub type Id = u128;

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

pub type GErr = Err<()>;

pub struct Err<D = ()> {
    id: Id,
    message: Cow<'static, str>,

    prefix: Option<&'static str>,
    source: Option<BoxError>,

    tags: Vec<Cow<'static, str>>,

    data: D,

    location: &'static Location<'static>,

    #[cfg(feature = "backtrace")]
    backtrace: Backtrace,
}

impl Err<()> {
    #[track_caller]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        Self {
            id: generate_id(),
            message: message.into(),

            prefix: None,
            source: None,

            tags: Vec::new(),

            data: (),

            location: Location::caller(),

            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),
        }
    }
}

impl<D> Err<D> {
    #[must_use]
    pub fn set_prefix(mut self, prefix: &'static str) -> Self {
        self.prefix = Some(prefix);
        self
    }

    #[must_use]
    pub fn set_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    #[must_use]
    pub fn set_tag<T>(mut self, tag: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.tags.push(tag.into());
        self
    }

    #[must_use]
    pub fn set_tags<I, T>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Cow<'static, str>>,
    {
        self.tags.extend(tags.into_iter().map(Into::into));
        self
    }

    #[must_use]
    pub fn set_data<T>(self, data: T) -> Err<T> {
        Err {
            id: self.id,
            message: self.message,

            prefix: self.prefix,
            source: self.source,

            tags: self.tags,

            data,

            location: self.location,

            #[cfg(feature = "backtrace")]
            backtrace: self.backtrace,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn prefix(&self) -> Option<&'static str> {
        self.prefix
    }

    pub fn source(&self) -> Option<&BoxError> {
        self.source.as_ref()
    }

    pub fn tags(&self) -> &[Cow<'static, str>] {
        &self.tags
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn location(&self) -> &'static Location<'static> {
        self.location
    }
}

impl<D> Display for Err<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.prefix, &self.source) {
            (None, None) => {
                write!(f, "{}", self.message)
            }
            (Some(prefix), None) => {
                write!(f, "[{}] {}", prefix, self.message)
            }
            (None, Some(source)) => {
                write!(f, "{}: {}", self.message, source)
            }
            (Some(prefix), Some(source)) => {
                write!(f, "[{}] {}: {}", prefix, self.message, source)
            }
        }
    }
}

impl<D: Debug> Debug for Err<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Err")
            .field("id", &self.id)
            .field("message", &self.message)
            .field("prefix", &self.prefix)
            .field("source", &self.source.as_ref().map(ToString::to_string))
            .field("tags", &self.tags)
            .field("data", &self.data)
            .field("location", &self.location)
            .finish()
    }
}

impl<D: Debug> Error for Err<D> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}

#[cfg(feature = "uuid")]
fn generate_id() -> Id {
    uuid::Uuid::new_v4()
}

#[cfg(not(feature = "uuid"))]
fn generate_id() -> Id {
    use core::sync::atomic::{AtomicU128, Ordering};

    static NEXT: AtomicU128 = AtomicU128::new(1);

    NEXT.fetch_add(1, Ordering::Relaxed)
}
