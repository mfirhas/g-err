#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

use g_err::{GErr, GErrBox, GErrSource, GErrView, NoData, gerr};

#[test]
fn test_auto_into_gerr_source() {
    let gerr: GErr<ErrAutoIDCode, NoData> = GErr::new("nganu")
        .add_tag("tag1")
        .set_code("user")
        .set_help("please halp!!");

    let gerr_source: GErrSource = gerr.into();
    assert_eq!(gerr_source.id.as_ref().unwrap().to_string(), "AutoID");
    assert_eq!(gerr_source.message, "nganu");
    assert_eq!(gerr_source.tags.unwrap()[0], "tag1");
    assert_eq!(gerr_source.code.unwrap(), "user");
    assert_eq!(gerr_source.help.unwrap(), "please halp!!");
}

#[test]
fn test_auto_into_gerr_view() {
    let gerr: GErr<ErrAutoIDCode, NoData> = GErr::new("nganu")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_code("user")
        .set_help("please halp!!");

    let gerr_source: GErrView<_, _> = (&gerr).into();
    assert_eq!(gerr_source.id.as_ref().unwrap().to_string(), "AutoID");
    assert_eq!(gerr_source.message, "nganu");
    assert_eq!(gerr_source.tags.unwrap()[0], "tag1");
    assert_eq!(gerr_source.tags.unwrap()[1], "tag2");
    assert_eq!(gerr_source.code.unwrap(), "user");
    assert_eq!(gerr_source.help.unwrap(), "please halp!!");
}

#[test]
fn test_manual_into_gerr_source() {
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<ErrIDStr, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_code("USER")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(
            gerr!("source 2"; config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        );

    let gerr_source: GErrSource = gerr.into();
    assert_eq!(gerr_source.id.as_ref().unwrap().to_string(), "AJO-123");
    assert_eq!(gerr_source.message, "asd");
    assert_eq!(gerr_source.tags.as_ref().unwrap()[0], "tag1");
    assert_eq!(gerr_source.tags.as_ref().unwrap()[1], "tag2");
    assert_eq!(gerr_source.code.unwrap(), "USER");
    assert_eq!(gerr_source.help.unwrap(), "please halp!!");
    assert_eq!(gerr_source.sources.as_ref().unwrap().len(), 2);
    match gerr_source.sources.as_ref().unwrap()[0] {
        g_err::Source::Err(ref err) => assert_eq!(err.to_string(), "invalid digit found in string"),
        g_err::Source::GErr(_) => panic!("halp!!"),
    }
    match gerr_source.sources.as_ref().unwrap()[1] {
        g_err::Source::Err(_) => panic!("halp!!"),
        g_err::Source::GErr(ref gerr) => {
            assert_eq!(gerr.message, "source 2");
            assert_eq!(gerr.id.as_ref().unwrap().to_string(), "123");
            assert_eq!(gerr.code.as_ref().unwrap(), "SOURCE-2");
            assert!(gerr.tags.as_ref().unwrap().iter().eq(["qwe", "wex"]));
        }
    }
}

#[test]
fn test_manual_into_gerr_view() {
    let err = "qwe".parse::<i32>().unwrap_err();
    let gerr: GErr<ErrIDStr, Data> = GErr::new_with_id("AJO", "asd")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_id("AJO-123")
        .set_help("please halp!!")
        .add_source(err)
        .add_source_gerr(
            gerr!("source 2"; config=ErrIDi32, id = 123, code="SOURCE-2", tags=["qwe","wex"]),
        );

    let gerr_source: GErrView<_, _> = (&gerr).into();
    assert_eq!(gerr_source.id.as_ref().unwrap().to_string(), "AJO-123");
    assert_eq!(gerr_source.message, "asd");
    assert_eq!(gerr_source.tags.unwrap()[0], "tag1");
    assert_eq!(gerr_source.tags.unwrap()[1], "tag2");
    assert_eq!(gerr_source.code.unwrap(), "-user");
    assert_eq!(gerr_source.help.unwrap(), "please halp!!");
    assert_eq!(gerr_source.sources.unwrap().len(), 2);
    match gerr_source.sources.unwrap()[0] {
        g_err::Source::Err(ref err) => assert_eq!(err.to_string(), "invalid digit found in string"),
        g_err::Source::GErr(_) => panic!("expected Source::Err"),
    }
    match gerr_source.sources.unwrap()[1] {
        g_err::Source::Err(_) => panic!("expected Source::GErr"),
        g_err::Source::GErr(ref gerr) => {
            assert_eq!(gerr.message, "source 2");
            assert_eq!(gerr.id.as_ref().unwrap().to_string(), "123");
            assert_eq!(gerr.code.as_ref().unwrap(), "SOURCE-2");
            assert!(gerr.tags.as_ref().unwrap().iter().eq(["qwe", "wex"]));
        }
    }
}

#[test]
fn test_box_into_gerr_source() {
    let gerr: GErrBox<ErrAutoIDCode, NoData> = GErr::new("nganu")
        .add_tag("tag1")
        .set_help("please halp!!")
        .boxed();

    let gerr_source: GErrSource = gerr.into();
    assert_eq!(gerr_source.id.as_ref().unwrap().to_string(), "AutoID");
    assert_eq!(gerr_source.message, "nganu");
    assert_eq!(gerr_source.tags.unwrap()[0], "tag1");
    assert_eq!(gerr_source.code.unwrap(), "AutoPrefix-user");
    assert_eq!(gerr_source.help.unwrap(), "please halp!!");
}

#[test]
fn test_box_into_gerr_view() {
    let gerr: Box<GErr<ErrAutoIDCode, NoData>> = GErr::new("nganu")
        .add_tag("tag1")
        .add_tag("tag2")
        .set_code("user")
        .set_help("please halp!!")
        .boxed();

    let gerr_source: GErrView<_, _> = (&gerr).into();
    assert_eq!(gerr_source.id.as_ref().unwrap().to_string(), "AutoID");
    assert_eq!(gerr_source.message, "nganu");
    assert_eq!(gerr_source.tags.unwrap()[0], "tag1");
    assert_eq!(gerr_source.tags.unwrap()[1], "tag2");
    assert_eq!(gerr_source.code.unwrap(), "user");
    assert_eq!(gerr_source.help.unwrap(), "please halp!!");
}
