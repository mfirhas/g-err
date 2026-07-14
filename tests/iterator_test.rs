#[path = "setup_test.rs"]
mod setup_test;
use std::num::ParseIntError;

use setup_test::*;

use g_err::{iterator::GErrNode, *};

#[test]
fn test_iterator() {
    let err = "asd".parse::<i32>().unwrap_err();
    let err2 = "400".parse::<u8>().unwrap_err();
    let gerr_source: core::result::Result<(), GErr<_, _>> =
        GErr::<ErrAutoCode, (&str, &str)>::new("the cause")
            .set_data(("kind", "not found"))
            .add_source(err2.clone())
            .add_source_gerr(gerr!("undefined error!"; code = "[UNDEFINED]"))
            .into();
    let gerr_source = gerr_source.unwrap_err();
    let gerr: core::result::Result<i32, GErrBox<ErrAutoIDCode, Data>> =
        GErr::new("default auto errors")
            .set_code("CODE")
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
    assert_eq!(gerr.code().unwrap(), "CODE");
    assert_eq!(gerr.sources().unwrap().len(), 3);

    for e in &*gerr {
        println!("{e}");
        dbg!("{:#?}", e);
    }

    for e in &gerr {
        println!("{e}");
        dbg!("{:#?}", e);
    }

    for e in gerr.iter() {
        println!("{e}");
        dbg!("{:#?}", e);
    }

    for (i, e) in gerr.as_ref().iter().enumerate() {
        match i {
            0 => {
                if let GErrNode::Root(root) = e {
                    assert_eq!(root.message(), "default auto errors");
                    assert_eq!(root.code().unwrap(), "@AutoPrefix[user]");
                    assert_eq!(root.iter_tags().count(), 3);
                } else {
                    panic!("first node should be root");
                }
            }
            1 => {
                if let GErrNode::LeafErr(err) = e {
                    if let Some(pie) = err.downcast_ref::<ParseIntError>() {
                        assert_eq!(pie.to_string(), "invalid digit found in string");
                    } else {
                        panic!("should be parsed into ParseIntError");
                    }
                } else {
                    panic!("second node should be err");
                }
            }
            2 => {
                if let GErrNode::LeafErr(err) = e {
                    if let Some(pie) = err.downcast_ref::<ParseIntError>() {
                        assert_eq!(pie.to_string(), "number too large to fit in target type");
                    } else {
                        panic!("should be parsed into ParseIntError");
                    }
                } else {
                    panic!("third node should be err");
                }
            }
            3 => {
                if let GErrNode::LeafGErr(gerr_source) = e {
                    assert_eq!(gerr_source.message, "the cause");
                    assert_eq!(gerr_source.id.as_ref().unwrap().to_string(), "NoID");
                    assert_eq!(gerr_source.code.as_ref().unwrap(), "AutoPrefix");
                    if let Some(data) = (&**gerr_source.data.as_ref().unwrap()
                        as &dyn core::any::Any)
                        .downcast_ref::<(&'static str, &'static str)>()
                    {
                        assert_eq!(data.0, "kind");
                        assert_eq!(data.1, "not found");
                    } else {
                        panic!("fourth node's data should be (&str, &str)");
                    }
                    if let Source::Err(ref err) = gerr_source.sources.as_ref().unwrap()[0] {
                        if let Some(the_e) = (&*err).downcast_ref::<ParseIntError>() {
                            assert_eq!(the_e.to_string(), "number too large to fit in target type");
                        } else {
                            panic!("fourth node's source should be ParseIntError");
                        }
                    } else {
                        panic!("fourth node's source should be exist");
                    }
                    if let Source::GErr(ref err) = gerr_source.sources.as_ref().unwrap()[1] {
                        assert_eq!(err.message, "undefined error!");
                        assert_eq!(err.code.as_ref().unwrap(), "[UNDEFINED]");
                    } else {
                        panic!("fourth node's second source should be exist");
                    }
                } else {
                    panic!("fourth node should be gerr");
                }
            }
            4 => {
                if let GErrNode::LeafErr(err) = e {
                    if let Some(pie) = err.downcast_ref::<ParseIntError>() {
                        assert_eq!(pie.to_string(), "number too large to fit in target type");
                    } else {
                        panic!("should be parsed into ParseIntError");
                    }
                } else {
                    panic!("fifth node should be err");
                }
            }
            5 => {
                if let GErrNode::LeafGErr(ge) = e {
                    assert_eq!(ge.message, "undefined error!");
                    assert_eq!(ge.code.as_ref().unwrap(), "[UNDEFINED]");
                } else {
                    panic!("fourth node's second source should be exist");
                }
            }
            _ => panic!("there should only 6 items in gerr nodes"),
        }
    }

    let all_nodes_into_iter = gerr.into_iter().count();
    let all_nodes = gerr.iter().count();
    assert_eq!(all_nodes_into_iter, all_nodes);
}
