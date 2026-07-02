extern crate alloc;

use crate::{DataSource, IdSource, Prefix};
use crate::{GErr, GErrSource, GErrView, NoData, NoID, NoPrefix};

/// Alias for Box of GErr.
///
/// For smaller size error type, but heap-allocated, use this.
///
/// Just like GErr, ID is defaulted to NoID, Prefix is defaulted to NoPrefix, and D is defaulted to NoData.
pub type GErrBox<ID = NoID, P = NoPrefix, D = NoData> = Box<GErr<ID, P, D>>;

#[cfg(not(feature = "serde"))]
impl<ID, P, D> From<Box<GErr<ID, P, D>>> for GErrSource
where
    ID: IdSource + 'static,
    D: DataSource + 'static,
{
    fn from(gerr: Box<GErr<ID, P, D>>) -> Self {
        (*gerr).into()
    }
}

#[cfg(feature = "serde")]
impl<ID, P, D> From<Box<GErr<ID, P, D>>> for GErrSource
where
    ID: ::serde::Serialize + IdSource + 'static,
    P: Prefix,
    D: ::serde::Serialize + DataSource + 'static,
{
    fn from(gerr: Box<GErr<ID, P, D>>) -> Self {
        (*gerr).into()
    }
}

impl<'a, ID, P: Prefix, D> From<&'a GErrBox<ID, P, D>> for GErrView<'a, ID, D> {
    fn from(err: &'a GErrBox<ID, P, D>) -> Self {
        (&**err).into()
    }
}

impl<T, ID, P, D> From<GErrBox<ID, P, D>> for core::result::Result<T, GErrBox<ID, P, D>> {
    #[inline]
    fn from(value: GErrBox<ID, P, D>) -> Self {
        core::result::Result::Err(value)
    }
}
