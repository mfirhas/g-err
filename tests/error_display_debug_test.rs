#[path = "setup_test.rs"]
mod setup_test;

use setup_test::*;

use g_err::{GErr, GErrSource, NoPrefix, gerr};

#[test]
fn test_gerr_debug() {
    #[cfg(not(feature = "serde"))]
    const EXPECTED_DEBUG: &str = r#"GErr { id: "AJO-123", prefix: Some("AutoPrefix-user"), message: "asd", sources: Some([Err(ParseIntError { kind: InvalidDigit }), GErr(GErrSource { id: 123, prefix: Some("SOURCE-2"), message: "source 2", sources: None, tags: Some(["qwe", "wex"]), data: None, help: None, location: Some(Location { file: "tests/error_display_debug_test.rs", line: 149, column: 26 }) })]), tags: Some(["tag1", "tag2"]), data: Some(Data { user_id: 234, user_name: "ajo_sidi" }), help: Some("please halp!!"), location: Location { file: "tests/error_display_debug_test.rs", line: 142, column: 46 } }"#;
    #[cfg(feature = "serde")]
    const EXPECTED_DEBUG: &str = r#"GErr { id: "AJO-123", prefix: Some("AutoPrefix-user"), message: "asd", sources: Some([Err(ParseIntError { kind: InvalidDigit }), GErr(GErrSource { id: 123, id_json: Number(123), prefix: Some("SOURCE-2"), message: "source 2", sources: None, tags: Some(["qwe", "wex"]), data: None, data_json: None, help: None, location: Some(Location { file: "tests/error_display_debug_test.rs", line: 149, column: 26 }) })]), tags: Some(["tag1", "tag2"]), data: Some(Data { user_id: 234, user_name: "ajo_sidi" }), help: Some("please halp!!"), location: Location { file: "tests/error_display_debug_test.rs", line: 142, column: 46 }, backtrace: <disabled> }"#;
    #[cfg(not(feature = "serde"))]
    const EXPECTED_DEBUG_FORMAT: &str = r#"GErr {
    id: "AJO-123",
    prefix: Some(
        "AutoPrefix-user",
    ),
    message: "asd",
    sources: Some(
        [
            Err(
                ParseIntError {
                    kind: InvalidDigit,
                },
            ),
            GErr(
                GErrSource {
                    id: 123,
                    prefix: Some(
                        "SOURCE-2",
                    ),
                    message: "source 2",
                    sources: None,
                    tags: Some(
                        [
                            "qwe",
                            "wex",
                        ],
                    ),
                    data: None,
                    help: None,
                    location: Some(
                        Location {
                            file: "tests/error_display_debug_test.rs",
                            line: 149,
                            column: 26,
                        },
                    ),
                },
            ),
        ],
    ),
    tags: Some(
        [
            "tag1",
            "tag2",
        ],
    ),
    data: Some(
        Data {
            user_id: 234,
            user_name: "ajo_sidi",
        },
    ),
    help: Some(
        "please halp!!",
    ),
    location: Location {
        file: "tests/error_display_debug_test.rs",
        line: 142,
        column: 46,
    },
}"#;
    #[cfg(feature = "serde")]
    const EXPECTED_DEBUG_FORMAT: &str = r#"GErr {
    id: "AJO-123",
    prefix: Some(
        "AutoPrefix-user",
    ),
    message: "asd",
    sources: Some(
        [
            Err(
                ParseIntError {
                    kind: InvalidDigit,
                },
            ),
            GErr(
                GErrSource {
                    id: 123,
                    id_json: Number(123),
                    prefix: Some(
                        "SOURCE-2",
                    ),
                    message: "source 2",
                    sources: None,
                    tags: Some(
                        [
                            "qwe",
                            "wex",
                        ],
                    ),
                    data: None,
                    data_json: None,
                    help: None,
                    location: Some(
                        Location {
                            file: "tests/error_display_debug_test.rs",
                            line: 149,
                            column: 26,
                        },
                    ),
                },
            ),
        ],
    ),
    tags: Some(
        [
            "tag1",
            "tag2",
        ],
    ),
    data: Some(
        Data {
            user_id: 234,
            user_name: "ajo_sidi",
        },
    ),
    help: Some(
        "please halp!!",
    ),
    location: Location {
        file: "tests/error_display_debug_test.rs",
        line: 142,
        column: 46,
    },
    backtrace: <disabled>,
}"#;
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<&str, AutoPrefix, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .append_prefix("-user")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(gerr!("source 2"; id = 123, prefix="SOURCE-2", tags=["qwe","wex"]))
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    let debug = format!("{:?}", gerr);
    assert_eq!(&debug, EXPECTED_DEBUG);
    let debug_fmt = format!("{:#?}", gerr);
    assert_eq!(&debug_fmt, EXPECTED_DEBUG_FORMAT);
}

