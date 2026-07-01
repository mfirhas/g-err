use g_err::*;
use std::io;

#[test]
fn plain_message() {
    let err = gerr!("hello");

    assert_eq!(err.message(), "hello");
    assert!(err.tags().is_none());
    assert!(err.data().is_none());
    assert!(err.sources().is_none());
}

#[test]
fn formatted_message() {
    let name = "john";

    let err = gerr!("hello {}", name);

    assert_eq!(err.message(), "hello john");
}

#[test]
fn formatted_message_multiple_args() {
    let name = "john";
    let age = 42;

    let err = gerr!("{} is {}", name, age);

    assert_eq!(err.message(), "john is 42");
}

#[test]
fn arbitrary_expression_message() {
    let msg = String::from("dynamic");

    let err = gerr!(msg.clone());

    assert_eq!(err.message(), msg);
}

#[test]
fn id_builder() {
    let err = gerr!(
        "oops";
        id = 123u32
    );

    assert_eq!(*err.id(), 123);
}

#[test]
fn prefix_builder() {
    let err = gerr!(
        "oops";
        prefix = "HTTP"
    );

    assert_eq!(err.prefix(), Some("HTTP"));
}

#[test]
fn data_builder() {
    let err = gerr!(
        "oops";
        data = 999u32
    );

    assert_eq!(err.data(), Some(&999));
}

#[test]
fn tag_builder() {
    let err = gerr!(
        "oops";
        tag = "one"
    );

    let tags = err.tags().unwrap();

    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], "one");
}

#[test]
fn tags_builder_array() {
    let err = gerr!(
        "oops";
        tags = ["one", "two"]
    );

    let tags = err.tags().unwrap();

    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], "one");
    assert_eq!(tags[1], "two");
}

#[test]
fn tags_builder_vec() {
    let err = gerr!(
        "oops";
        tags = vec!["one", "two"]
    );

    let tags = err.tags().unwrap();

    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], "one");
    assert_eq!(tags[1], "two");
}

#[test]
fn source_builder() {
    let source = io::Error::other("disk failure");

    let err = gerr!(
        "outer";
        source = source
    );

    let sources = err.sources().unwrap();

    assert_eq!(sources.len(), 1);
}

#[test]
fn gerr_builder() {
    let inner = gerr!("inner");

    let err = gerr!(
        "outer";
        gerr = inner
    );

    let sources = err.sources().unwrap();

    assert_eq!(sources.len(), 1);
}

#[test]
fn multiple_builder_fields() {
    let inner = gerr!("inner");

    let external_error = "anu".parse::<i32>().unwrap_err();
    let err = gerr!(
        "failed {}",
        500;
        id = 999u32,
        prefix = "HTTP",
        tag = "server",
        tags = ["api", "v1"],
        data = "payload",
        source = external_error,
        gerr = inner,
        help = "Try parsing valid signed integer 32",
    );

    assert_eq!(err.message(), "failed 500");
    assert_eq!(*err.id(), 999);
    assert_eq!(err.prefix(), Some("HTTP"));
    assert_eq!(err.data(), Some(&"payload"));
    assert_eq!(err.help(), Some("Try parsing valid signed integer 32"));

    let tags = err.tags().unwrap();
    assert_eq!(tags.len(), 3);

    let sources = err.sources().unwrap();
    assert_eq!(sources.len(), 2);
}

#[test]
fn trailing_comma() {
    let err = gerr!(
        "hello";
        id = 123,
    );

    assert_eq!(*err.id(), 123);
}

#[test]
fn interpolation_style_usage() {
    let name = "alice";

    let err = gerr!(format!("hello {name}"));

    assert_eq!(err.message(), "hello alice");
}

#[derive(Debug, PartialEq, Eq)]
struct AutoID;

impl core::fmt::Display for AutoID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AutoID")
    }
}

impl Id for AutoID {
    fn id() -> Self {
        Self
    }
}

struct AutoPrefix;

impl Prefix for AutoPrefix {
    const PREFIX: Option<&'static str> = Some("AutoPrefix");
}

#[test]
fn test_macro_default() {
    let gerr = gerr!("sdf: {}", "qwe");

    dbg!(&gerr);
    assert!(gerr.prefix().is_none());
    assert!(gerr.data().is_none());
}

#[test]
fn test_macro_set_manual_id() {
    let gerr = gerr!(
        "test: {}", 123;
        id=123,
    );

    dbg!(&gerr);
    assert_eq!(gerr.id(), &123);
}

#[test]
fn test_macro_set_manual_prefix() {
    let gerr = gerr!(
        "test: {}", 123;
        prefix="anu",
        pprefix = "[TEST]",
        aprefix = "-asd",
    );

    dbg!(&gerr);
    assert_eq!(gerr.prefix().unwrap(), "[TEST]anu-asd");
}

#[test]
fn test_macro_set_manual_data() {
    let gerr = gerr!("test data: {}", "data"; data = ("username", "ajo"));

    dbg!(&gerr);
    assert_eq!(gerr.data().unwrap(), &("username", "ajo"));
}

#[test]
fn test_macro_set_manual_all() {
    let gerr =
        gerr!("manual all: {}", 4; prefix = "[TEST]", aprefix = "-user", id=123, data=234_u64);

    dbg!(&gerr);

    assert_eq!(gerr.id(), &123);
    assert_eq!(gerr.prefix().unwrap(), "[TEST]-user");
    assert_eq!(gerr.data().unwrap(), &234);
}

#[test]
fn test_macro_id_auto() {
    let gerr = gerr!("asdsdf: {}", 234; id_auto=AutoID);
    dbg!(&gerr);

    assert_eq!(gerr.id().to_string(), "AutoID");
}

#[test]
fn test_macro_prefix_auto() {
    let gerr = gerr!("asdsdf: {}", 234; prefix_auto=AutoPrefix, aprefix="-user");
    dbg!(&gerr);

    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix-user");
}

#[test]
fn test_macro_data_type() {
    let gerr = gerr!("sldkvm: {}", 345; data=123, data_type = (&str, u64));

    dbg!(&gerr);

    assert!(gerr.data().is_none());
}

#[test]
fn test_macro_auto_all() {
    let gerr = gerr!("test: {}", 4; prefix_auto=AutoPrefix, id_auto=AutoID, data_type = i128);
    dbg!(&gerr);

    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");
}
