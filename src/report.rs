extern crate alloc;

use crate::Source;
use crate::{GErr, Prefix};
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt::{Debug, Display};
use core::panic::Location;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

mod pretty_report;
pub use pretty_report::PrettyReport;

mod markdown_report;
pub use markdown_report::MarkdownReport;

mod trace_report;
pub use trace_report::TraceReport;

#[cfg(feature = "serde")]
mod json_report;

#[cfg(feature = "serde")]
pub use json_report::{DisplayJsonReport, JsonReport};

pub struct GErrView<'a, ID, D> {
    pub id: &'a ID,
    pub message: &'a str,
    pub prefix: Option<&'static str>,
    pub data: Option<&'a D>,
    pub tags: Option<&'a [Cow<'static, str>]>,
    pub sources: Option<&'a [Source]>,
    pub location: &'a Location<'static>,
    #[cfg(feature = "backtrace")]
    pub backtrace: &'a Backtrace,
}

impl<'a, ID, P: Prefix, D> From<&'a GErr<ID, P, D>> for GErrView<'a, ID, D> {
    fn from(err: &'a GErr<ID, P, D>) -> Self {
        Self {
            id: &err.id,
            message: err.message.as_ref(),
            prefix: err.prefix(),

            data: err.data.as_ref(),

            tags: err.tags.as_deref(),

            sources: err.source.as_deref(),

            location: err.location,

            #[cfg(feature = "backtrace")]
            backtrace: &err.backtrace,
        }
    }
}

pub trait Report: crate::sealed::Sealed {
    #[cfg(not(feature = "serde"))]
    fn report<ID, D>(err: &GErrView<ID, D>) -> String
    where
        ID: Display,
        D: Debug;

    #[cfg(feature = "serde")]
    fn report<ID, D>(err: &GErrView<ID, D>) -> String
    where
        ID: Display + serde::Serialize,
        D: Debug + serde::Serialize;
}

#[cfg(not(feature = "serde"))]
impl<ID, P, D> GErr<ID, P, D>
where
    ID: Display,
    P: Prefix,
    D: Debug,
{
    pub fn report(&self) -> String {
        PrettyReport::report::<ID, D>(&self.into())
    }

    pub fn report_as<R>(&self) -> String
    where
        R: Report,
    {
        R::report::<ID, D>(&self.into())
    }
}

#[cfg(feature = "serde")]
impl<ID, P, D> GErr<ID, P, D>
where
    ID: Display + serde::Serialize,
    P: Prefix,
    D: Debug + serde::Serialize,
{
    pub fn report(&self) -> String {
        PrettyReport::report::<ID, D>(&self.into())
    }

    pub fn report_as<R>(&self) -> String
    where
        R: Report,
    {
        R::report::<ID, D>(&self.into())
    }
}
