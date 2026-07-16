# `g-err`

[![MSRV](https://img.shields.io/crates/msrv/g-err)](https://github.com/mfirhas/g-err/actions/workflows/msrv.yml)
[![Crates.io](https://img.shields.io/crates/v/g-err.svg)](https://crates.io/crates/g-err)
[![ci](https://github.com/mfirhas/g-err/actions/workflows/ci.yml/badge.svg)](https://github.com/mfirhas/g-err/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/g-err/badge.svg)](https://docs.rs/g-err)
[![codecov](https://codecov.io/gh/mfirhas/g-err/branch/master/graph/badge.svg)](https://codecov.io/gh/mfirhas/g-err)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/mfirhas/g-err/blob/master/LICENSE)

`g-err` — Generically configurable structured error type.

## Overview
g-err provides a structured error type designed to make errors more informative and easier to work with.
It supports configurable and rich metadata such as id, code, error data and more, with compile-time customization through generics and optional features for reporting and serialization.
This gives an error type which is good for libraries and applications.

## Features
- Rich metadata:
  - Error id: for identifying the error. Can be set manually or auto-generated.
  - Error code: for labeling/grouping the error. Can be set manually or made constant accross multiple error instances.
  - Message: the error message.
  - Error sources/causes: each error might be caused by multiple errors, serially or parallel. GErr composes them all in form of tree and can be queried by metadata. 
  - Error tags: tagging error adding better visibility for monitoring.
  - Error data: additional data might be needed for error, such as error kinds, user data, or any data that's useful to add.
  - Help hint: information to help to solve the error.
  - Location: Where the error happen in source code.
  - Backtrace: When feature `backtrace` enabled, this will add stacktrace into error.
- Iterator for error sources/causes which can be queried by its metadata. The error sources/causes are traversed by DFS.
  Instead of just boxing error in fat pointer `dyn` and check only for type, you can query GErr with more metadata.
- Error reporting: you can report error in many form(pretty format, markdown, trace, or json), or you can create your own report format by implementing [`Report`] trait.
- Result extension: work and chain the errors with Result type.
- JSON serde: for cross-boundaries communication, error can be serde'd in form of JSON.
- `no_std` (`alloc` is included in `no_std` setting).

## Why?
When you deal with error, usually we ended up with question what kind of error which we'll be using. It can be as simple as `&'static str'`, but can't be formatted. Using `String` helps, but what if you need literal string?.
Some best practices create error enum listing multiple possibilities of error kinds. There're 2 problems with this enum approach, either you end up with massive enum for all errors, or you have to create many enums for each errors.You have to implement standard `Error` trait for them to make your error type interop well. Another problem with these approaches you end up creating many possible error types. You could box them into fat pointer dyn, but it's too general and remove the type. In the end you carry too little informations about the error.

This crate solves all of these issues in one concrete generically configurable error type, good for just an error message, boxed, and with rich metadata. It's informative for monitoring. 

## Error ID
Identity for error. Each error can bring id. The id can be set manually, for things such as request id, correlation id, or any id be it manual or auto-generated. It's good for monitoring.
Each id must implement `Debug` & `Display`. If `serde` feature enabled, must implement `serde::Serialize` and `serde::Deserialize` as well.

## Error code
Group multiple error in the same category. Error code bring together multiple error in the same domain/label/category like http layer, domain layer, or repository layer, or any grouping.

## Error sources
Each error can be built up from multiple causes. These causes/sources can be chained in serial, or in parallel if caused by many at once. This is good for chaining up errors from multiple layers and from multiple sources,
be it from in-process, or inter-process/boundaries.

## Error tags
Tag the error with relevant keyword for query and monitoring.

## Error data
Each error might bring different kind of additional data with it, such as error kinds, user data, additional metrics, etc. These data can be defined generically for each error type.
Each error data type must implement `Debug`. If `serde` feature enabled, must implement `serde::Serialize` and `serde::Deserialize` as well.

## Error help
Show hint about how to solve the error.

## Error location
Show where the error happen, always auto-generated, or set during deserialization.

## Error backtrace
Show error backtrace if `backtrace` feature enabled.

## Result extensions
There are 2 traits for extending Result: [`ResultExt`] for non-gerr errors and [`GResultExt`] for gerr errors.
If a function return general/non-gerr errors, e.g. ParseIntError, use former one. If the error is GErr, or any error convertible to [`GErrSource`], use the latter.
It's good for chaining up with results for multiple layers.

## Serde
There are 3 kinds of serde supported:
- Default serialization only to string.
- Serialization only with JSON format using `#[serde(serialize_with = "g_err::serde::display_json::serialize")]`
- Both serialization and deserialization with JSON format using `#[serde(with = "g_err::serde::json")]`

## Example
```rust
#[cfg(feature = "serde")]
use g_err::json::*;

use g_err::*;
use std::num::ParseIntError;

// Error config for manual u64 error id
struct U64;
impl Config for U64 {
    type Id = u64;
}

// Error with id auto-generated as uuid v4 with error code.
struct AutoUUID;
impl Config for AutoUUID {
    const CODE: Option<&'static str> = Some("E-123");

    type Id = uuid::Uuid;
    #[inline]
    fn id() -> Option<Self::Id> {
        Some(uuid::Uuid::new_v4())
    }
}

let gerr = GErr::<U64>::new_with_id(409, "An error with manually set error id");
assert_eq!(
    gerr.to_string(),
    "[409][-] An error with manually set error id"
);

let gerr: GErr<AutoUUID> = GErr::new("An error with auto-generated error id");
println!("{gerr}"); // prints [<some uuid>][E-123] An error with auto-generated error id

// override display
struct MyError;
impl Config for MyError {
    const CODE: Option<&'static str> = Some("HTTP");
    type Id = &'static str;
    fn display<C: g_err::Config, D>(gerr: &g_err::GErr<C, D>) -> String
    where
        C::Id: std::fmt::Display,
        D: std::fmt::Debug,
    {
        if let Some(id) = gerr.id() {
            format!("[MyError][{}] {}", id, gerr.message())
        } else {
            format!("[MyError][-] {}", gerr.message())
        }
    }
}

let gerr2 = GErr::<MyError>::new_with_id(
    "USER-lk12m3l1k23",
    "error with manual user id as id and custom display",
)
.add_tags(["tag1", "tag2"])
.add_source("qwe".parse::<i32>().unwrap_err())
.add_source_gerr(gerr)
.set_help("Check your input");
println!("{}", gerr2);
assert_eq!(
    gerr2.to_string(),
    "[MyError][USER-lk12m3l1k23] error with manual user id as id and custom display"
);

// build with macro
let gerr3 = gerr!("error built with macro: {}", 123;
    config=MyError,
    tags=["tag1", "tag2"],
    source="qwe".parse::<i32>().unwrap_err(),
    gerr=gerr!("gerr source"; gerr=gerr!("deep error"; tag="deep")), help="Please pass valid input");

// report the error in
// pretty format:
let pretty_report = gerr3.report_as::<PrettyReport>();
println!("{}", pretty_report);

// markdown format
let markdown_report = gerr3.report_as::<MarkdownReport>();
println!("{}", markdown_report);

// trace
let trace_report = gerr3.report_as::<TraceReport>();
println!("{}", trace_report);

// json format, if `serde` feature enabled
#[cfg(feature = "serde")]
{
    let json_report = gerr3.report_as::<JsonReport>();
    println!("{}", json_report);
}

// or you can customize your own report format by creating a type implementing `Report` trait.
struct MyReport;
impl Report for MyReport {
    fn report<E, C: g_err::Config, D>(_err: &E) -> String
    where
        for<'a> &'a E: Into<g_err::GErrView<'a, C, D>>,
        C::Id: std::fmt::Display,
        D: std::fmt::Debug,
    {
        // todo!("your own report formatting here...")
        "my format".into()
    }
}
let my_report = gerr3.report_as::<MyReport>();
println!("{my_report}");

// iterating and querying the error
let ret = gerr3.iter_by_tag("tag2").next();
assert!(ret.is_some());

let ret = gerr3.iter_by_tag("deep").next();
assert!(ret.is_some());

let ret = gerr3.iter_source::<GErrSource>().next(); // if GErr becomes source error
assert!(ret.is_some());

let ret = gerr3.iter_source::<ParseIntError>().next();
assert!(ret.is_some());

#[cfg(feature = "serde")]
{
    let json_data = gerr3.json_data();
    println!("{:#?}", json_data); // json data of GErr.
}
```

## Box
If you think the concrete type is too big for you, box it, or use [`GErrBox`] for boxed GErr.

## Feature flags
- `default`: std enabled. Disable it for `no_std`.
- `serde`: for json and serde.
- `backtrace`: for enabling backtracing.
