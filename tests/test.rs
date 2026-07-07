#[path = "macro_test.rs"]
mod macro_test;

#[path = "constructor_test.rs"]
mod constructor_test;

#[path = "builder_test.rs"]
mod builder_test;

#[path = "conversion_test.rs"]
mod conversion_test;

#[path = "error_display_debug_test.rs"]
mod error_display_debug_test;

#[path = "prefix_test.rs"]
mod prefix_test;

#[path = "query_test.rs"]
mod query_test;

#[path = "iterator_test.rs"]
mod iterator_test;

#[path = "report_test.rs"]
mod report_test;

#[cfg(feature = "serde")]
#[path = "json_report_test.rs"]
mod json_report_test;

#[path = "sources_test.rs"]
mod sources_test;

#[path = "tags_test.rs"]
mod tags_test;

#[path = "result_test.rs"]
mod result_test;
