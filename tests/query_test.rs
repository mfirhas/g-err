#[path = "setup_test.rs"]
mod setup_test;

use std::num::ParseIntError;

use setup_test::*;

use g_err::{iterator::GErrNode, *};

#[test]
fn test_iterator_id() {
    let int_err = "wer".parse::<i32>().unwrap_err();
    let gerr = gerr!("id iterator error: {}", 1;
        config=ErrAutoIDCode,
        source=int_err,
        gerr=gerr!("testing: {}", "abc"; config=ErrIDi32, id=123, gerr=gerr!("error"; config=ErrAutoID, code="anu")),
        gerr=gerr!("source err: {}", 69; tag="69", gerr=gerr!("asd"; config=ErrAutoID, gerr=gerr!("xcv";config=ErrIDi32, id=42))),
    );

    let ids_by_type = gerr.iter_id::<i32>(); // 2nd source(gerr), id=123 and id=42 are defaulted to i32.
    assert_eq!(ids_by_type.count(), 2);
    let ids = gerr.iter_id::<i32>().collect::<Vec<_>>();
    assert_eq!(ids.len(), 2);
    for (i, v) in ids.iter().enumerate() {
        match i {
            0 => {
                if let GErrNode::LeafGErr(gerr) = v {
                    assert_eq!(gerr.id.as_ref().unwrap().to_string(), "123");
                    assert_eq!(gerr.message, "testing: abc");
                    assert_eq!(gerr.sources.as_ref().unwrap().len(), 1);
                } else {
                    panic!("shouvebeen gerr!");
                }
            }
            1 => {
                if let GErrNode::LeafGErr(gerr) = v {
                    assert_eq!(gerr.id.as_ref().unwrap().to_string(), "42");
                    assert_eq!(gerr.message, "xcv");
                } else {
                    panic!("shouvebeen gerr!");
                }
            }
            _ => panic!("shouvebeen only 2 nodes with id of i32"),
        }
    }

    let ids_by_value = gerr.iter_by_id(&123);
    assert_eq!(ids_by_value.count(), 1);

    let ids_by_value = gerr.iter_by_id(&42);
    assert_eq!(ids_by_value.count(), 1);

    let ids_by_value = gerr.iter_by_id(&AutoID);
    assert_eq!(ids_by_value.count(), 3);

    for (i, v) in gerr.iter_id::<AutoID>().enumerate() {
        match i {
            0 => {
                if let GErrNode::Root(root) = v {
                    assert_eq!(root.message(), "id iterator error: 1");
                    assert_eq!(root.id().unwrap(), &AutoID);
                    assert_eq!(root.code().unwrap(), "AutoCode");
                    assert_eq!(root.sources().as_ref().unwrap().len(), 3);
                } else {
                    panic!("first node shouvebeen Root");
                }
            }
            1 => {
                if let GErrNode::LeafGErr(gerr) = v {
                    assert_eq!(gerr.message, "error");
                    assert_eq!(gerr.id.as_ref().unwrap().to_string(), "AutoID");
                    assert_eq!(gerr.code.as_ref().unwrap(), "anu");
                } else {
                    panic!("second node shouvebeen gerr");
                }
            }
            2 => {
                if let GErrNode::LeafGErr(gerr) = v {
                    assert_eq!(gerr.message, "asd");
                    assert_eq!(gerr.id.as_ref().unwrap().to_string(), "AutoID");
                    assert_eq!(gerr.sources.as_ref().unwrap().len(), 1);
                } else {
                    panic!("third node shouvebeen gerr");
                }
            }
            _ => panic!("shouvebeen only 3 nodes with id of AutoID"),
        }
    }
}

#[test]
fn test_iter_code() {
    let int_err = "wer".parse::<i32>().unwrap_err();
    let gerr = gerr!("id iterator error: {}", 1;
        config=ErrAutoIDCode,
        source=int_err,
        gerr=gerr!("testing: {}", "abc"; config=ErrIDi32, id=123, gerr=gerr!("error"; config=ErrAutoID, code="anu")),
        gerr=gerr!("source err: {}", 69; tag="69", gerr=gerr!("asd"; config=ErrAutoID, gerr=gerr!("xcv"; config=ErrIDi32, id=42))),
    );

    let iter_by_code = gerr.iter_by_code("anu").count();
    assert_eq!(iter_by_code, 1);

    let iter_by_code = gerr.iter_by_code("AutoCode").count();
    assert_eq!(iter_by_code, 1);
}

