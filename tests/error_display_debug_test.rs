#[path = "setup_test.rs"]
mod setup_test;
use g_err::{GErr, GErrSource, gerr};
use setup_test::*;

#[path = "error_display_debug_test_data.rs"]
mod error_display_debug_test_data;

#[test]
fn test_gerr_debug() {
    use error_display_debug_test_data::test_gerr_debug_data;
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<ErrIDStrAutoCode, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(
            gerr!("source 2"; config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        )
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    let debug = format!("{:?}", gerr);
    assert_eq!(&debug, test_gerr_debug_data::EXPECTED_DEBUG);
    let debug_fmt = format!("{:#?}", gerr);
    assert_eq!(&debug_fmt, test_gerr_debug_data::EXPECTED_DEBUG_FORMAT);
}

#[test]
fn test_gerr_display() {
    const EXPECTED_DISPLAY: &str = "[AJO-123][AutoCode] asd";
    const EXPECTED_DISPLAY_WITHOUT_CODE: &str = "[AJO-123][-] zxc";
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<ErrIDStrAutoCode, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(
            gerr!("source 2"; config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        )
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });
    let display = format!("{}", gerr);
    assert_eq!(&display, EXPECTED_DISPLAY);
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<ErrIDStr, Data> = GErr::new_with_id("AJO", "zxc")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(
            gerr!("source 2"; config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        )
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });
    let display = format!("{}", gerr);
    assert_eq!(&display, EXPECTED_DISPLAY_WITHOUT_CODE);
}

#[test]
fn test_gerr_source_debug() {
    use error_display_debug_test_data::test_gerr_source_debug_data;
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr = GErr::<ErrIDStrAutoCode, Data>::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(
            gerr!("source 2";config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        )
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    let debug = format!("{:?}", gerr);
    assert_eq!(&debug, test_gerr_source_debug_data::EXPECTED_DEBUG);
    let debug_fmt = format!("{:#?}", gerr);
    assert_eq!(
        &debug_fmt,
        test_gerr_source_debug_data::EXPECTED_DEBUG_FORMAT
    );
}

#[test]
fn test_gerr_source_display() {
    const EXPECTED_DISPLAY: &str = "asd";
    const EXPECTED_DISPLAY_WITHOUT_CODE: &str = "zxc";
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErrSource = GErr::<ErrIDStrAutoCode, Data>::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(
            gerr!("source 2";config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        )
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    let display = format!("{}", gerr);
    assert_eq!(&display, EXPECTED_DISPLAY);

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErrSource = GErr::<ErrIDStr, Data>::new_with_id("AJO", "zxc")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(
            gerr!("source 2"; config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        )
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    let display = format!("{}", gerr);
    assert_eq!(&display, EXPECTED_DISPLAY_WITHOUT_CODE);
}

use core::error::Error;
use std::num::{IntErrorKind, ParseIntError};

#[test]
fn test_gerr_error_impl() {
    let gerr: GErr<ErrIDStrAutoCode, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_code("user")
        .set_help("please halp!!")
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    assert!(gerr.source().is_none());

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<ErrIDStrAutoCode, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_code("-user")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(
            gerr!("source 2"; config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        )
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    let error_source = gerr.source().unwrap();
    let source = error_source.downcast_ref::<ParseIntError>().unwrap();
    assert_eq!(source.kind(), &IntErrorKind::InvalidDigit);

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<ErrIDStrAutoCode, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_code("user")
        .set_help("please halp!!")
        .add_source_gerr(
            gerr!("source 2"; config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        )
        .add_source(err)
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        });

    let error_source = gerr.source().unwrap();
    let source = error_source.downcast_ref::<GErrSource>().unwrap();
    assert_eq!(source.id.as_ref().unwrap().to_string(), "123");
    assert_eq!(source.code.as_ref().unwrap(), "SOURCE-2");
    assert!(source.tags.as_ref().unwrap().iter().eq(["qwe", "wex"]));
}

#[test]
fn test_gerr_source_error_impl() {
    let gerr = GErr::<ErrIDStrAutoCode, Data>::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_code("user")
        .set_help("please halp!!")
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    assert!(gerr.source().is_none());

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr = GErr::<ErrIDStrAutoCode, Data>::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_code("user")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(
            gerr!("source 2"; config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        )
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    let error_source = gerr.source().unwrap();
    let source = error_source.downcast_ref::<ParseIntError>().unwrap();
    assert_eq!(source.kind(), &IntErrorKind::InvalidDigit);

    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr = GErr::<ErrIDStrAutoCode, Data>::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_code("USER")
        .set_help("please halp!!")
        .add_source_gerr(
            gerr!("source 2"; config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        )
        .add_source(err)
        .set_data(Data {
            user_id: 234,
            user_name: "ajo_sidi".into(),
        })
        .into_gerr_source();

    let error_source = gerr.source().unwrap();
    let source = error_source.downcast_ref::<GErrSource>().unwrap();
    assert_eq!(source.id.as_ref().unwrap().to_string(), "123");
    assert_eq!(source.code.as_ref().unwrap(), "SOURCE-2");
    assert!(source.tags.as_ref().unwrap().iter().eq(["qwe", "wex"]));
}
