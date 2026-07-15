use g_err::*;

#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

#[test]
fn test_no_prefix() {
    let gerr: GErr<()> = GErr::new("test no prefix");

    assert_eq!(gerr.code(), None);
}

#[test]
fn test_auto_prefix() {
    let gerr: GErr<ErrAutoCode> = GErr::new("test auto prefix");

    assert_eq!(gerr.code(), Some("AutoCode"));
}

#[test]
fn test_no_prefix_set() {
    let gerr: GErr<()> = GErr::new("test no prefix").set_code("the-prefix");

    assert_eq!(gerr.code(), Some("the-prefix"));
}

#[test]
fn test_auto_prefix_set() {
    let gerr: GErr<ErrAutoCode> = GErr::new("test auto prefix").set_code("new_prefix");

    assert_eq!(gerr.code(), Some("new_prefix"));
}

#[test]
fn test_auto_prefix_prepend_append() {
    let gerr: GErr<ErrAutoCode> = GErr::new("test auto prefix");
    assert_eq!(gerr.code(), Some("AutoCode"));
}

#[test]
fn test_auto_prefix_set_prepend_append() {
    let gerr: GErr<ErrAutoCode> = GErr::new("test auto prefix").set_code("[FRE]");
    assert_eq!(gerr.code(), Some("[FRE]"));
}