#[test]
fn test_iter_tags() {
    let int_err = "wer".parse::<i32>().unwrap_err();
    let gerr = gerr!("id iterator error: {}", 1;
        config=ErrAutoIDCode,
        source=int_err,
        gerr=gerr!("testing: {}", "abc"; config=ErrIDi32, id=123, gerr=gerr!("error"; config=ErrAutoID, code="anu")),
        gerr=gerr!("source err: {}", 69; tag="69", gerr=gerr!("asd"; config=ErrAutoID, gerr=gerr!("xcv";config=ErrIDi32, id=42))),
        tags=["tag1", "tag2"],
    );

    let mut iter_gerr_by_tag = gerr.iter_by_tag("69");
    let iter_by_tag = iter_gerr_by_tag.next().unwrap();
    if let GErrNode::LeafGErr(gerr) = iter_by_tag {
        assert_eq!(gerr.message, "source err: 69");
        assert_eq!(gerr.tags.as_ref().unwrap().len(), 1);
        assert_eq!(gerr.sources.as_ref().unwrap().len(), 1);
    } else {
        panic!("shouvebeen gerr source");
    }
    assert!(iter_gerr_by_tag.next().is_none());

    let mut iter_gerr_by_tag = gerr.iter_by_tag("tag1");
    let iter_by_tag = iter_gerr_by_tag.next().unwrap();
    if let GErrNode::Root(root) = iter_by_tag {
        assert_eq!(root.message(), "id iterator error: 1");
        assert_eq!(root.tags().as_ref().unwrap().len(), 2);
        assert_eq!(root.sources().as_ref().unwrap().len(), 3);
    } else {
        panic!("shouvebeen gerr source");
    }
    assert!(iter_gerr_by_tag.next().is_none());
}

#[test]
fn test_iter_data() {
    let int_err = "wer".parse::<i32>().unwrap_err();
    let gerr = gerr!("id iterator error: {}", 1;
        config=ErrAutoIDCode,
        source=int_err,
        gerr=gerr!("testing: {}", "abc"; config=ErrIDi32, id=123, gerr=gerr!("error"; config=ErrAutoID, code="anu", data = ("user_name", "ajo"))),
        gerr=gerr!("source err: {}", 69; tag="69", gerr=gerr!("asd"; config=ErrAutoID, gerr=gerr!("xcv";config=ErrIDi32, id=42)), data = ["data1", "data2", "data3"]),
        tags=["tag1", "tag2"],
        data = Data { user_id:123, user_name: "qwerty_123".to_string() }
    );

    let mut iter_by_data = gerr.iter_data::<Data>();
    let data = iter_by_data.next().unwrap();
    if let GErrNode::Root(root) = data {
        assert_eq!(root.message(), "id iterator error: 1");
        assert_eq!(root.tags().as_ref().unwrap().len(), 2);
        assert_eq!(root.sources().as_ref().unwrap().len(), 3);
        assert_eq!(root.data().as_ref().unwrap().user_id, 123);
        assert_eq!(root.data().as_ref().unwrap().user_name, "qwerty_123");
    } else {
        panic!("shouvebeen root");
    }
    assert!(iter_by_data.next().is_none());

    let data_q = Data {
        user_id: 20,
        user_name: "qwerty_123".into(),
    };
    let mut iter_by_data = gerr.iter_by_data(&data_q);
    assert!(iter_by_data.next().is_none());

    let data_q = Data {
        user_id: 123,
        user_name: "qwerty_123".into(),
    };
    let mut iter_by_data = gerr.iter_by_data(&data_q);
    let data = iter_by_data.next().unwrap();
    if let GErrNode::Root(root) = data {
        assert_eq!(root.message(), "id iterator error: 1");
        assert_eq!(root.tags().as_ref().unwrap().len(), 2);
        assert_eq!(root.sources().as_ref().unwrap().len(), 3);
        assert_eq!(root.data().as_ref().unwrap().user_id, 123);
        assert_eq!(root.data().as_ref().unwrap().user_name, "qwerty_123");
    } else {
        panic!("shouvebeen root");
    }
    assert!(iter_by_data.next().is_none());

    // query data ---
    let mut iter_by_data = gerr.iter_data::<(&'static str, &'static str)>();
    let data = iter_by_data.next().unwrap();
    if let GErrNode::LeafGErr(gerr) = data {
        assert_eq!(gerr.message, "error");
        assert_eq!(gerr.code.as_ref().unwrap(), "anu");
        assert_eq!(gerr.id.as_ref().unwrap().to_string(), "AutoID");
        if let Some(data) = (&**gerr.data.as_ref().unwrap() as &dyn core::any::Any)
            .downcast_ref::<(&'static str, &'static str)>()
        {
            assert_eq!(data.0, "user_name");
            assert_eq!(data.1, "ajo");
        } else {
            panic!("data should be (&str, &str)");
        };
    } else {
        panic!("shouvebeen root");
    }
    assert!(iter_by_data.next().is_none());

    let mut iter_by_data = gerr.iter_by_data(&("username", "ajo"));
    assert!(iter_by_data.next().is_none());

    let mut iter_by_data = gerr.iter_by_data(&("user_name", "ajo"));
    let data = iter_by_data.next().unwrap();
    if let GErrNode::LeafGErr(gerr) = data {
        assert_eq!(gerr.message, "error");
        assert_eq!(gerr.code.as_ref().unwrap(), "anu");
        assert_eq!(gerr.id.as_ref().unwrap().to_string(), "AutoID");
    } else {
        panic!("shouvebeen root");
    }
    assert!(iter_by_data.next().is_none());
}

