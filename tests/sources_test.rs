use g_err::*;

#[test]
fn test_general_error() {
    let err = "abc".parse::<u32>().unwrap_err();
    let gerr: GErr = GErr::new("Parse integer error").add_source(err);

    assert_eq!(gerr.sources().unwrap().len(), 1);
    // general error from non-GErr contains no tracked location.
    assert!(matches!(gerr.sources().unwrap()[0], Source::Err(_)));
}

#[test]
fn test_gerr_as_general_error() {
    let err = gerr!("this error");
    let gerr: GErr = GErr::new("error").add_source(err);

    assert_eq!(gerr.sources().unwrap().len(), 1);
    // general error from GErr as general error contains no tracked location.
    assert!(matches!(gerr.sources().unwrap()[0], Source::Err(_)));
}

#[test]
fn test_gerr_as_gerr_source() {
    let line = line!();
    let err = gerr!("this error");
    let gerr: GErr = GErr::new("error").add_source_gerr(err);

    assert_eq!(gerr.sources().unwrap().len(), 1);
    if let Source::GErr(ref ge) = gerr.sources().unwrap()[0] {
        assert_eq!(ge.location.as_ref().unwrap().file, file!());
        assert_eq!(ge.location.as_ref().unwrap().line, line + 1);
    } else {
        panic!("expected source as GErr");
    }
}

#[test]
fn test_gerr_into_source_gerr() {
    let line = line!();
    let err = gerr!("this error");
    let gerr: GErr = GErr::new("error").add_source_gerr(err);

    assert_eq!(gerr.sources().unwrap().len(), 1);
    if let Source::GErr(ref ge) = gerr.sources().unwrap()[0] {
        assert_eq!(ge.location.as_ref().unwrap().file, file!());
        assert_eq!(ge.location.as_ref().unwrap().line, line + 1);
    } else {
        panic!("expected source as GErr");
    }
}

#[test]
fn test_mixed_sources() {
    let int_err = "abc".parse::<u32>().unwrap_err();
    let line = line!();
    let err = gerr!("this error");
    let gerr: GErr = GErr::new("error").add_source_gerr(err).add_source(int_err);

    assert_eq!(gerr.sources().unwrap().len(), 2);
    if let Source::Err(_) = gerr.sources().unwrap()[1] {
    } else {
        panic!("expected source at index 1 as Source::Err")
    }

    if let Source::GErr(ref ge) = gerr.sources().unwrap()[0] {
        assert_eq!(ge.location.as_ref().unwrap().file, file!());
        assert_eq!(ge.location.as_ref().unwrap().line, line + 1);
    } else {
        panic!("expected source at index 0 as Source::GErr")
    }
}

#[test]
fn test_nested_sources() {
    let int_err = "abc".parse::<u32>().unwrap_err();
    let line = line!();
    let err = gerr!(
        "this error";
        gerr = gerr!(
            "cause";
            prefix = "[OUTBOUND]",
            gerr = gerr!("cause's case"; tags = ["tag1", "tag2"])
        ),
        source = int_err.clone(),
        gerr = gerr!("the root cause"; id = 100),
        source = gerr!("the root cause"; id = 120),
        gerr = gerr!("the root cause"; id = 121).into_gerr_source(),
    );

    let gerr: GErr = GErr::new("error").add_source_gerr(err).add_source(int_err);

    let sources = gerr.sources().unwrap();

    assert_eq!(sources.len(), 2);

    // second source is ParseIntError
    match &sources[1] {
        Source::Err(_) => {}
        Source::GErr(_) => panic!("expected general error"),
    }

    // first source is GErr
    let root = match &sources[0] {
        Source::GErr(src) => src,
        Source::Err(_) => panic!("expected GErr source"),
    };

    assert_eq!(root.location.as_ref().unwrap().file, file!());
    assert_eq!(root.location.as_ref().unwrap().line, line + 1);

    let nested = root.sources.as_ref().unwrap();
    assert_eq!(nested.len(), 5);

    // --------------------------------------------------
    // [0] nested outbound gerr
    // --------------------------------------------------
    let outbound = match &nested[0] {
        Source::GErr(src) => src,
        Source::Err(_) => panic!("expected GErr"),
    };

    assert_eq!(outbound.prefix.as_deref(), Some("[OUTBOUND]"));
    assert_eq!(outbound.sources.as_ref().unwrap().len(), 1);

    let cause = match &outbound.sources.as_ref().unwrap()[0] {
        Source::GErr(src) => src,
        Source::Err(_) => panic!("expected GErr"),
    };

    assert_eq!(cause.tags.as_ref().unwrap().len(), 2);

    // --------------------------------------------------
    // [1] ParseIntError
    // --------------------------------------------------
    match &nested[1] {
        Source::Err(_) => {}
        Source::GErr(_) => panic!("expected general error"),
    }

    // --------------------------------------------------
    // [2] gerr!(id = 100)
    // --------------------------------------------------
    let id100 = match &nested[2] {
        Source::GErr(src) => src,
        Source::Err(_) => panic!("expected GErr"),
    };

    assert_eq!(id100.id.to_string(), "100");

    // --------------------------------------------------
    // [3] source = gerr!(...)
    // Wrapped as ordinary Error
    // --------------------------------------------------
    match &nested[3] {
        Source::Err(err) => {
            assert!(err.is::<GErr<i32>>());
        }
        Source::GErr(_) => panic!("expected general error"),
    }

    // --------------------------------------------------
    // [4] into_gerr_source()
    // --------------------------------------------------
    let id121 = match &nested[4] {
        Source::GErr(src) => src,
        Source::Err(_) => panic!("expected GErr"),
    };

    assert_eq!(id121.id.to_string(), "121");
    assert_eq!(id121.location.as_ref().unwrap().file, file!());
}
