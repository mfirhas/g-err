use g_err::*;

#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

#[test]
fn test_no_code() {
    let gerr: GErr<()> = GErr::new("test no code");

    assert_eq!(gerr.code(), None);
}

#[test]
fn test_auto_code() {
    let gerr: GErr<ErrAutoCode> = GErr::new("test auto code");

    assert_eq!(gerr.code(), Some("AutoCode"));
}

#[test]
fn test_no_code_set() {
    let gerr: GErr<()> = GErr::new("test no code").set_code("the-code");

    assert_eq!(gerr.code(), Some("the-code"));
}

#[test]
fn test_auto_code_set() {
    let gerr: GErr<ErrAutoCode> = GErr::new("test auto code").set_code("new_code");

    assert_eq!(gerr.code(), Some("new_code"));
}

#[test]
fn test_auto_code_prepend_append() {
    let gerr: GErr<ErrAutoCode> = GErr::new("test auto code");
    assert_eq!(gerr.code(), Some("AutoCode"));
}

#[test]
fn test_auto_code_set_prepend_append() {
    let gerr: GErr<ErrAutoCode> = GErr::new("test auto code").set_code("[FRE]");
    assert_eq!(gerr.code(), Some("[FRE]"));
}
