extern crate alloc;

use crate::GErrSource;
use crate::gerr::{Id, Prefix};
use crate::{
    gerr::{GErr, Result},
    sealed,
};
use core::error::Error;
use core::panic::Location;

use alloc::borrow::Cow;

impl<T, E> sealed::Sealed for core::result::Result<T, E> {}

/// Extension for Result wrapping `E` as GErr's source.
///
/// This is for general error, including GErr itself.
///
/// Passing GErr will be parsed as general error.
pub trait ResultExt<T>: sealed::Sealed {
    /// Wrap `E` inside GErr as source with manually-set id.
    #[track_caller]
    fn context<ID, P, D>(
        self,
        id: ID,
        message: impl Into<Cow<'static, str>>,
    ) -> Result<T, ID, P, D>
    where
        P: Prefix;

    /// Wrap `E` inside GErr as source with manually-set id, with a closure generating the error message.
    #[track_caller]
    fn with_context<ID, P, D, F, M>(self, id: ID, func: F) -> Result<T, ID, P, D>
    where
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>;

    /// Wrap `E` inside GErr as source with auto-generated id.
    #[track_caller]
    fn context_auto<ID, P, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix;

    /// Wrap `E` inside GErr as source with auto-generated id, with a closure generating the error message.
    #[track_caller]
    fn with_context_auto<ID, P, D, F, M>(self, func: F) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>;
}

impl<T, E> ResultExt<T> for core::result::Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    #[track_caller]
    fn context<ID, P, D>(self, id: ID, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        P: Prefix,
    {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<ID, P, D>::with_id_untracked(id, message.into(), location).add_source(source)
        })
    }

    #[track_caller]
    fn with_context<ID, P, D, F, M>(self, id: ID, func: F) -> Result<T, ID, P, D>
    where
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>,
    {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<ID, P, D>::with_id_untracked(id, func().into(), location).add_source(source)
        })
    }

    #[track_caller]
    fn context_auto<ID, P, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
    {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<ID, P, D>::new_untracked(message.into(), location).add_source(source)
        })
    }

    #[track_caller]
    fn with_context_auto<ID, P, D, F, M>(self, func: F) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>,
    {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<ID, P, D>::new_untracked(func().into(), location).add_source(source)
        })
    }
}

/// Result extension where `E` is GErr or any error implementing `Into<GErrSource>`.
///
/// Use this extension if you pass GErr or `Into<GErrSource>` from Result's E and want to keep the detail attributes.
pub trait GResultExt<T, E>: sealed::Sealed {
    /// Wrap `E` as GErr's source, where E is `Into<GErrSource>`, and id is manually-set.
    #[track_caller]
    fn gerr<ID, P, D>(self, id: ID, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        P: Prefix;

    /// Wrap `E` as GErr's source, where E is `Into<GErrSource>`, with closure producing error message, and id is manually-set.
    #[track_caller]
    fn with_gerr<ID, P, D, F, M>(self, id: ID, func: F) -> Result<T, ID, P, D>
    where
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>;

    /// Wrap `E` as GErr's source, where E is `Into<GErrSource>`, and id is auto-generated.
    #[track_caller]
    fn gerr_auto<ID, P, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix;

    /// Wrap `E` as GErr's source, where E is `Into<GErrSource>`, with closure producing error message, and id is auto-generated.
    #[track_caller]
    fn with_gerr_auto<ID, P, D, F, M>(self, func: F) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>;

    /// Box `E`
    fn boxed(self) -> core::result::Result<T, Box<E>>;
}

impl<T, E> GResultExt<T, E> for core::result::Result<T, E>
where
    E: Into<GErrSource> + Error + Send + Sync + 'static,
{
    #[track_caller]
    fn gerr<ID, P, D>(self, id: ID, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        P: Prefix,
    {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<ID, P, D>::with_id_untracked(id, message.into(), location)
                .add_source_gerr(source)
        })
    }

    #[track_caller]
    fn with_gerr<ID, P, D, F, M>(self, id: ID, func: F) -> Result<T, ID, P, D>
    where
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>,
    {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<ID, P, D>::with_id_untracked(id, func().into(), location).add_source_gerr(source)
        })
    }

    #[track_caller]
    fn gerr_auto<ID, P, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
    {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<ID, P, D>::new_untracked(message.into(), location).add_source_gerr(source)
        })
    }

    #[track_caller]
    fn with_gerr_auto<ID, P, D, F, M>(self, func: F) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>,
    {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<ID, P, D>::new_untracked(func().into(), location).add_source_gerr(source)
        })
    }

    #[inline]
    fn boxed(self) -> core::result::Result<T, Box<E>> {
        self.map_err(Box::new)
    }
}