#[test]
fn test_iter_source() {
    let int_err = "wer".parse::<i32>().unwrap_err();
    let gerr = gerr!("id iterator error: {}", 1;
        config=ErrAutoIDCode,
        source=int_err,
        gerr=gerr!("testing: {}", "abc"; config=ErrIDi32, id=123, gerr=gerr!("error";config=ErrAutoID, code="anu", data = ("user_name", "ajo"))),
        gerr=gerr!("source err: {}", 69; tag="69", gerr=gerr!("asd"; config=ErrAutoID, gerr=gerr!("xcv"; config=ErrIDi32, id=42)), data = ["data1", "data2", "data3"]),
        tags=["tag1", "tag2"],
        data = Data { user_id:123, user_name: "qwerty_123".to_string() }
    );

    assert_eq!(gerr.iter_source::<ParseIntError>().count(), 1);
    assert_eq!(gerr.iter_source::<GErrSource>().count(), 5);

    if let Some(parse_int_error) = gerr.iter_source::<ParseIntError>().next()
        && let GErrNode::LeafErr(err) = parse_int_error
    {
        assert!(err.is::<ParseIntError>());
        assert_eq!(err.to_string(), "invalid digit found in string");
    }

    for (i, v) in gerr.iter_source::<GErrSource>().enumerate() {
        match i {
            0 => {
                if let GErrNode::LeafGErr(gerr) = v {
                    assert_eq!(gerr.message, "testing: abc");
                    assert_eq!(gerr.id.as_ref().unwrap().to_string(), "123");
                } else {
                    panic!("shouvebeen gerr");
                }
            }
            1 => {
                if let GErrNode::LeafGErr(gerr) = v {
                    assert_eq!(gerr.message, "error");
                    assert_eq!(gerr.code.as_ref().unwrap(), "anu");
                } else {
                    panic!("shouvebeen gerr");
                }
            }
            2 => {
                if let GErrNode::LeafGErr(gerr) = v {
                    assert_eq!(gerr.message, "source err: 69");
                } else {
                    panic!("shouvebeen gerr");
                }
            }
            3 => {
                if let GErrNode::LeafGErr(gerr) = v {
                    assert_eq!(gerr.message, "asd");
                    assert_eq!(gerr.id.as_ref().unwrap().to_string(), "AutoID");
                } else {
                    panic!("shouvebeen gerr");
                }
            }
            4 => {
                if let GErrNode::LeafGErr(gerr) = v {
                    assert_eq!(gerr.message, "xcv");
                    assert_eq!(gerr.id.as_ref().unwrap().to_string(), "42");
                } else {
                    panic!("shouvebeen gerr");
                }
            }
            _ => panic!("there should only 5 gerr sources"),
        }
    }
}
