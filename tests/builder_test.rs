#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

use std::any::Any;

use g_err::*;

#[test]
fn test_default_auto_builder() {
    let gerr = GErr::<NoID>::new("default auto errors").set_prefix("[prefix]");
    dbg!(&gerr);
    assert_eq!(gerr.id(), &NoID);
    assert_eq!(gerr.prefix().unwrap(), "[prefix]");
}

#[test]
fn test_auto_builder() {
    let err = "asd".parse::<i32>().unwrap_err();
    let err2 = "400".parse::<u8>().unwrap_err();
    let gerr_source = GErr::<NoID, AutoPrefix, (&str, &str)>::new("the cause")
        .set_data(("kind", "not found"))
        .add_source(err2.clone());
    let gerr: GErr<AutoID, AutoPrefix, Data> = GErr::new("default auto errors")
        .prepend_prefix("@")
        .append_prefix("[user]")
        .set_sources([Source::Err(Box::new(err))])
        .add_source(err2)
        .add_source_gerr(gerr_source)
        .add_tags(["tag1"])
        .add_tag("tag2")
        .add_tag("tag3")
        .set_help("something is wrong")
        .set_data(Data {
            user_id: 123,
            user_name: "xXx".into(),
        })
        .set_field("user_name", "qwertty".to_string())
        .set_field("invalid_key", 34);
    dbg!(&gerr);
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "@AutoPrefix[user]");
    assert_eq!(gerr.sources().unwrap().len(), 3);
    let source1 = &gerr.sources().unwrap()[0];
    match source1 {
        Source::Err(err) => assert_eq!(err.to_string(), "invalid digit found in string"), // ParseIntError message
        Source::GErr(_) => panic!("kacau!"),
    }
    let source2 = &gerr.sources().unwrap()[1];
    match source2 {
        Source::Err(err) => assert_eq!(err.to_string(), "number too large to fit in target type"), // ParseIntError message
        Source::GErr(_) => panic!("kacau!"),
    }
    let source3 = &gerr.sources().unwrap()[2];
    match source3 {
        Source::Err(_) => panic!("opps"),
        Source::GErr(gerr) => {
            assert_eq!(gerr.id.to_string(), "NoID");
            assert_eq!(gerr.prefix.as_ref().unwrap(), "AutoPrefix");
            assert_eq!(
                (gerr.data.as_ref().unwrap().as_ref() as &dyn Any).downcast_ref::<(&str, &str)>(),
                Some(&("kind", "not found"))
            );
        }
    }
    let source3_2 = match source3 {
        Source::Err(_) => panic!("kacau"),
        Source::GErr(gerr) => {
            assert_eq!(gerr.sources.as_ref().unwrap().len(), 1);
            &gerr.sources.as_ref().unwrap()[0]
        }
    };
    match source3_2 {
        Source::Err(err) => assert_eq!(err.to_string(), "number too large to fit in target type"),
        Source::GErr(_) => panic!("waduh"),
    }

    assert!(gerr.iter_tags().eq(["tag1", "tag2", "tag3"]));
    assert_eq!(gerr.help().unwrap(), "something is wrong");
    assert_eq!(
        gerr.data().unwrap(),
        &Data {
            user_id: 123,
            user_name: "qwertty".into()
        }
    );
}

#[test]
fn test_manual_builder() {
    let gerr = GErr::<u64>::new_with_id(84, "manual id").append_prefix("lkj");
    assert_eq!(gerr.prefix().unwrap(), "lkj");
    assert_eq!(gerr.id(), &84);
    assert!(gerr.data().is_none());

    let gerr = GErr::<u64>::new_with_id(84, "manual id")
        .set_id(1324)
        .prepend_prefix("asd")
        .append_prefix("qwe");
    assert_eq!(gerr.id(), &1324);
    assert_eq!(gerr.prefix().unwrap(), "asdqwe");
}

