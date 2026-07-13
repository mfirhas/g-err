extern crate alloc;

use crate::GErrSource;
use crate::{
    Config,
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
    fn context<C: Config, D>(
        self,
        id: C::Id,
        message: impl Into<Cow<'static, str>>,
    ) -> Result<T, C, D>;

    /// Wrap `E` inside GErr as source with auto-generated id.
    #[track_caller]
    fn context_auto<C: Config, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, C, D>;

    /// Wrap `E` inside GErr
    #[track_caller]
    fn wrap_err<C: Config, D>(self, gerr: GErr<C, D>) -> Result<T, C, D>;
}

impl<T, E> ResultExt<T> for core::result::Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    #[track_caller]
    fn context<C: Config, D>(
        self,
        id: C::Id,
        message: impl Into<Cow<'static, str>>,
    ) -> Result<T, C, D> {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<C, D>::new_with_id_untracked(Some(id), message.into(), location)
                .add_source(source)
        })
    }

    #[track_caller]
    fn context_auto<C: Config, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, C, D> {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<C, D>::new_untracked(message.into(), location).add_source(source)
        })
    }

    #[track_caller]
    fn wrap_err<C: Config, D>(self, gerr: GErr<C, D>) -> Result<T, C, D> {
        self.map_err(|err| gerr.add_source(err))
    }
}

/// Result extension where `E` is GErr or any error implementing `Into<GErrSource>`.
///
/// Use this extension if you pass GErr or `Into<GErrSource>` from Result's E and want to keep the detail attributes.
pub trait GResultExt<T, E>: sealed::Sealed {
    /// Wrap `E` as GErr's source, where E is `Into<GErrSource>`, and id is manually-set.
    #[track_caller]
    fn gerr<C: Config, D>(
        self,
        id: C::Id,
        message: impl Into<Cow<'static, str>>,
    ) -> Result<T, C, D>;

    /// Wrap `E` as GErr's source, where E is `Into<GErrSource>`, and id is auto-generated.
    #[track_caller]
    fn gerr_auto<C: Config, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, C, D>;

    /// Wrap `E` inside GErr
    #[track_caller]
    fn wrap_gerr<C: Config, D>(self, gerr: GErr<C, D>) -> Result<T, C, D>;

    /// Box `E`
    fn boxed(self) -> core::result::Result<T, Box<E>>;
}

impl<T, E> GResultExt<T, E> for core::result::Result<T, E>
where
    E: Into<GErrSource> + Error + Send + Sync + 'static,
{
    #[track_caller]
    fn gerr<C: Config, D>(
        self,
        id: C::Id,
        message: impl Into<Cow<'static, str>>,
    ) -> Result<T, C, D> {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<C, D>::new_with_id_untracked(Some(id), message.into(), location)
                .add_source_gerr(source)
        })
    }

    #[track_caller]
    fn gerr_auto<C: Config, D>(self, message: impl Into<Cow<'static, str>>) -> Result<T, C, D> {
        let location = Location::caller();
        self.map_err(|source| {
            GErr::<C, D>::new_untracked(message.into(), location).add_source_gerr(source)
        })
    }

    /// Wrap `E` inside GErr
    #[track_caller]
    fn wrap_gerr<C: Config, D>(self, gerr: GErr<C, D>) -> Result<T, C, D> {
        self.map_err(|err| gerr.add_source_gerr(err))
    }

    #[inline]
    fn boxed(self) -> core::result::Result<T, Box<E>> {
        self.map_err(Box::new)
    }
}
