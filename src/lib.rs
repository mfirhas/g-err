#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use core::{
    error::Error,
    fmt::{self, Debug, Display},
    marker::PhantomData,
    panic::Location,
};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

pub type Result<T, ID = (), P = (), D = ()> = core::result::Result<T, Err<ID, P, D>>;

pub trait Id {
    fn id() -> Self;
}

impl Id for () {
    #[inline]
    fn id() {}
}

pub trait Prefix {
    const PREFIX: Option<&'static str> = None;
}

impl Prefix for () {}

pub trait SetField<K, V> {
    fn set_field(&mut self, key: K, value: V);
}

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

pub struct Err<ID = (), P = (), D = ()> {
    id: ID,
    message: Cow<'static, str>,

    prefix: Option<&'static str>,
    source: Option<BoxError>,

    tags: Vec<Cow<'static, str>>,

    data: Option<D>,

    location: &'static Location<'static>,

    #[cfg(feature = "backtrace")]
    backtrace: Backtrace,

    _static_prefix: PhantomData<P>,
}

impl<ID: Id, P: Prefix, D> Err<ID, P, D> {
    #[track_caller]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        Self::with_id(ID::id(), message.into())
    }
}

impl<ID, P: Prefix, D> Err<ID, P, D> {
    #[track_caller]
    #[inline]
    pub fn with_id<M>(id: ID, message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        Self {
            id,
            message: message.into(),

            prefix: P::PREFIX,
            source: None,

            tags: Vec::new(),

            data: None,

            location: Location::caller(),

            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),

            _static_prefix: PhantomData,
        }
    }
}

impl<ID, P: Prefix, D> Err<ID, P, D> {
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
    pub fn set_data<T>(self, data: T) -> Err<ID, P, T> {
        Err {
            id: self.id,
            message: self.message,

            prefix: self.prefix,
            source: self.source,

            tags: self.tags,

            data: Some(data),

            location: self.location,

            #[cfg(feature = "backtrace")]
            backtrace: self.backtrace,

            _static_prefix: PhantomData,
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
        self.prefix.or(P::PREFIX)
    }

    #[inline]
    pub fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as &(dyn Error + 'static))
    }

    #[inline]
    pub fn tags(&self) -> &[Cow<'static, str>] {
        &self.tags
    }

    #[inline]
    pub fn data(&self) -> Option<&D> {
        self.data.as_ref()
    }

    #[inline]
    pub fn location(&self) -> &'static Location<'static> {
        self.location
    }
}

impl<ID, P: Prefix, D> Err<ID, P, D> {
    #[must_use]
    #[inline]
    pub fn set_field<K, V>(mut self, key: K, value: V) -> Self
    where
        D: SetField<K, V>,
    {
        if let Some(ref mut data) = self.data {
            data.set_field(key, value);
        }
        self
    }
}

impl<ID, P: Prefix, D> Display for Err<ID, P, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = self.prefix.or(P::PREFIX);

        match (prefix, &self.source) {
            (None, None) => {
                write!(f, "{}", self.message)
            }
            (Some(prefix), None) => {
                write!(f, "{} {}", prefix, self.message)
            }
            (None, Some(source)) => {
                write!(f, "{}: {}", self.message, source)
            }
            (Some(prefix), Some(source)) => {
                write!(f, "{} {}: {}", prefix, self.message, source)
            }
        }
    }
}

impl<ID: Debug, P: Prefix, D: Debug> Debug for Err<ID, P, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Err");

        debug
            .field("id", &self.id)
            .field("prefix", &self.prefix.or(P::PREFIX))
            .field("message", &self.message)
            .field("source", &self.source)
            .field("tags", &self.tags)
            .field("data", &self.data)
            .field("location", &self.location);

        #[cfg(feature = "backtrace")]
        debug.field("backtrace", &self.backtrace);

        debug.finish()
    }
}

impl<ID: Debug, P: Prefix, D: Debug> Error for Err<ID, P, D> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}

// --- Result Extension
pub trait ResultExt<T> {
    #[must_use]
    #[track_caller]
    fn err<ID, P>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix;

    #[must_use]
    #[track_caller]
    fn wrap<ID, P: Prefix, D>(self, err: Err<ID, P, D>) -> Result<T, ID, P, D>;
}

impl<T, E> ResultExt<T> for core::result::Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    #[track_caller]
    fn err<ID, P>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix,
    {
        let message = message.into();

        self.map_err(|source| Err::<ID, P>::new(message).set_source(source))
    }

    #[track_caller]
    fn wrap<ID, P: Prefix, D>(self, err: Err<ID, P, D>) -> Result<T, ID, P, D> {
        self.map_err(|source| err.set_source(source))
    }
}