#[test]
fn test_box() {
    let err = "asd".parse::<i32>().unwrap_err();
    let err2 = "400".parse::<u8>().unwrap_err();
    let gerr_source = GErr::<NoID, AutoPrefix, (&str, &str)>::new("the cause")
        .set_data(("kind", "not found"))
        .add_source(err2.clone());
    let gerr: GErrBox<AutoID, AutoPrefix, Data> = GErr::new("default auto errors")
        .prepend_prefix("@")
        .append_prefix("[user]")
        .set_sources([Source::Err(Box::new(err))])
        .add_source(err2)
        .add_source_gerr(gerr_source)
        .add_tags(["tag1"])
        .add_tag("tag2")
        .add_tag("tag3")
        .set_help("something is wrong")
        .set_data(Data {
            user_id: 123,
            user_name: "xXx".into(),
        })
        .set_field("user_name", "qwertty".to_string())
        .set_field("invalid_key", 34)
        .boxed();
    dbg!(&gerr);
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "@AutoPrefix[user]");
    assert_eq!(gerr.sources().unwrap().len(), 3);
    let source1 = &gerr.sources().unwrap()[0];
    match source1 {
        Source::Err(err) => assert_eq!(err.to_string(), "invalid digit found in string"), // ParseIntError message
        Source::GErr(_) => panic!("kacau!"),
    }
    let source2 = &gerr.sources().unwrap()[1];
    match source2 {
        Source::Err(err) => assert_eq!(err.to_string(), "number too large to fit in target type"), // ParseIntError message
        Source::GErr(_) => panic!("kacau!"),
    }
    let source3 = &gerr.sources().unwrap()[2];
    match source3 {
        Source::Err(_) => panic!("opps"),
        Source::GErr(gerr) => {
            assert_eq!(gerr.id.to_string(), "NoID");
            assert_eq!(gerr.prefix.as_ref().unwrap(), "AutoPrefix");
            assert_eq!(
                (gerr.data.as_ref().unwrap().as_ref() as &dyn Any).downcast_ref::<(&str, &str)>(),
                Some(&("kind", "not found"))
            );
        }
    }
    let source3_2 = match source3 {
        Source::Err(_) => panic!("kacau"),
        Source::GErr(gerr) => {
            assert_eq!(gerr.sources.as_ref().unwrap().len(), 1);
            &gerr.sources.as_ref().unwrap()[0]
        }
    };
    match source3_2 {
        Source::Err(err) => assert_eq!(err.to_string(), "number too large to fit in target type"),
        Source::GErr(_) => panic!("waduh"),
    }

    assert!(gerr.iter_tags().eq(["tag1", "tag2", "tag3"]));
    assert_eq!(gerr.help().unwrap(), "something is wrong");
    assert_eq!(
        gerr.data().unwrap(),
        &Data {
            user_id: 123,
            user_name: "qwertty".into()
        }
    );
}

#[test]
fn test_into_result() {
    let err = "asd".parse::<i32>().unwrap_err();
    let err2 = "400".parse::<u8>().unwrap_err();
    let gerr_source = GErr::<NoID, AutoPrefix, (&str, &str)>::new("the cause")
        .set_data(("kind", "not found"))
        .add_source(err2.clone());
    let gerr: Result<(), AutoID, AutoPrefix, Data> = GErr::new("default auto errors")
        .prepend_prefix("@")
        .append_prefix("[user]")
        .set_sources([Source::Err(Box::new(err))])
        .add_source(err2)
        .add_source_gerr(gerr_source)
        .add_tags(["tag1"])
        .add_tag("tag2")
        .add_tag("tag3")
        .set_help("something is wrong")
        .set_data(Data {
            user_id: 123,
            user_name: "xXx".into(),
        })
        .set_field("user_name", "qwertty".to_string())
        .set_field("invalid_key", 34)
        .result();
    let gerr = gerr.unwrap_err();
    dbg!(&gerr);
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "@AutoPrefix[user]");
    assert_eq!(gerr.sources().unwrap().len(), 3);
    let source1 = &gerr.sources().unwrap()[0];
    match source1 {
        Source::Err(err) => assert_eq!(err.to_string(), "invalid digit found in string"), // ParseIntError message
        Source::GErr(_) => panic!("kacau!"),
    }
    let source2 = &gerr.sources().unwrap()[1];
    match source2 {
        Source::Err(err) => assert_eq!(err.to_string(), "number too large to fit in target type"), // ParseIntError message
        Source::GErr(_) => panic!("kacau!"),
    }
    let source3 = &gerr.sources().unwrap()[2];
    match source3 {
        Source::Err(_) => panic!("opps"),
        Source::GErr(gerr) => {
            assert_eq!(gerr.id.to_string(), "NoID");
            assert_eq!(gerr.prefix.as_ref().unwrap(), "AutoPrefix");
            assert_eq!(
                (gerr.data.as_ref().unwrap().as_ref() as &dyn Any).downcast_ref::<(&str, &str)>(),
                Some(&("kind", "not found"))
            );
        }
    }
    let source3_2 = match source3 {
        Source::Err(_) => panic!("kacau"),
        Source::GErr(gerr) => {
            assert_eq!(gerr.sources.as_ref().unwrap().len(), 1);
            &gerr.sources.as_ref().unwrap()[0]
        }
    };
    match source3_2 {
        Source::Err(err) => assert_eq!(err.to_string(), "number too large to fit in target type"),
        Source::GErr(_) => panic!("waduh"),
    }

    assert!(gerr.iter_tags().eq(["tag1", "tag2", "tag3"]));
    assert_eq!(gerr.help().unwrap(), "something is wrong");
    assert_eq!(
        gerr.data().unwrap(),
        &Data {
            user_id: 123,
            user_name: "qwertty".into()
        }
    );
}

