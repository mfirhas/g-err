#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

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
pub use report::{MarkdownReport, PrettyReport, TraceReport};

#[cfg(feature = "serde")]
pub use report::{
    DisplayJsonData, DisplayJsonReport, JsonData, JsonReport, LocationJsonData, SourceJsonData,
};

#[cfg(feature = "serde")]
pub mod serde;

mod macros;

mod types;
pub use types::*;

mod result_ext;
pub use result_ext::{GResultExt, ResultExt};

mod sealed {
    pub trait Sealed {}
}

pub struct GErr<ID = NoID, P = NoPrefix, D = NoData> {
    id: ID,
    message: Cow<'static, str>,

    prefix: Option<Cow<'static, str>>,
    sources: Option<Vec<Source>>,

    tags: Option<Vec<Cow<'static, str>>>,

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
    pub fn new_with_id<M>(id: ID, message: M) -> Self
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

            prefix: P::PREFIX.map(Cow::Borrowed),
            sources: None,

            tags: None,

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
    pub fn set_id(mut self, id: ID) -> Self {
        self.id = id;
        self
    }

    #[must_use]
    #[inline]
    pub fn set_prefix<T>(mut self, prefix: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.prefix = Some(prefix.into());
        self
    }

    #[must_use]
    #[inline]
    pub fn set_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.sources = Some(vec![Source::Err(Box::new(source))]);
        self
    }

    #[must_use]
    #[inline]
    pub fn add_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.sources
            .get_or_insert_default()
            .push(Source::Err(Box::new(source)));
        self
    }

    #[must_use]
    #[inline]
    pub fn set_source_gerr<E>(mut self, source: E) -> Self
    where
        E: Into<GErrSource> + Error + Send + Sync + 'static,
    {
        self.sources = Some(vec![Source::GErr(Box::new(source.into()))]);
        self
    }

    #[inline]
    pub fn add_source_gerr<E>(mut self, gerr: E) -> Self
    where
        E: Into<GErrSource> + Error + Send + Sync + 'static,
    {
        self.sources
            .get_or_insert_default()
            .push(Source::GErr(Box::new(gerr.into())));
        self
    }

    #[must_use]
    pub fn set_tag<T>(mut self, tag: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.tags.get_or_insert_default().push(tag.into());
        self
    }

    #[must_use]
    pub fn set_tags<I, T>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Cow<'static, str>>,
    {
        self.tags
            .get_or_insert_default()
            .extend(tags.into_iter().map(Into::into));
        self
    }

    #[must_use]
    #[inline]
    pub fn set_data(mut self, data: D) -> Self {
        self.data = Some(data);
        self
    }

    /// Override ID value and probably type.
    #[must_use]
    #[inline]
    pub fn with_id<T>(self, id: T) -> GErr<T, P, D> {
        GErr {
            id,
            message: self.message,

            prefix: self.prefix,
            sources: self.sources,

            tags: self.tags,

            data: self.data,

            location: self.location,

            #[cfg(feature = "backtrace")]
            backtrace: self.backtrace,

            _static_prefix: PhantomData,
        }
    }

    /// Override Prefix(P) value and probably type.
    #[must_use]
    #[inline]
    pub fn with_prefix<T: Prefix>(self) -> GErr<ID, T, D> {
        GErr {
            id: self.id,
            message: self.message,

            prefix: T::PREFIX.map(Cow::Borrowed),
            sources: self.sources,

            tags: self.tags,

            data: self.data,

            location: self.location,

            #[cfg(feature = "backtrace")]
            backtrace: self.backtrace,

            _static_prefix: PhantomData,
        }
    }

    /// Override Data(D) value and probably type.
    #[must_use]
    #[inline]
    pub fn with_data<T>(self, data: T) -> GErr<ID, P, T> {
        GErr {
            id: self.id,
            message: self.message,

            prefix: self.prefix,
            sources: self.sources,

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
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref().or(P::PREFIX.as_deref())
    }

    #[inline]
    pub fn tags(&self) -> Option<&[Cow<'static, str>]> {
        self.tags.as_deref()
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
    pub fn result<T>(self) -> self::Result<T, ID, P, D> {
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
        if let Some(prefix) = self.prefix.as_deref().or(P::PREFIX.as_deref()) {
            write!(f, "{prefix} {}", self.message())
        } else {
            write!(f, "{}", self.message())
        }
    }
}

impl<ID: Debug, P: Prefix, D: Debug> Debug for GErr<ID, P, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Err");

        debug
            .field("id", &self.id)
            .field("prefix", &self.prefix.as_deref().or(P::PREFIX.as_deref()))
            .field("message", &self.message)
            .field("source", &self.sources)
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

pub trait IdSource: Debug + Display + Send + Sync {}

impl<T> IdSource for T where T: Debug + Display + Send + Sync {}

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

    pub data: Option<Box<dyn Debug + Send + Sync>>,

    #[cfg(feature = "serde")]
    pub data_json: Option<serde_json::Value>,

    pub location: &'static Location<'static>,
}

impl Display for GErrSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[cfg(not(feature = "serde"))]
impl<ID, P, D> From<GErr<ID, P, D>> for GErrSource
where
    ID: IdSource + 'static,
    D: Debug + Send + Sync + 'static,
{
    fn from(gerr: GErr<ID, P, D>) -> Self {
        GErrSource {
            id: Box::new(gerr.id),
            message: gerr.message,
            prefix: gerr.prefix,
            tags: gerr.tags,
            data: gerr
                .data
                .map(|d| Box::new(d) as Box<dyn Debug + Send + Sync>),
            location: gerr.location,
            sources: gerr.sources,
        }
    }
}

#[cfg(feature = "serde")]
impl<ID, P, D> From<GErr<ID, P, D>> for GErrSource
where
    ID: ::serde::Serialize + IdSource + 'static,
    D: ::serde::Serialize + Debug + Send + Sync + 'static,
{
    fn from(gerr: GErr<ID, P, D>) -> Self {
        GErrSource {
            id_json: serde_json::to_value(&gerr.id).unwrap_or_default(),
            id: Box::new(gerr.id),
            message: gerr.message,
            prefix: gerr.prefix,
            tags: gerr.tags,
            data_json: gerr
                .data
                .as_ref()
                .map(|d| serde_json::to_value(d).unwrap_or_default()),
            data: gerr
                .data
                .map(|d| Box::new(d) as Box<dyn Debug + Send + Sync>),
            location: gerr.location,
            sources: gerr.sources,
        }
    }
}
