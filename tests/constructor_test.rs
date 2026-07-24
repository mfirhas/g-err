use std::num::ParseIntError;

use g_err::{GErr, MarkdownReport, NoID, PrettyReport, TraceReport};

#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

#[test]
fn test_new_auto_default() {
    let gerr: GErr = GErr::new("auto default");

    assert_eq!(gerr.message(), "auto default");
    assert!(gerr.id().is_none());
    assert!(gerr.code().is_none());
    assert!(gerr.data().is_none());
}

#[test]
fn test_new_manual_id() {
    let gerr: GErr = GErr::new_with_id(NoID, "no id").set_code("E123");
    assert_eq!(format!("{}", gerr.id().unwrap()), "NoID");
    assert_eq!(format!("{}", gerr), "[NoID][E123] no id");

    let gerr: GErr = GErr::new("no id")
        .set_code("E123")
        .add_source_gerr(GErr::<()>::new("gerr source"))
        .add_source_gerr(
            GErr::<()>::new("gerr source")
                .set_id(NoID)
                .add_source_gerr(GErr::<()>::new("gerr source").set_id(NoID)),
        );
    assert_eq!(format!("{}", gerr), "[-][E123] no id");
    let _ = gerr.report_as::<MarkdownReport>();
    let _ = gerr.report_as::<PrettyReport>();
    let gerr = GErr::<()>::new("asd");
    let _ = gerr.report_as::<TraceReport>();
    let gerr = GErr::<()>::new("asd").set_code("E34");
    let _ = gerr.report_as::<TraceReport>();

    let gerr: GErr<ErrIDi32> = GErr::new_with_id(123, "manual id");

    assert_eq!(gerr.message(), "manual id");
    assert_eq!(gerr.id().unwrap(), &123);
    assert!(gerr.code().is_none());
    assert!(gerr.data().is_none());
}

// test new with auto-generated id.
#[test]
fn test_new_auto() {
    // auto id, no code, no data
    let gerr = GErr::<ErrAutoID>::new("error");
    assert_eq!(gerr.message(), "error");
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert!(gerr.code().is_none());

    let gerr = GErr::<ErrAutoCode>::new("auto code");
    assert!(gerr.id().is_none());
    assert_eq!(gerr.code().unwrap(), "AutoCode");

    // auto id, auto code
    let gerr: GErr<ErrAutoIDCode> = GErr::new("id and code are auto");
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "AutoCode");
    assert_eq!(gerr.message(), "id and code are auto");

    // auto id, auto code, with data
    let gerr: GErr<ErrAutoIDCode, Data> = GErr::new("all").set_data(Data {
        user_id: 234,
        user_name: "xxx".into(),
    });
    assert_eq!(gerr.message(), "all");
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "AutoCode");
    assert_eq!(gerr.data().unwrap().user_id, 234);
    assert_eq!(gerr.data().unwrap().user_name, "xxx");

    let gerr: GErr<ErrAutoIDCode, Data> = GErr::new("all")
        .set_field("user_id", 400)
        .set_field("user_name", "yyy".to_string());
    assert_eq!(gerr.message(), "all");
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "AutoCode");
    assert_eq!(gerr.data().unwrap().user_id, 400);
    assert_eq!(gerr.data().unwrap().user_name, "yyy");
}

// test new with manually-set id.
#[test]
fn test_new_manual() {
    let gerr = GErr::<ErrIDi32>::new_with_id(23, "test");
    assert_eq!(gerr.id().unwrap(), &23);
    assert_eq!(gerr.message(), "test");
    assert!(gerr.code().is_none());

    let gerr = GErr::<ErrIDStr>::new_with_id("zxc", "test").set_code("code");
    assert_eq!(gerr.id().unwrap(), &"zxc");
    assert_eq!(gerr.message(), "test");
    assert_eq!(gerr.code().unwrap(), "code");
}

#[test]
fn test_from_non_gerr() {
    let err = "asd".parse::<i32>().unwrap_err();
    let gerr = GErr::<ErrAutoID>::from_error(err);
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert!(gerr.code().is_none());
    assert_eq!(gerr.message(), "invalid digit found in string");
}

#[test]
fn test_from_non_gerr_id() {
    let err = "asd".parse::<i32>().unwrap_err();
    let gerr: GErr<ErrIDi32AutoCode> = GErr::from_error_with_id(123, err);
    assert_eq!(gerr.id().unwrap(), &123);
    assert_eq!(gerr.code().unwrap(), "AutoCode");
    assert_eq!(gerr.message(), "invalid digit found in string");
}

#[test]
fn test_with_tags() {
    let gerr: GErr<ErrIDCodeTags> = GErr::new("with tags");
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "AutoCode");
    assert!(gerr.iter_tags().eq(["tag1", "tag2", "tag3"]));

    let gerr = gerr.add_tag("tag4");
    assert!(gerr.iter_tags().eq(["tag1", "tag2", "tag3", "tag4"]));

    let gerr = gerr.add_tags(["tag5", "tag6"]);
    assert!(
        gerr.iter_tags()
            .eq(["tag1", "tag2", "tag3", "tag4", "tag5", "tag6"])
    );

    let gerr = gerr.with_config::<ErrTags>();
    assert!(gerr.id().is_none());
    assert!(gerr.code().is_none());
    assert!(gerr.iter_tags().eq(["http"]));
}

use g_err::{Config, iterator::GErrNode};

struct ErrFuncA;
impl Config for ErrFuncA {
    type Id = i32;
    const CODE: Option<&'static str> = Some("CODE_A");
}

struct ErrFuncB;
impl Config for ErrFuncB {
    type Id = u64;
    fn id() -> Option<Self::Id> {
        Some(4000)
    }

    const TAGS: Option<&'static [&'static str]> = Some(&["tag1", "tag2"]);
}

#[test]
fn test_from_gerr() {
    let err = "qwe".parse::<i32>().unwrap_err();

    let gerr: GErr<ErrFuncA, (&'static str, String)> = GErr::new_with_id(123, "func_a error")
        .add_source(err)
        .set_data(("anu", "qwe".into()));
    assert_eq!(gerr.id().unwrap(), &123);
    if let GErrNode::LeafErr(e) = gerr.iter_source::<ParseIntError>().next().unwrap() {
        assert_eq!(e.to_string(), "invalid digit found in string");
    }
    assert_eq!(gerr.data().unwrap(), &("anu", String::from("qwe")));
    assert!(gerr.tags().is_none());
    assert_eq!(gerr.location().file, file!());

    let gerr: GErr<ErrFuncB> = GErr::from_gerr(gerr);
    assert_eq!(gerr.id().unwrap(), &4000);
    if let GErrNode::LeafErr(e) = gerr.iter_source::<ParseIntError>().next().unwrap() {
        assert_eq!(e.to_string(), "invalid digit found in string");
    }
    assert!(gerr.data().is_none());
    assert!(gerr.iter_tags().eq(["tag1", "tag2"]));
    assert_eq!(gerr.location().file, file!());
}
