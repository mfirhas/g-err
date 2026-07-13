use g_err::*;

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

#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
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

#[test]
fn test_macro_id_auto_return() {
    let gerr: GErr<AutoID> = gerr!("test: {}", 5; id_auto, prefix="[asd]");

    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "[asd]");
}

#[test]
fn test_macro_prefix_auto_return() {
    let gerr: GErr<_, AutoPrefix> = gerr!("test: {}", 5; prefix_auto);

    dbg!(&gerr);
    assert_eq!(gerr.code().unwrap(), "AutoPrefix");

    let gerr: GErr<AutoID, AutoPrefix> =
        gerr!("test: {}", 5; prefix_auto, id_auto, aprefix = "-user");

    dbg!(&gerr);
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix-user");
}

#[test]
fn test_macro_data_type_return() {
    let gerr: GErr<_, _, (&str, u32)> = gerr!("sdf"; data_type);

    dbg!(&gerr);
    assert!(gerr.code().is_none());
    assert!(gerr.data().is_none());

    let gerr: GErr<_, _, _> = gerr!("sdf"; data_type = u32);

    dbg!(&gerr);
    assert!(gerr.code().is_none());
    assert!(gerr.data().is_none());

    let gerr: GErr<AutoID, _, i128> = gerr!("sdf"; data_type, id_auto, prefix_auto = AutoPrefix);
    let gerr = gerr.set_data(1230);

    dbg!(&gerr);
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");
    assert_eq!(gerr.data().unwrap(), &1230);
}

#[test]
fn test_macro_all() {
    let external_error = "anu".parse::<i32>().unwrap_err();
    let gerr = gerr!(
        "test all";
        id_auto=AutoID,
        prefix = "[APP]",
        data="test",
        tags=["tag1", "tag2"],

        source = gerr!("error source"),
        gerr = gerr!("another cause: {}", 123; tag="sdf", data=234, source=external_error.clone(), gerr=gerr!("dalem"; id_auto=AutoID, prefix_auto=AutoPrefix, data=("username", "babi"))),
        source = external_error,

        pprefix = "[E002]",
        aprefix = "[User]",
        tag = "nganu",
    );

    assert_eq!(gerr.sources().unwrap().len(), 3);
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "[E002][APP][User]");
    assert!(gerr.iter_tags().eq(["tag1", "tag2", "nganu"]));

    let source1 = &gerr.sources().unwrap()[0];
    let source2 = &gerr.sources().unwrap()[1];
    let source3 = &gerr.sources().unwrap()[2];

    match source1 {
        Source::Err(err) => assert_eq!(err.to_string(), "error source"),
        Source::GErr(_) => panic!("shouvebeen Source::Err"),
    }

    match source2 {
        Source::GErr(err) => {
            assert_eq!(err.to_string(), "another cause: 123");
            assert_eq!(err.tags.as_ref().unwrap(), &["sdf"]);
            assert!(err.data.is_some());

            assert_eq!(err.sources.as_ref().unwrap().len(), 2);

            let nested_source1 = &err.sources.as_ref().unwrap()[0];
            let nested_source2 = &err.sources.as_ref().unwrap()[1];

            match nested_source1 {
                Source::Err(_) => {}
                Source::GErr(_) => panic!("should have been Source::Err"),
            }

            match nested_source2 {
                Source::GErr(inner) => {
                    assert_eq!(inner.to_string(), "AutoPrefix dalem");
                    assert_eq!(inner.id.to_string(), "AutoID");
                    assert_eq!(inner.code.as_ref().unwrap(), "AutoPrefix");

                    assert!(inner.data.is_some());

                    assert!(inner.sources.is_none());
                    assert!(inner.tags.is_none());
                }
                Source::Err(_) => panic!("should have been Source::GErr"),
            }
        }
        Source::Err(_) => panic!("should have been Source::GErr"),
    }

    match source3 {
        Source::Err(_) => {}
        Source::GErr(_) => panic!("shouvebeen Source::Err"),
    }
}
