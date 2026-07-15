#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

use std::any::Any;

use g_err::*;

#[test]
fn test_default_auto_builder() {
    let gerr = GErr::<()>::new("default auto errors").set_code("code");
    assert!(gerr.id().is_none());
    assert_eq!(gerr.code().unwrap(), "code");
}

#[test]
fn test_auto_builder() {
    let err = "asd".parse::<i32>().unwrap_err();
    let err2 = "400".parse::<u8>().unwrap_err();
    let gerr_source = GErr::<ErrAutoCode, (&str, &str)>::new("the cause")
        .set_data(("kind", "not found"))
        .add_source(err2.clone());
    let gerr: GErr<ErrAutoIDCode, Data> = GErr::new("default auto errors")
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
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "AutoCode");
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
            assert!(gerr.id.is_none());
            assert_eq!(gerr.code.as_ref().unwrap(), "AutoCode");
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
    let gerr = GErr::<ErrIDi32>::new_with_id(84, "manual id");
    assert_eq!(gerr.id().unwrap(), &84);
    assert!(gerr.data().is_none());

    let gerr = GErr::<ErrIDi32>::new_with_id(84, "manual id")
        .set_id(1324)
        .set_code("asdqwe");
    assert_eq!(gerr.id().unwrap(), &1324);
    assert_eq!(gerr.code().unwrap(), "asdqwe");
}

#[test]
fn test_box() {
    let err = "asd".parse::<i32>().unwrap_err();
    let err2 = "400".parse::<u8>().unwrap_err();
    let gerr_source = GErr::<ErrAutoCode, (&str, &str)>::new("the cause")
        .set_data(("kind", "not found"))
        .add_source(err2.clone());
    let gerr: GErrBox<ErrAutoIDCode, Data> = GErr::new("default auto errors")
        .set_code("E-001")
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
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "E-001");
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
            assert!(gerr.id.is_none());
            assert_eq!(gerr.code.as_ref().unwrap(), "AutoCode");
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
    let gerr_source = GErr::<ErrAutoCode, (&str, &str)>::new("the cause")
        .set_data(("kind", "not found"))
        .add_source(err2.clone());
    let gerr: Result<(), ErrAutoIDCode, Data> = GErr::new("default auto errors")
        .set_code("E234")
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
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "E234");
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
            assert!(gerr.id.is_none());
            assert_eq!(gerr.code.as_ref().unwrap(), "AutoCode");
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
    let gerr_source: core::result::Result<(), GErr<_, _>> =
        GErr::<ErrAutoCode, (&str, &str)>::new("the cause")
            .set_data(("kind", "not found"))
            .add_source(err2.clone())
            .into();
    let gerr_source = gerr_source.unwrap_err();
    let gerr: core::result::Result<i32, GErrBox<ErrAutoIDCode, Data>> =
        GErr::new("default auto errors")
            .set_code("E-890")
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
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "E-890");
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
            assert!(gerr.id.is_none());
            assert_eq!(gerr.code.as_ref().unwrap(), "AutoCode");
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