#[test]
fn test_gerr_display() {
    const EXPECTED_DISPLAY: &str = "AutoPrefix-user asd";
    const EXPECTED_DISPLAY_WITHOUT_PREFIX: &str = "zxc";
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<&str, AutoPrefix, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .append_prefix("-user")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(gerr!("source 2"; id = 123, prefix="SOURCE-2", tags=["qwe","wex"]))
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    let display = format!("{}", gerr);
    assert_eq!(&display, EXPECTED_DISPLAY);

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<&str, NoPrefix, Data> = GErr::new_with_id("AJO", "zxc")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(gerr!("source 2"; id = 123, prefix="SOURCE-2", tags=["qwe","wex"]))
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    let display = format!("{}", gerr);
    assert_eq!(&display, EXPECTED_DISPLAY_WITHOUT_PREFIX);
}

#[test]
fn test_gerr_source_debug() {
    #[cfg(not(feature = "serde"))]
    const EXPECTED_DEBUG: &str = r#"GErrSource { id: "AJO-123", prefix: Some("AutoPrefix-user"), message: "asd", sources: Some([Err(ParseIntError { kind: InvalidDigit }), GErr(GErrSource { id: 123, prefix: Some("SOURCE-2"), message: "source 2", sources: None, tags: Some(["qwe", "wex"]), data: None, help: None, location: Some(Location { file: "tests/error_display_debug_test.rs", line: 351, column: 26 }) })]), tags: Some(["tag1", "tag2"]), data: Some(Data { user_id: 234, user_name: "ajo_sidi" }), help: Some("please halp!!"), location: Some(Location { file: "tests/error_display_debug_test.rs", line: 344, column: 16 }) }"#;
    #[cfg(feature = "serde")]
    const EXPECTED_DEBUG: &str = r#"GErrSource { id: "AJO-123", id_json: String("AJO-123"), prefix: Some("AutoPrefix-user"), message: "asd", sources: Some([Err(ParseIntError { kind: InvalidDigit }), GErr(GErrSource { id: 123, id_json: Number(123), prefix: Some("SOURCE-2"), message: "source 2", sources: None, tags: Some(["qwe", "wex"]), data: None, data_json: None, help: None, location: Some(Location { file: "tests/error_display_debug_test.rs", line: 351, column: 26 }) })]), tags: Some(["tag1", "tag2"]), data: Some(Data { user_id: 234, user_name: "ajo_sidi" }), data_json: Some(Object {"user_id": Number(234), "user_name": String("ajo_sidi")}), help: Some("please halp!!"), location: Some(Location { file: "tests/error_display_debug_test.rs", line: 344, column: 16 }) }"#;

    #[cfg(not(feature = "serde"))]
    const EXPECTED_DEBUG_FORMAT: &str = r#"GErrSource {
    id: "AJO-123",
    prefix: Some(
        "AutoPrefix-user",
    ),
    message: "asd",
    sources: Some(
        [
            Err(
                ParseIntError {
                    kind: InvalidDigit,
                },
            ),
            GErr(
                GErrSource {
                    id: 123,
                    prefix: Some(
                        "SOURCE-2",
                    ),
                    message: "source 2",
                    sources: None,
                    tags: Some(
                        [
                            "qwe",
                            "wex",
                        ],
                    ),
                    data: None,
                    help: None,
                    location: Some(
                        Location {
                            file: "tests/error_display_debug_test.rs",
                            line: 351,
                            column: 26,
                        },
                    ),
                },
            ),
        ],
    ),
    tags: Some(
        [
            "tag1",
            "tag2",
        ],
    ),
    data: Some(
        Data {
            user_id: 234,
            user_name: "ajo_sidi",
        },
    ),
    help: Some(
        "please halp!!",
    ),
    location: Some(
        Location {
            file: "tests/error_display_debug_test.rs",
            line: 344,
            column: 16,
        },
    ),
}"#;
    #[cfg(feature = "serde")]
    const EXPECTED_DEBUG_FORMAT: &str = r#"GErrSource {
    id: "AJO-123",
    id_json: String("AJO-123"),
    prefix: Some(
        "AutoPrefix-user",
    ),
    message: "asd",
    sources: Some(
        [
            Err(
                ParseIntError {
                    kind: InvalidDigit,
                },
            ),
            GErr(
                GErrSource {
                    id: 123,
                    id_json: Number(123),
                    prefix: Some(
                        "SOURCE-2",
                    ),
                    message: "source 2",
                    sources: None,
                    tags: Some(
                        [
                            "qwe",
                            "wex",
                        ],
                    ),
                    data: None,
                    data_json: None,
                    help: None,
                    location: Some(
                        Location {
                            file: "tests/error_display_debug_test.rs",
                            line: 351,
                            column: 26,
                        },
                    ),
                },
            ),
        ],
    ),
    tags: Some(
        [
            "tag1",
            "tag2",
        ],
    ),
    data: Some(
        Data {
            user_id: 234,
            user_name: "ajo_sidi",
        },
    ),
    data_json: Some(
        Object {
            "user_id": Number(234),
            "user_name": String("ajo_sidi"),
        },
    ),
    help: Some(
        "please halp!!",
    ),
    location: Some(
        Location {
            file: "tests/error_display_debug_test.rs",
            line: 344,
            column: 16,
        },
    ),
}"#;
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr = GErr::<&str, AutoPrefix, Data>::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .append_prefix("-user")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(gerr!("source 2"; id = 123, prefix="SOURCE-2", tags=["qwe","wex"]))
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    let debug = format!("{:?}", gerr);
    assert_eq!(&debug, EXPECTED_DEBUG);
    let debug_fmt = format!("{:#?}", gerr);
    assert_eq!(&debug_fmt, EXPECTED_DEBUG_FORMAT);
}

