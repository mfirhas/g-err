use g_err::*;

#[test]
fn test_general_error() {
    let err = "abc".parse::<u32>().unwrap_err();
    let gerr: GErr = GErr::new("Parse integer error").add_source(err);

    assert_eq!(gerr.sources().unwrap().len(), 1);
    // general error from non-GErr contains no tracked location.
    assert_eq!(gerr.sources().unwrap()[0].location, None);
}

#[test]
fn test_gerr_as_general_error() {
    let err = gerr!("this error");
    let gerr: GErr = GErr::new("error").add_source(err);

    assert_eq!(gerr.sources().unwrap().len(), 1);
    // general error from GErr as general error contains no tracked location.
    assert_eq!(gerr.sources().unwrap()[0].location, None);
}

#[test]
fn test_gerr_as_gerr_source() {
    let line = line!();
    let err = gerr!("this error");
    let gerr: GErr = GErr::new("error").add_source(err.into_gerr_source());

    assert_eq!(gerr.sources().unwrap().len(), 1);
    // general error from GErr as gerr source contains everything.
    assert_eq!(gerr.sources().unwrap()[0].location.unwrap().file(), file!());
    assert_eq!(
        gerr.sources().unwrap()[0].location.unwrap().line(),
        line + 1
    );
}

#[test]
fn test_gerr_into_source_gerr() {
    let line = line!();
    let err = gerr!("this error");
    let gerr: GErr = GErr::new("error").add_source_gerr(err);

    assert_eq!(gerr.sources().unwrap().len(), 1);
    // general error from GErr as gerr source contains everything.
    assert_eq!(gerr.sources().unwrap()[0].location.unwrap().file(), file!());
    assert_eq!(
        gerr.sources().unwrap()[0].location.unwrap().line(),
        line + 1
    );
}

#[test]
fn test_mixed_sources() {
    let int_err = "abc".parse::<u32>().unwrap_err();
    let line = line!();
    let err = gerr!("this error");
    let gerr: GErr = GErr::new("error").add_source_gerr(err).add_source(int_err);

    assert_eq!(gerr.sources().unwrap().len(), 2);
    assert_eq!(gerr.sources().unwrap()[1].location, None);
    assert_eq!(gerr.sources().unwrap()[0].location.unwrap().file(), file!());
    assert_eq!(
        gerr.sources().unwrap()[0].location.unwrap().line(),
        line + 1
    );
}

#[test]
fn test_nested_sources() {
    let int_err = "abc".parse::<u32>().unwrap_err();
    let line = line!();
    let err = gerr!(
        "this error";
        gerr = gerr!("cause"; prefix = "[OUTBOUND]", gerr = gerr!("cause's case"; tags = ["tag1", "tag2"])),
        source = int_err.clone(),
        gerr = gerr!("the root cause"; id = 100),
        source = gerr!("the root cause"; id = 120),
        source = gerr!("the root cause"; id = 121).into_gerr_source(),
    );
    let gerr: GErr = GErr::new("error").add_source_gerr(err).add_source(int_err);

    assert_eq!(gerr.sources().unwrap().len(), 2);
    assert_eq!(gerr.sources().unwrap()[1].location, None);
    assert_eq!(
        gerr.sources().unwrap()[0].sources.as_ref().unwrap().len(),
        5
    );
    // attributes check
    assert_eq!(
        gerr.sources().unwrap()[0].sources.as_ref().unwrap()[0]
            .sources
            .as_ref()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        gerr.sources().unwrap()[0].sources.as_ref().unwrap()[0]
            .prefix
            .as_ref()
            .unwrap(),
        "[OUTBOUND]"
    );
    assert_eq!(
        gerr.sources().unwrap()[0].sources.as_ref().unwrap()[0]
            .sources
            .as_ref()
            .unwrap()[0]
            .tags // tag1 and tag2
            .as_ref()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        gerr.sources().unwrap()[0].sources.as_ref().unwrap()[1].location,
        None
    );
    assert_eq!(
        gerr.sources().unwrap()[0].sources.as_ref().unwrap()[2]
            .id
            .to_string(),
        "100"
    );
    assert_eq!(
        gerr.sources().unwrap()[0].sources.as_ref().unwrap()[3]
            .id
            .to_string(),
        "NoID" // because gerr wrapped as general error
    );
    assert_eq!(
        gerr.sources().unwrap()[0].sources.as_ref().unwrap()[3]
            .location
            .as_ref(),
        None
    );
    assert_eq!(
        gerr.sources().unwrap()[0].sources.as_ref().unwrap()[4]
            .id
            .to_string(),
        "121"
    );
    assert_eq!(
        gerr.sources().unwrap()[0].sources.as_ref().unwrap()[4]
            .location
            .as_ref()
            .unwrap()
            .file(),
        file!()
    );

    assert_eq!(gerr.sources().unwrap()[0].location.unwrap().file(), file!());
    assert_eq!(
        gerr.sources().unwrap()[0].location.unwrap().line(),
        line + 1
    );
}
