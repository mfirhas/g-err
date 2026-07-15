use g_err::*;

#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

#[test]
fn test_no_tags() {
    let gerr: GErr<()> = GErr::new("test no prefix");

    assert_eq!(gerr.tags(), None);
}

#[test]
fn test_add_tags() {
    let gerr: GErr<ErrAutoCode> = GErr::new("test auto prefix").add_tags(["tag1", "tag2"]);

    assert_eq!(gerr.tags().unwrap().iter().len(), 2);
    assert!(gerr.iter_tags().eq(["tag1", "tag2"]));
}

#[test]
fn test_add_tag() {
    let gerr: GErr<ErrAutoCode> = GErr::new("test auto prefix")
        .add_tag("tag1")
        .add_tag("tag2")
        .add_tag("tag3");

    assert_eq!(gerr.tags().unwrap().iter().len(), 3);
    assert!(gerr.iter_tags().eq(["tag1", "tag2", "tag3"]));
}