#[test]
fn test_gerr_source_display() {
    const EXPECTED_DISPLAY: &str = "AutoPrefix-user asd";
    const EXPECTED_DISPLAY_WITHOUT_PREFIX: &str = "zxc";
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErrSource = GErr::<&str, AutoPrefix, Data>::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .append_prefix("-user")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(gerr!("source 2"; id = 123, prefix="SOURCE-2", tags=["qwe","wex"]))
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    let display = format!("{}", gerr);
    assert_eq!(&display, EXPECTED_DISPLAY);

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<&str, NoPrefix, Data> = GErr::new_with_id("AJO", "zxc")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(gerr!("source 2"; id = 123, prefix="SOURCE-2", tags=["qwe","wex"]))
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    let display = format!("{}", gerr);
    assert_eq!(&display, EXPECTED_DISPLAY_WITHOUT_PREFIX);
}

use core::error::Error;
use std::num::{IntErrorKind, ParseIntError};

#[test]
fn test_gerr_error_impl() {
    let gerr: GErr<&str, AutoPrefix, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .append_prefix("-user")
        .set_help("please halp!!")
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    assert!(gerr.source().is_none());

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<&str, AutoPrefix, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .append_prefix("-user")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(gerr!("source 2"; id = 123, prefix="SOURCE-2", tags=["qwe","wex"]))
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    let error_source = gerr.source().unwrap();
    let source = error_source.downcast_ref::<ParseIntError>().unwrap();
    assert_eq!(source.kind(), &IntErrorKind::InvalidDigit);

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<&str, AutoPrefix, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .append_prefix("-user")
        .set_help("please halp!!")
        .add_source_gerr(gerr!("source 2"; id = 123, prefix="SOURCE-2", tags=["qwe","wex"]))
        .add_source(err)
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    let error_source = gerr.source().unwrap();
    let source = error_source.downcast_ref::<GErrSource>().unwrap();
    assert_eq!(source.id.to_string(), "123");
    assert_eq!(source.prefix.as_ref().unwrap(), "SOURCE-2");
    assert!(source.tags.as_ref().unwrap().iter().eq(["qwe", "wex"]));
}

#[test]
fn test_gerr_source_error_impl() {
    let gerr = GErr::<&str, AutoPrefix, Data>::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .append_prefix("-user")
        .set_help("please halp!!")
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    assert!(gerr.source().is_none());

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr = GErr::<&str, AutoPrefix, Data>::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .append_prefix("-user")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(gerr!("source 2"; id = 123, prefix="SOURCE-2", tags=["qwe","wex"]))
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    let error_source = gerr.source().unwrap();
    let source = error_source.downcast_ref::<ParseIntError>().unwrap();
    assert_eq!(source.kind(), &IntErrorKind::InvalidDigit);

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr = GErr::<&str, AutoPrefix, Data>::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .append_prefix("-user")
        .set_help("please halp!!")
        .add_source_gerr(gerr!("source 2"; id = 123, prefix="SOURCE-2", tags=["qwe","wex"]))
        .add_source(err)
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    let error_source = gerr.source().unwrap();
    let source = error_source.downcast_ref::<GErrSource>().unwrap();
    assert_eq!(source.id.to_string(), "123");
    assert_eq!(source.prefix.as_ref().unwrap(), "SOURCE-2");
    assert!(source.tags.as_ref().unwrap().iter().eq(["qwe", "wex"]));
}
