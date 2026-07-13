extern crate alloc;

use alloc::borrow::Cow;

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

use crate::gerr::{Config, ErrorLocation, GErr, Source};

/// GErrView - GErr in borrowed form for reporting.
pub struct GErrView<'a, C: Config, D> {
    /// Error id.
    pub id: Option<&'a C::Id>,
    /// Error message.
    pub message: &'a str,
    /// Error code.
    pub code: Option<&'a str>,
    /// Error data.
    pub data: Option<&'a D>,
    /// Error tags.
    pub tags: Option<&'a [Cow<'static, str>]>,
    /// Error sources.
    pub sources: Option<&'a [Source]>,
    /// Error help hint.
    pub help: Option<&'a str>,
    /// Error location.
    pub location: &'a ErrorLocation,
    /// Error backtrace.
    #[cfg(feature = "backtrace")]
    pub backtrace: &'a Backtrace,
}

impl<'a, C: Config, D> From<&'a GErr<C, D>> for GErrView<'a, C, D> {
    fn from(err: &'a GErr<C, D>) -> Self {
        Self {
            id: err.id(),
            message: err.message(),
            code: err.code(),

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
