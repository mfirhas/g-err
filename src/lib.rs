#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

extern crate alloc;

mod gerr;
pub use gerr::{BoxError, GErr, Id, Prefix, Result, SetField, Source};

mod gerr_source;
pub use gerr_source::{DataSource, GErrSource, IdSource};

mod gerr_view;
pub use gerr_view::GErrView;

mod report;
pub use report::{MarkdownReport, PrettyReport, TraceReport};

#[cfg(feature = "serde")]
pub use report::{
    DisplayJsonData, DisplayJsonReport, JsonData, JsonReport, LocationJsonData, SourceJsonData,
};

#[cfg(feature = "serde")]
pub mod serde;

mod macros;

mod types;
pub use types::{NoData, NoID, NoPrefix};

mod result_ext;
pub use result_ext::{GResultExt, ResultExt};

mod iterator;
pub use iterator::{Iter, IterItem};
mod query;

mod sealed {
    pub trait Sealed {}
}
