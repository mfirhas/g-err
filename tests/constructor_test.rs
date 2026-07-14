use g_err::GErr;

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

    // auto id, auto prefix, with data
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
