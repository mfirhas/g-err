extern crate alloc;

use alloc::string::String;
use core::fmt::{Debug, Display};

#[cfg(feature = "std")]
extern crate std;

mod pretty_report;
pub use pretty_report::PrettyReport;

mod markdown_report;
pub use markdown_report::MarkdownReport;

mod trace_report;
pub use trace_report::TraceReport;

#[cfg(feature = "serde")]
mod json_report;

#[cfg(feature = "serde")]
mod json_data;

#[cfg(feature = "serde")]
pub use json_data::{DisplayJsonData, JsonData, LocationJsonData, SourceJsonData};

#[cfg(feature = "serde")]
pub use json_report::{DisplayJsonReport, JsonReport};

use crate::{
    gerr::{GErr, Prefix},
    gerr_view::GErrView,
};

pub trait Report {
    #[cfg(not(feature = "serde"))]
    fn report<E, ID, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: Display,
        D: Debug;

    #[cfg(feature = "serde")]
    fn report<E, ID, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
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
        PrettyReport::report::<_, ID, D>(self)
    }

    pub fn report_as<R>(&self) -> String
    where
        R: Report,
    {
        R::report::<_, ID, D>(self)
    }
}

#[cfg(feature = "serde")]
impl<ID, P, D> GErr<ID, P, D>
where
    ID: Display + serde::Serialize,
    P: Prefix,
    D: Debug + serde::Serialize,
{
    #[inline]
    pub fn report(&self) -> String {
        PrettyReport::report::<_, ID, D>(self)
    }

    #[inline]
    pub fn report_as<R>(&self) -> String
    where
        R: Report,
    {
        R::report::<_, ID, D>(self)
    }

    #[inline]
    pub fn json_data(&self) -> JsonData {
        JsonReport::data(self)
    }

    #[inline]
    pub fn display_json_data(&self) -> DisplayJsonData {
        DisplayJsonReport::data(self)
    }
}
