extern crate alloc;

use alloc::boxed::Box;

use crate::types::DefaultConfig;
use crate::{Config, GErr, GErrSource, GErrView, NoData};
use crate::{DataSource, IdSource};

/// Alias for Box of GErr.
///
/// For smaller size error type, but heap-allocated, use this.
///
/// Just like GErr, C(Config) is defaulted to DefaultConfig and D(Data) is defaulted to NoData.
pub type GErrBox<C = DefaultConfig, D = NoData> = Box<GErr<C, D>>;

#[cfg(not(feature = "serde"))]
impl<C: Config, D> From<Box<GErr<C, D>>> for GErrSource
where
    C::Id: IdSource + 'static,
    D: DataSource + 'static,
{
    fn from(gerr: Box<GErr<C, D>>) -> Self {
        (*gerr).into()
    }
}

#[cfg(feature = "serde")]
impl<C: Config, D> From<Box<GErr<C, D>>> for GErrSource
where
    C::Id: ::serde::Serialize + IdSource + 'static,
    D: ::serde::Serialize + DataSource + 'static,
{
    fn from(gerr: Box<GErr<C, D>>) -> Self {
        (*gerr).into()
    }
}

impl<'a, C: Config, D> From<&'a GErrBox<C, D>> for GErrView<'a, C, D> {
    fn from(err: &'a GErrBox<C, D>) -> Self {
        (&**err).into()
    }
}

impl<T, C: Config, D> From<GErrBox<C, D>> for core::result::Result<T, GErrBox<C, D>> {
    #[inline]
    fn from(value: GErrBox<C, D>) -> Self {
        core::result::Result::Err(value)
    }
}
