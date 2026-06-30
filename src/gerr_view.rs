extern crate alloc;

use core::panic::Location;

use alloc::borrow::Cow;

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

use crate::{
    GErrSource,
    gerr::{GErr, Prefix},
};

/// GErrView - GErr in borrowed form for reporting.
pub struct GErrView<'a, ID, D> {
    /// Error id.
    pub id: &'a ID,
    /// Error message.
    pub message: &'a str,
    /// Error prefix.
    pub prefix: Option<&'a str>,
    /// Error data.
    pub data: Option<&'a D>,
    /// Error tags.
    pub tags: Option<&'a [Cow<'static, str>]>,
    /// Error sources.
    pub sources: Option<&'a [GErrSource]>,
    /// Error help hint.
    pub help: Option<&'a str>,
    /// Error location.
    pub location: &'a Location<'static>,
    /// Error backtrace.
    #[cfg(feature = "backtrace")]
    pub backtrace: &'a Backtrace,
}

impl<'a, ID, P: Prefix, D> From<&'a GErr<ID, P, D>> for GErrView<'a, ID, D> {
    fn from(err: &'a GErr<ID, P, D>) -> Self {
        Self {
            id: err.id(),
            message: err.message(),
            prefix: err.prefix(),

            data: err.data(),

            tags: err.tags(),

            sources: err.sources(),

            help: err.help(),

            location: err.location(),

            #[cfg(feature = "backtrace")]
            backtrace: err.backtrace(),
        }
    }
}
