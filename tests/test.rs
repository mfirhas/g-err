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

#[path = "code_test.rs"]
mod code_test;

#[path = "query_test.rs"]
mod query_test;

#[path = "iterator_test.rs"]
mod iterator_test;

#[path = "report_test.rs"]
mod report_test;

#[cfg(feature = "serde")]
#[path = "json_report_test.rs"]
mod json_report_test;

#[cfg(feature = "serde")]
#[path = "json_serde_test.rs"]
mod json_serde_test;

#[path = "sources_test.rs"]
mod sources_test;

#[path = "tags_test.rs"]
mod tags_test;

#[path = "result_test.rs"]
mod result_test;

#[path = "size_test.rs"]
mod size_test;

#[cfg(feature = "serde")]
#[test]
fn test_example() {
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
}
