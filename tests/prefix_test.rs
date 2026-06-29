use g_err::*;

#[test]
fn test_no_prefix() {
    let gerr: GErr<NoID, NoPrefix> = GErr::new_auto("test no prefix");

    assert_eq!(gerr.prefix(), None);
}

struct AppPrefix;

impl Prefix for AppPrefix {
    const PREFIX: Option<&'static str> = Some("[APP]");
}

#[test]
fn test_auto_prefix() {
    let gerr: GErr<NoID, AppPrefix> = GErr::new_auto("test auto prefix");

    assert_eq!(gerr.prefix(), Some("[APP]"));
}

#[test]
fn test_no_prefix_set() {
    let gerr: GErr<NoID, NoPrefix> = GErr::new_auto("test no prefix").set_prefix("the-prefix");

    assert_eq!(gerr.prefix(), Some("the-prefix"));
}

#[test]
fn test_auto_prefix_set() {
    let gerr: GErr<NoID, AppPrefix> = GErr::new_auto("test auto prefix").set_prefix("new_prefix");

    assert_eq!(gerr.prefix(), Some("new_prefix"));
}

#[test]
fn test_auto_prefix_prepend_append() {
    let gerr: GErr<NoID, AppPrefix> = GErr::new_auto("test auto prefix");
    assert_eq!(gerr.prefix(), Some("[APP]"));

    let gerr = gerr.prepend_prefix("[MY_APP]");
    assert_eq!(gerr.prefix(), Some("[MY_APP][APP]"));

    let gerr = gerr.append_prefix("[HTTP]");
    assert_eq!(gerr.prefix(), Some("[MY_APP][APP][HTTP]"));
}

#[test]
fn test_auto_prefix_set_prepend_append() {
    let gerr: GErr<NoID, AppPrefix> = GErr::new_auto("test auto prefix").set_prefix("[FRE]");
    assert_eq!(gerr.prefix(), Some("[FRE]"));

    let gerr = gerr.prepend_prefix("[MY_APP]");
    assert_eq!(gerr.prefix(), Some("[MY_APP][FRE]"));

    let gerr = gerr.append_prefix("[HTTP]");
    assert_eq!(gerr.prefix(), Some("[MY_APP][FRE][HTTP]"));
}
