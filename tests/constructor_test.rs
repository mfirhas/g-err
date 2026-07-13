use g_err::{GErr, NoID};

#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

#[test]
fn test_new_auto_default() {
    let gerr: GErr = GErr::new("auto default");

    assert_eq!(gerr.message(), "auto default");
    assert_eq!(gerr.id(), &NoID);
    assert!(gerr.code().is_none());
    assert!(gerr.data().is_none());
}

#[test]
fn test_new_manual_id() {
    let gerr: GErr<u32> = GErr::new_with_id(123, "manual id");

    assert_eq!(gerr.message(), "manual id");
    assert_eq!(gerr.id(), &123);
    assert!(gerr.prefix().is_none());
    assert!(gerr.data().is_none());
}

// test new with auto-generated id.
#[test]
fn test_new_auto() {
    // auto id, no prefix, no data
    let gerr = GErr::<AutoID>::new("error");
    assert_eq!(gerr.message(), "error");
    assert_eq!(gerr.id(), &AutoID);

    // no id, auto prefix, no data
    let gerr = GErr::<NoID, AutoPrefix>::new("auto prefix");
    assert_eq!(gerr.id(), &NoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");

    // auto id, auto prefix
    let gerr: GErr<AutoID, AutoPrefix> = GErr::new("id and prefix are auto");
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");
    assert_eq!(gerr.message(), "id and prefix are auto");

    // auto id, auto prefix, with data
    let gerr: GErr<AutoID, AutoPrefix, Data> = GErr::new("all").set_data(Data {
        user_id: 234,
        user_name: "xxx".into(),
    });
    assert_eq!(gerr.message(), "all");
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");
    assert_eq!(gerr.data().unwrap().user_id, 234);
    assert_eq!(gerr.data().unwrap().user_name, "xxx");

    let gerr: GErr<AutoID, AutoPrefix, Data> = GErr::new("all")
        .set_field("user_id", 400)
        .set_field("user_name", "yyy".to_string());
    assert_eq!(gerr.message(), "all");
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");
    assert_eq!(gerr.data().unwrap().user_id, 400);
    assert_eq!(gerr.data().unwrap().user_name, "yyy");
}

// test new with manually-set id.
#[test]
fn test_new_manual() {
    let gerr = GErr::<_>::new_with_id(23, "test");
    assert_eq!(gerr.id(), &23);
    assert_eq!(gerr.message(), "test");
    assert!(gerr.code().is_none());

    let gerr = GErr::<&'static str>::new_with_id("zxc", "test").set_prefix("prefix");
    assert_eq!(gerr.id(), &"zxc");
    assert_eq!(gerr.message(), "test");
    assert_eq!(gerr.prefix().unwrap(), "prefix");
}

#[test]
fn test_from_non_gerr() {
    let err = "asd".parse::<i32>().unwrap_err();
    let gerr = GErr::<AutoID>::from_error(err);
    assert_eq!(gerr.id(), &AutoID);
    assert!(gerr.prefix().is_none());
    assert_eq!(gerr.message(), "invalid digit found in string");
}

#[test]
fn test_from_non_gerr_id() {
    let err = "asd".parse::<i32>().unwrap_err();
    let gerr: GErr<_, AutoPrefix> = GErr::from_error_with_id(123, err);
    assert_eq!(gerr.id(), &123);
    assert_eq!(gerr.code().unwrap(), "AutoPrefix");
    assert_eq!(gerr.message(), "invalid digit found in string");
}
