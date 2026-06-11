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

mod report;

#[cfg(feature = "serde")]
mod serde;

pub type Result<T, ID = (), P = (), D = ()> = core::result::Result<T, GErr<ID, P, D>>;

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

pub struct GErr<ID = (), P = (), D = ()> {
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

impl<ID: Id, P: Prefix, D> GErr<ID, P, D> {
    #[track_caller]
    #[inline]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        Self::new_untracked(message, Location::caller())
    }

    #[inline]
    pub(crate) fn new_untracked<M>(message: M, location: &'static Location<'static>) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        Self::with_id_untracked(ID::id(), message.into(), location)
    }
}

impl<ID, P: Prefix, D> GErr<ID, P, D> {
    #[track_caller]
    #[inline]
    pub fn with_id<M>(id: ID, message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        Self::with_id_untracked(id, message, Location::caller())
    }

    #[inline]
    pub(crate) fn with_id_untracked<M>(
        id: ID,
        message: M,
        location: &'static Location<'static>,
    ) -> Self
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

            location,

            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),

            _static_prefix: PhantomData,
        }
    }
}

impl<ID, P: Prefix, D> GErr<ID, P, D> {
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
    pub fn with_data<T>(self, data: T) -> GErr<ID, P, T> {
        GErr {
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

    // --- getter ---

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

    #[inline]
    pub fn result_err<T>(self) -> self::Result<T, ID, P, D> {
        Result::Err(self)
    }
}

impl<ID, P: Prefix, D> GErr<ID, P, D> {
    #[must_use]
    #[inline]
    pub fn set_field<K, V>(mut self, key: K, value: V) -> Self
    where
        D: Default + SetField<K, V>,
    {
        let data = self.data.get_or_insert_with(Default::default);
        data.set_field(key, value);
        self
    }
}

impl<ID, P: Prefix, D> Display for GErr<ID, P, D> {
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

impl<ID: Debug, P: Prefix, D: Debug> Debug for GErr<ID, P, D> {
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

impl<ID: Debug, P: Prefix, D: Debug> Error for GErr<ID, P, D> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}

// --- Result Extension
pub trait ResultExt<T> {
    #[must_use]
    #[track_caller]
    fn gerr<ID, P>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix;

    #[must_use]
    #[track_caller]
    fn wrap<ID, P: Prefix, D>(self, err: GErr<ID, P, D>) -> Result<T, ID, P, D>;
}

impl<T, E> ResultExt<T> for core::result::Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    #[track_caller]
    fn gerr<ID, P>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix,
    {
        let message = message.into();

        self.map_err(|source| {
            GErr::<ID, P, ()>::new_untracked(message, Location::caller()).set_source(source)
        })
    }

    #[track_caller]
    fn wrap<ID, P: Prefix, D>(self, err: GErr<ID, P, D>) -> Result<T, ID, P, D> {
        self.map_err(|source| err.set_source(source))
    }
}

pub struct Chain<'a> {
    current: Option<&'a (dyn Error + 'static)>,
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a (dyn Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = current.source();
        Some(current)
    }
}

impl<ID, P, D> GErr<ID, P, D>
where
    Self: Error + 'static,
{
    /// Returns an iterator over this error and all of its sources.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// for err in my_err.chain() {
    ///     println!("{err}");
    /// }
    /// ```
    pub fn chain(&self) -> Chain<'_> {
        Chain {
            current: Some(self as &(dyn Error + 'static)),
        }
    }

    /// Returns the deepest source in the error chain.
    ///
    /// If this error has no source, returns `self`.
    pub fn root_cause(&self) -> &(dyn Error + 'static) {
        let mut current: &(dyn Error + 'static) = self;

        while let Some(src) = current.source() {
            current = src;
        }

        current
    }
}

impl<ID, P: Prefix, D> GErr<ID, P, D>
where
    Self: Error + 'static,
{
    fn find_in_chain<T: Error + 'static>(&self) -> Option<&T> {
        self.chain().find_map(|e| e.downcast_ref::<T>())
    }

    pub fn is<T>(&self) -> bool
    where
        T: Error + 'static,
    {
        self.find_in_chain::<T>().is_some()
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Error + 'static,
    {
        self.find_in_chain::<T>()
    }
}
