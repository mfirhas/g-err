extern crate alloc;

use crate::{GErr, GErrSource, Id, NoData, Prefix, Result, sealed};
use core::error::Error;
use core::panic::Location;

use alloc::borrow::Cow;

impl<T, E> sealed::Sealed for core::result::Result<T, E> {}

pub trait ResultExt<T>: sealed::Sealed {
    #[must_use]
    #[track_caller]
    fn context<ID, P>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix;

    #[must_use]
    #[track_caller]
    fn with_context<ID, P, F, M>(self, func: F) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>;

    #[must_use]
    #[track_caller]
    fn wrap_err<ID, P: Prefix, D>(self, err: GErr<ID, P, D>) -> Result<T, ID, P, D>;
}

impl<T, E> ResultExt<T> for core::result::Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    #[track_caller]
    fn context<ID, P>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix,
    {
        let message = message.into();
        let location = Location::caller();

        self.map_err(|source| {
            GErr::<ID, P, NoData>::new_untracked(message, location).add_source(source)
        })
    }

    #[track_caller]
    fn with_context<ID, P, F, M>(self, func: F) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>,
    {
        let message = func().into();
        let location = Location::caller();

        self.map_err(|source| {
            GErr::<ID, P, NoData>::new_untracked(message, location).add_source(source)
        })
    }

    #[track_caller]
    fn wrap_err<ID, P: Prefix, D>(self, err: GErr<ID, P, D>) -> Result<T, ID, P, D> {
        self.map_err(|source| err.add_source(source))
    }
}

pub trait GResultExt<T>: sealed::Sealed {
    #[must_use]
    #[track_caller]
    fn gerr<ID, P>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix;

    #[must_use]
    #[track_caller]
    fn with_gerr<ID, P, F, M>(self, func: F) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>;

    #[must_use]
    #[track_caller]
    fn wrap_gerr<ID, P: Prefix, D>(self, err: GErr<ID, P, D>) -> Result<T, ID, P, D>;
}

impl<T, E> GResultExt<T> for core::result::Result<T, E>
where
    E: Into<GErrSource> + Error + Send + Sync + 'static,
{
    #[track_caller]
    fn gerr<ID, P>(self, message: impl Into<Cow<'static, str>>) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix,
    {
        let message = message.into();
        let location = Location::caller();

        self.map_err(|source| {
            GErr::<ID, P, NoData>::new_untracked(message, location).add_source_gerr(source)
        })
    }

    #[track_caller]
    fn with_gerr<ID, P, F, M>(self, func: F) -> Result<T, ID, P>
    where
        ID: Id,
        P: Prefix,
        F: FnOnce() -> M,
        M: Into<Cow<'static, str>>,
    {
        let message = func().into();
        let location = Location::caller();

        self.map_err(|source| {
            GErr::<ID, P, NoData>::new_untracked(message, location).add_source_gerr(source)
        })
    }

    #[track_caller]
    fn wrap_gerr<ID, P: Prefix, D>(self, err: GErr<ID, P, D>) -> Result<T, ID, P, D> {
        self.map_err(|source| err.add_source_gerr(source))
    }
}
