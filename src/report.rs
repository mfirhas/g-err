extern crate alloc;

use alloc::string::String;
use core::fmt::{Debug, Display};

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

use crate::{Config, gerr::GErr, gerr_view::GErrView};

/// Reporting trait.
///
/// Reports error in multiple forms of display.
pub trait Report {
    /// Reports error from error borrowed form [`crate::GErrView`] into String.
    #[cfg(not(feature = "serde"))]
    fn report<E, C: Config, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, C, D>>,
        C::Id: Display,
        D: Debug;

    /// Reports error from error borrowed form [`crate::GErrView`] into String.
    ///
    /// Supports JSON.
    #[cfg(feature = "serde")]
    fn report<E, C: Config, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, C, D>>,
        C::Id: Display + serde::Serialize,
        D: Debug + serde::Serialize;
}

#[cfg(not(feature = "serde"))]
impl<C: Config, D> GErr<C, D>
where
    C::Id: Display,
    D: Debug,
{
    /// Reports error in pretty format.
    pub fn report(&self) -> String {
        PrettyReport::report::<_, C, D>(self)
    }

    /// Reports error as specified format.
    pub fn report_as<R>(&self) -> String
    where
        R: Report,
    {
        R::report::<_, C, D>(self)
    }
}

#[cfg(feature = "serde")]
impl<C: Config, D> GErr<C, D>
where
    C::Id: Display + serde::Serialize,
    D: Debug + serde::Serialize,
{
    /// Reports error in pretty format.
    #[inline]
    pub fn report(&self) -> String {
        PrettyReport::report::<_, C, D>(self)
    }

    /// Reports error as specified format.
    #[inline]
    pub fn report_as<R>(&self) -> String
    where
        R: Report,
    {
        R::report::<_, C, D>(self)
    }
}

#[cfg(feature = "serde")]
impl<C: Config, D> GErr<C, D>
where
    C::Id: serde::Serialize,
    D: serde::Serialize,
{
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
