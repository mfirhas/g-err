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
    /// Wrap `E` inside GErr as source with auto-generated id.
    #[track_caller]
    fn context<ID, P, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix;

    /// Wrap `E` inside GErr as source with auto-generated id, with a closure generating the error message.
    #[track_caller]
    fn with_context<ID, P, D, F, M>(self, func: F) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>;

    /// Returns any `E` as GErr with auto-generated id.
    ///
    /// Useful if the return type is GErr, accepting any errors.
    ///
    /// Make sure GErr return type ID: Id and P: Prefix.
    #[track_caller]
    fn into_gerr<ID, P, D>(self) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix;
}

impl<T, E> ResultExt<T> for core::result::Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    #[track_caller]
    fn context<ID, P, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
    {
        let message = message.into();
        let location = Location::caller();

        self.map_err(|source| GErr::<ID, P, D>::new_untracked(message, location).add_source(source))
    }

    #[track_caller]
    fn with_context<ID, P, D, F, M>(self, func: F) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>,
    {
        let message = func().into();
        let location = Location::caller();

        self.map_err(|source| GErr::<ID, P, D>::new_untracked(message, location).add_source(source))
    }

    #[track_caller]
    fn into_gerr<ID, P, D>(self) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
    {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<ID, P, D>::with_id_untracked(ID::id(), source.to_string(), location)
        })
    }
}

/// Result extension where `E` is GErr or any error implementing `Into<GErrSource>`.
///
/// Use this extension if you pass GErr or `Into<GErrSource>` from Result's E and want to keep the detail attributes.
pub trait GResultExt<T>: sealed::Sealed {
    /// Wrap `E` as GErr's source, where E is `Into<GErrSource>`.
    #[track_caller]
    fn gerr<ID, P, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix;

    /// Wrap `E` as GErr's source, where E is `Into<GErrSource>`, with closure producing error message.
    #[track_caller]
    fn with_gerr<ID, P, D, F, M>(self, func: F) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>;
}

impl<T, E> GResultExt<T> for core::result::Result<T, E>
where
    E: Into<GErrSource> + Error + Send + Sync + 'static,
{
    #[track_caller]
    fn gerr<ID, P, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
    {
        let message = message.into();
        let location = Location::caller();

        self.map_err(|source| {
            GErr::<ID, P, D>::new_untracked(message, location).add_source_gerr(source.into())
        })
    }

    #[track_caller]
    fn with_gerr<ID, P, D, F, M>(self, func: F) -> Result<T, ID, P, D>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>,
    {
        let message = func().into();
        let location = Location::caller();

        self.map_err(|source| {
            GErr::<ID, P, D>::new_untracked(message, location).add_source_gerr(source.into())
        })
    }
}
