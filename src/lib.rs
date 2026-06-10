#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use core::{
    error::Error,
    fmt::{self, Debug, Display},
    panic::Location,
};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

pub trait Id {
    fn id() -> Self;
}

impl Id for () {
    #[inline]
    fn id() {}
}

pub trait SetField<K, V> {
    fn set_field(&mut self, key: K, value: V);
}

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

pub struct Err<ID = (), D = ()> {
    id: ID,
    message: Cow<'static, str>,

    prefix: Option<&'static str>,
    source: Option<BoxError>,

    tags: Vec<Cow<'static, str>>,

    data: D,

    location: &'static Location<'static>,

    #[cfg(feature = "backtrace")]
    backtrace: Backtrace,
}

impl<ID: Id> Err<ID> {
    #[track_caller]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        Self::with_id(ID::id(), message.into())
    }
}

impl<ID> Err<ID> {
    #[track_caller]
    #[inline]
    pub fn with_id<M>(id: ID, message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        Self {
            id,
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

impl<ID, D> Err<ID, D> {
    #[must_use]
    #[inline]
    pub fn set_prefix(mut self, prefix: &'static str) -> Self {
        self.prefix = Some(prefix);
        self
    }

    #[must_use]
    #[inline]
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
    #[inline]
    pub fn set_data<T>(self, data: T) -> Err<ID, T> {
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

    #[inline]
    pub fn id(&self) -> &ID {
        &self.id
    }

    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[inline]
    pub fn prefix(&self) -> Option<&'static str> {
        self.prefix
    }

    #[inline]
    pub fn source(&self) -> Option<&BoxError> {
        self.source.as_ref()
    }

    #[inline]
    pub fn tags(&self) -> &[Cow<'static, str>] {
        &self.tags
    }

    #[inline]
    pub fn data(&self) -> &D {
        &self.data
    }

    #[inline]
    pub fn location(&self) -> &'static Location<'static> {
        self.location
    }
}

impl<ID, D> Err<ID, D> {
    pub fn set_field<K, V>(mut self, key: K, value: V) -> Self
    where
        D: SetField<K, V>,
    {
        self.data.set_field(key, value);
        self
    }
}

impl<ID, D> Display for Err<ID, D> {
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

impl<ID: Debug, D: Debug> Debug for Err<ID, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Err")
            .field("id", &self.id)
            .field("message", &self.message)
            .field("prefix", &self.prefix)
            .field("source", &self.source)
            .field("tags", &self.tags)
            .field("data", &self.data)
            .field("location", &self.location)
            .finish()
    }
}

impl<ID: Debug, D: Debug> Error for Err<ID, D> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}
