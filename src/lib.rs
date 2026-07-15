#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::result_large_err)]

extern crate alloc;

mod gerr;
pub use gerr::{Config, GErr, GErrDefault, Result, SetField, Source};

mod gerr_source;
pub use gerr_source::{DataSource, GErrSource, IdSource};

mod gerr_view;
pub use gerr_view::GErrView;

mod gerr_box;
pub use gerr_box::GErrBox;

mod report;
pub use report::{MarkdownReport, PrettyReport, TraceReport};

#[cfg(feature = "serde")]
pub mod json {
    //! Contains JSON reporting and data for [`GErr`](super::GErr).
    pub use super::report::{
        DisplayJsonData, JsonData, JsonReport, LocationJsonData, SourceJsonData,
    };

    pub use ::serde_json::Value;
}

#[cfg(feature = "serde")]
pub mod serde;

mod macros;

mod types;
pub use types::{DefaultConfig, NoData, NoID};

mod result_ext;
pub use result_ext::{GResultExt, ResultExt};

pub mod iterator;
mod query;

mod sealed {
    pub trait Sealed {}
}
