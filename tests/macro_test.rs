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

    match &sources[0] {
        Source::Err(_) => {}
        _ => panic!("expected Source::Err"),
    }
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

    match &sources[0] {
        Source::GErr(_) => {}
        _ => panic!("expected Source::GErr"),
    }
}

#[test]
fn multiple_builder_fields() {
    let inner = gerr!("inner");

    let err = gerr!(
        "failed {}",
        500;
        id = 999u32,
        prefix = "HTTP",
        tag = "server",
        tags = ["api", "v1"],
        data = "payload",
        gerr = inner,
    );

    assert_eq!(err.message(), "failed 500");
    assert_eq!(*err.id(), 999);
    assert_eq!(err.prefix(), Some("HTTP"));
    assert_eq!(err.data(), Some(&"payload"));

    let tags = err.tags().unwrap();
    assert_eq!(tags.len(), 3);

    let sources = err.sources().unwrap();
    assert_eq!(sources.len(), 1);

    match &sources[0] {
        Source::GErr(_) => {}
        _ => panic!("expected Source::GErr"),
    }
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