#[test]
fn test_box_into_result() {
    let err = "asd".parse::<i32>().unwrap_err();
    let err2 = "400".parse::<u8>().unwrap_err();
    let gerr_source: core::result::Result<(), GErr<_, _, _>> =
        GErr::<NoID, AutoPrefix, (&str, &str)>::new("the cause")
            .set_data(("kind", "not found"))
            .add_source(err2.clone())
            .into();
    let gerr_source = gerr_source.unwrap_err();
    let gerr: core::result::Result<i32, GErrBox<AutoID, AutoPrefix, Data>> =
        GErr::new("default auto errors")
            .prepend_prefix("@")
            .append_prefix("[user]")
            .set_sources([Source::Err(Box::new(err))])
            .add_source(err2)
            .add_source_gerr(gerr_source)
            .add_tags(["tag1"])
            .add_tag("tag2")
            .add_tag("tag3")
            .set_help("something is wrong")
            .set_data(Data {
                user_id: 123,
                user_name: "xXx".into(),
            })
            .set_field("user_name", "qwertty".to_string())
            .set_field("invalid_key", 34)
            .boxed()
            .into();
    let gerr = gerr.unwrap_err();
    dbg!(&gerr);
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "@AutoPrefix[user]");
    assert_eq!(gerr.sources().unwrap().len(), 3);
    let source1 = &gerr.sources().unwrap()[0];
    match source1 {
        Source::Err(err) => assert_eq!(err.to_string(), "invalid digit found in string"), // ParseIntError message
        Source::GErr(_) => panic!("kacau!"),
    }
    let source2 = &gerr.sources().unwrap()[1];
    match source2 {
        Source::Err(err) => assert_eq!(err.to_string(), "number too large to fit in target type"), // ParseIntError message
        Source::GErr(_) => panic!("kacau!"),
    }
    let source3 = &gerr.sources().unwrap()[2];
    match source3 {
        Source::Err(_) => panic!("opps"),
        Source::GErr(gerr) => {
            assert_eq!(gerr.id.to_string(), "NoID");
            assert_eq!(gerr.prefix.as_ref().unwrap(), "AutoPrefix");
            assert_eq!(
                (gerr.data.as_ref().unwrap().as_ref() as &dyn Any).downcast_ref::<(&str, &str)>(),
                Some(&("kind", "not found"))
            );
        }
    }
    let source3_2 = match source3 {
        Source::Err(_) => panic!("kacau"),
        Source::GErr(gerr) => {
            assert_eq!(gerr.sources.as_ref().unwrap().len(), 1);
            &gerr.sources.as_ref().unwrap()[0]
        }
    };
    match source3_2 {
        Source::Err(err) => assert_eq!(err.to_string(), "number too large to fit in target type"),
        Source::GErr(_) => panic!("waduh"),
    }

    assert!(gerr.iter_tags().eq(["tag1", "tag2", "tag3"]));
    assert_eq!(gerr.help().unwrap(), "something is wrong");
    assert_eq!(
        gerr.data().unwrap(),
        &Data {
            user_id: 123,
            user_name: "qwertty".into()
        }
    );
}
