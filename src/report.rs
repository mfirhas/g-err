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
pub use json_report::JsonReport;

use crate::{
    gerr::{GErr, Prefix},
    gerr_view::GErrView,
};

/// Reporting trait.
///
/// Reports error in multiple forms of display.
pub trait Report {
    /// Reports error from error borrowed form [`crate::GErrView`] into String.
    #[cfg(not(feature = "serde"))]
    fn report<E, ID, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: Display,
        D: Debug;

    /// Reports error from error borrowed form [`crate::GErrView`] into String.
    ///
    /// Supports JSON.
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
    /// Reports error in pretty format.
    pub fn report(&self) -> String {
        PrettyReport::report::<_, ID, D>(self)
    }

    /// Reports error as specified format.
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
    /// Reports error in pretty format.
    #[inline]
    pub fn report(&self) -> String {
        PrettyReport::report::<_, ID, D>(self)
    }

    /// Reports error as specified format.
    #[inline]
    pub fn report_as<R>(&self) -> String
    where
        R: Report,
    {
        R::report::<_, ID, D>(self)
    }

    /// JSON data of GErr.
    #[inline]
    pub fn json_data(&self) -> JsonData {
        JsonReport::data(self)
    }

    /// Public JSON data of GErr.
    #[inline]
    pub fn display_json_data(&self) -> DisplayJsonData {
        JsonReport::display_data(self)
    }
}
