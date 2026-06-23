extern crate alloc;

use core::panic::Location;

use alloc::borrow::Cow;

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

use crate::{
    GErrSource,
    gerr::{GErr, Prefix},
};

pub struct GErrView<'a, ID, D> {
    pub id: &'a ID,
    pub message: &'a str,
    pub prefix: Option<&'a str>,
    pub data: Option<&'a D>,
    pub tags: Option<&'a [Cow<'static, str>]>,
    pub sources: Option<&'a [GErrSource]>,
    pub help: Option<&'a str>,
    pub location: &'a Location<'static>,
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
            backtrace: &err.backtrace(),
        }
    }
}
