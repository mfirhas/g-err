use g_err::*;

#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

#[test]
fn multiple_builder_fields() {
    let inner = gerr!("inner");

    let external_error = "anu".parse::<i32>().unwrap_err();
    let err = gerr!(
        "failed {}",
        500;
        config=ErrIDi32,
        id = 999,
        code = "HTTP",
        tag = "server",
        tags = ["api", "v1"],
        data = "payload",
        source = external_error,
        gerr = inner,
        help = "Try parsing valid signed integer 32",
    );

    assert_eq!(err.message(), "failed 500");
    assert_eq!(*err.id().unwrap(), 999);
    assert_eq!(err.code(), Some("HTTP"));
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
        config = ErrIDi32,
        id = 123,
    );

    assert_eq!(*err.id().unwrap(), 123);
}

#[test]
fn interpolation_style_usage() {
    let name = "alice";

    let err = gerr!(format!("hello {name}"));

    assert_eq!(err.message(), "hello alice");
}

#[test]
fn test_macro_default() {
    let gerr = gerr!("sdf: {}", "qwe");

    dbg!(&gerr);
    assert!(gerr.code().is_none());
    assert!(gerr.data().is_none());
}

#[test]
fn test_macro_set_manual_id() {
    let gerr = gerr!(
        "test: {}", 123;
        config = ErrIDi32,
        id=123,
    );

    dbg!(&gerr);
    assert_eq!(gerr.id().unwrap(), &123);
}

#[test]
fn test_macro_set_manual_code() {
    let gerr = gerr!(
        "test: {}", 123;
        code="anu",
    );

    dbg!(&gerr);
    assert_eq!(gerr.code().unwrap(), "anu");
}

#[test]
fn test_macro_set_manual_data() {
    let gerr = gerr!("test data: {}", "data"; data = ("username", "ajo"));

    dbg!(&gerr);
    assert_eq!(gerr.data().unwrap(), &("username", "ajo"));
}

#[test]
fn test_macro_set_manual_all() {
    let gerr = gerr!("manual all: {}", 4; code = "[TEST]", config=ErrIDi32, id=123, data=234_u64);

    dbg!(&gerr);

    assert_eq!(gerr.id().unwrap(), &123);
    assert!(gerr.code().is_none());
    assert_eq!(gerr.data().unwrap(), &234);
}

#[test]
fn test_macro_id_auto() {
    let gerr = gerr!("asdsdf: {}", 234; config=ErrAutoID);
    dbg!(&gerr);

    assert_eq!(gerr.id().unwrap().to_string(), "AutoID");
}

#[test]
fn test_macro_code_auto() {
    let gerr = gerr!("asdsdf: {}", 234; config=ErrAutoCode);
    dbg!(&gerr);

    assert_eq!(gerr.code().unwrap(), "AutoCode");
}

#[test]
fn test_macro_data_type() {
    let gerr = gerr!("sldkvm: {}", 345; data=123, data_type = (&str, u64));

    dbg!(&gerr);

    assert!(gerr.data().is_none());
}

#[test]
fn test_macro_auto_all() {
    let gerr = gerr!("test: {}", 4; config=ErrAutoIDCode, data_type = i128);
    dbg!(&gerr);

    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "AutoCode");
}

#[test]
fn test_macro_id_auto_return() {
    let gerr: GErr<ErrAutoID> = gerr!("test: {}", 5; config, code="asd");

    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "asd");
}

#[test]
fn test_macro_code_auto_return() {
    let gerr: GErr<ErrAutoCode> = gerr!("test: {}", 5; config=ErrAutoCode);

    dbg!(&gerr);
    assert_eq!(gerr.code().unwrap(), "AutoCode");

    let gerr: GErr<ErrAutoIDCode> = gerr!("test: {}", 5; config, code = "user");

    dbg!(&gerr);
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "user");
}

#[test]
fn test_macro_data_type_return() {
    let gerr: GErr<_, (&str, u32)> = gerr!("sdf"; data_type);

    dbg!(&gerr);
    assert!(gerr.code().is_none());
    assert!(gerr.data().is_none());

    let gerr: GErr<_, _> = gerr!("sdf"; data_type = u32);

    dbg!(&gerr);
    assert!(gerr.code().is_none());
    assert!(gerr.data().is_none());

    let gerr: GErr<ErrAutoIDCode, i128> = gerr!("sdf"; data_type, config);
    let gerr = gerr.set_data(1230);

    dbg!(&gerr);
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "AutoCode");
    assert_eq!(gerr.data().unwrap(), &1230);
}

#[test]
fn test_macro_all() {
    let external_error = "anu".parse::<i32>().unwrap_err();
    let gerr = gerr!(
        "test all";
        config=ErrAutoID,
        code = "APP",
        data="test",
        tags=["tag1", "tag2"],

        source = gerr!("error source"),
        gerr = gerr!("another cause: {}", 123; tag="sdf", data=234, source=external_error.clone(), gerr=gerr!("dalem"; config=ErrAutoIDCode, data=("username", "babi"))),
        source = external_error,
        tag = "nganu",
    );

    assert_eq!(gerr.sources().unwrap().len(), 3);
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "APP");
    assert!(gerr.iter_tags().eq(["tag1", "tag2", "nganu"]));

    let source1 = &gerr.sources().unwrap()[0];
    let source2 = &gerr.sources().unwrap()[1];
    let source3 = &gerr.sources().unwrap()[2];

    match source1 {
        Source::Err(err) => assert_eq!(err.to_string(), "[-][-] error source"),
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
                    assert_eq!(inner.to_string(), "dalem");
                    assert_eq!(inner.id.as_ref().unwrap().to_string(), "AutoID");
                    assert_eq!(inner.code.as_ref().unwrap(), "AutoCode");

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
