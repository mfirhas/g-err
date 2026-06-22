extern crate alloc;

use crate::gerr_source::DataSource;
use crate::gerr_source::GErrSource;
use crate::gerr_source::IdSource;
use crate::types::{NoData, NoID, NoPrefix};

use core::{
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
    panic::Location,
};

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

use alloc::borrow::Cow;

pub trait Id {
    fn id() -> Self;
}

pub trait Prefix {
    const PREFIX: Option<&'static str> = None;
}

pub trait SetField<K, V> {
    fn set_field(&mut self, key: K, value: V);
}

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum Source {
    Err(BoxError),
    GErr(Box<GErrSource>),
}

pub type Result<T, ID = NoID, P = NoPrefix, D = NoData> = core::result::Result<T, GErr<ID, P, D>>;

pub struct GErr<ID = NoID, P = NoPrefix, D = NoData> {
    id: ID,
    message: Cow<'static, str>,

    prefix: Option<Cow<'static, str>>,
    sources: Option<Vec<Source>>,

    tags: Option<Vec<Cow<'static, str>>>,

    data: Option<D>,

    help: Option<Cow<'static, str>>,

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

    #[track_caller]
    #[inline]
    pub fn from_error<E>(err: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::new_untracked(err.to_string(), Location::caller()).add_source(err)
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

            help: None,

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
    pub fn set_sources<I>(mut self, sources: I) -> Self
    where
        I: IntoIterator<Item = Source>,
    {
        self.sources = Some(sources.into_iter().map(Into::into).collect());
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
    pub fn add_tag<T>(mut self, tag: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.tags.get_or_insert_default().push(tag.into());
        self
    }

    #[must_use]
    pub fn add_tags<I, T>(mut self, tags: I) -> Self
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

    #[must_use]
    #[inline]
    pub fn set_help<H>(mut self, help: H) -> Self
    where
        H: Into<Cow<'static, str>>,
    {
        self.help = Some(help.into());
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

            help: self.help,

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

            help: self.help,

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

            help: self.help,

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
    pub fn sources(&self) -> Option<&[Source]> {
        self.sources.as_deref()
    }

    #[inline]
    pub fn location(&self) -> &'static Location<'static> {
        self.location
    }

    #[inline]
    pub fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    #[cfg(feature = "backtrace")]
    #[inline]
    pub fn backtrace(&self) -> &std::backtrace::Backtrace {
        &self.backtrace
    }

    #[inline]
    pub fn result<T>(self) -> Result<T, ID, P, D> {
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(prefix) = self.prefix.as_deref().or(P::PREFIX.as_deref()) {
            write!(f, "{prefix} {}", self.message())
        } else {
            write!(f, "{}", self.message())
        }
    }
}

impl<ID: Debug, P: Prefix, D: Debug> Debug for GErr<ID, P, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = f.debug_struct("Err");

        debug
            .field("id", &self.id)
            .field("prefix", &self.prefix.as_deref().or(P::PREFIX.as_deref()))
            .field("message", &self.message)
            .field("source", &self.sources)
            .field("tags", &self.tags)
            .field("data", &self.data)
            .field("help", &self.help())
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

#[cfg(not(feature = "serde"))]
impl<ID, P, D> From<GErr<ID, P, D>> for GErrSource
where
    ID: IdSource + 'static,
    D: DataSource + 'static,
{
    fn from(gerr: GErr<ID, P, D>) -> Self {
        GErrSource {
            id: Box::new(gerr.id),
            message: gerr.message,
            prefix: gerr.prefix,
            tags: gerr.tags,
            data: gerr.data.map(|d| Box::new(d) as Box<dyn DataSource>),
            help: gerr.help,
            location: gerr.location,
            sources: gerr.sources,
        }
    }
}

#[cfg(feature = "serde")]
impl<ID, P, D> From<GErr<ID, P, D>> for GErrSource
where
    ID: ::serde::Serialize + IdSource + 'static,
    P: Prefix,
    D: ::serde::Serialize + DataSource + 'static,
{
    fn from(gerr: GErr<ID, P, D>) -> Self {
        GErrSource {
            id_json: serde_json::to_value(gerr.id()).unwrap_or_default(),
            id: Box::new(gerr.id),
            message: gerr.message,
            prefix: gerr.prefix,
            tags: gerr.tags,
            data_json: gerr
                .data
                .as_ref()
                .map(|d| serde_json::to_value(d).unwrap_or_default()),
            data: gerr.data.map(|d| Box::new(d) as Box<dyn DataSource>),
            help: gerr.help,
            location: gerr.location,
            sources: gerr.sources,
        }
    }
}
