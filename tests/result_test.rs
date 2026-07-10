use g_err::*;

#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

fn parse_age(str_age: &str) -> Result<i32, u64, NoPrefix, Data> {
    str_age
        .parse()
        .context(4235, "failed parsing age")
        .map_err(|gerr| {
            gerr.set_prefix("[REPO]").set_data(Data {
                user_id: 123,
                user_name: "ajo".into(),
            })
        })
}

fn get_age(str_age: &str) -> Result<i32, AutoID, AutoPrefix, (String, String)> {
    let age = parse_age(str_age)
        .gerr_auto("failed getting age")
        .map_err(|gerr| gerr.set_data(("age".into(), str_age.into())))?;
    Ok(age)
}

fn parse_email(email: &str) -> Result<String, AutoID, NoPrefix, NoData> {
    if !email.contains("@") {
        return gerr!("invalid email: {}", email; id_auto=AutoID, prefix="[REPO]")
            .result()
            .context_auto("failed parsing email");
    }
    Ok(email.into())
}

fn get_email(email: &str) -> Result<String, NoID, AutoPrefix, Data> {
    parse_email(email).wrap_err(gerr!("failed getting email: {}", email; prefix_auto, data=Data { user_id: 123, user_name: "ajo".into() }))
}

fn use_case(
    email: &str,
    age: &str,
) -> core::result::Result<(String, i32), GErr<&'static str, AutoPrefix, NoData>> {
    let age = get_age(age);

    let email = get_email(email);

    let ret = match (age, email) {
        (Ok(a), Ok(e)) => Ok((e, a)),
        (Ok(_), Err(ee)) => Err(ee),
        (Err(ae), Ok(_)) => Err(ae.with_id(NoID).with_data(Data {
            user_id: 34,
            user_name: "asd".into(),
        })),
        (Err(ae), Err(ee)) => Err(GErr::<NoID, AutoPrefix, Data>::new(
            "failed both get age and email",
        )
        .add_source_gerr(ae)
        .add_source_gerr(ee)),
    };

    ret.gerr("E-001", "failed getting age or email")
}

#[rustfmt::skip]
fn handler(email: &str, age: &str) -> core::result::Result<String, GErrBox<AutoID, AutoPrefix, NoData>> {
    let ret = use_case(email, age).wrap_gerr(gerr!("handler get email and age error";
        id_auto=AutoID,
        prefix_auto=AutoPrefix,
        tags=["http", "handler", "email", "age"],
        help="please pass valid email and age",
    )).boxed()?;

    Ok(format!("email: {} and age: {}", ret.0, ret.1))
}

#[cfg(not(feature = "serde"))]
const EXPECTED_DEBUG: &str = r#"GErr {
    id: AutoID,
    prefix: Some(
        "AutoPrefix",
    ),
    message: "handler get email and age error",
    sources: Some(
        [
            GErr(
                GErrSource {
                    id: "E-001",
                    prefix: Some(
                        "AutoPrefix",
                    ),
                    message: "failed getting age or email",
                    sources: Some(
                        [
                            GErr(
                                GErrSource {
                                    id: NoID,
                                    prefix: Some(
                                        "AutoPrefix",
                                    ),
                                    message: "failed both get age and email",
                                    sources: Some(
                                        [
                                            GErr(
                                                GErrSource {
                                                    id: AutoID,
                                                    prefix: Some(
                                                        "AutoPrefix",
                                                    ),
                                                    message: "failed getting age",
                                                    sources: Some(
                                                        [
                                                            GErr(
                                                                GErrSource {
                                                                    id: 4235,
                                                                    prefix: Some(
                                                                        "[REPO]",
                                                                    ),
                                                                    message: "failed parsing age",
                                                                    sources: Some(
                                                                        [
                                                                            Err(
                                                                                ParseIntError {
                                                                                    kind: InvalidDigit,
                                                                                },
                                                                            ),
                                                                        ],
                                                                    ),
                                                                    tags: None,
                                                                    data: Some(
                                                                        Data {
                                                                            user_id: 123,
                                                                            user_name: "ajo",
                                                                        },
                                                                    ),
                                                                    help: None,
                                                                    location: Some(
                                                                        ErrorLocation {
                                                                            file: "tests/result_test.rs",
                                                                            line: 10,
                                                                            column: 10,
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    ),
                                                    tags: None,
                                                    data: Some(
                                                        (
                                                            "age",
                                                            "52!",
                                                        ),
                                                    ),
                                                    help: None,
                                                    location: Some(
                                                        ErrorLocation {
                                                            file: "tests/result_test.rs",
                                                            line: 21,
                                                            column: 10,
                                                        },
                                                    ),
                                                },
                                            ),
                                            GErr(
                                                GErrSource {
                                                    id: NoID,
                                                    prefix: Some(
                                                        "AutoPrefix",
                                                    ),
                                                    message: "failed getting email: dono.the_dono_gmail.com",
                                                    sources: Some(
                                                        [
                                                            Err(
                                                                GErr {
                                                                    id: AutoID,
                                                                    prefix: None,
                                                                    message: "failed parsing email",
                                                                    sources: Some(
                                                                        [
                                                                            Err(
                                                                                GErr {
                                                                                    id: AutoID,
                                                                                    prefix: Some(
                                                                                        "[REPO]",
                                                                                    ),
                                                                                    message: "invalid email: dono.the_dono_gmail.com",
                                                                                    sources: None,
                                                                                    tags: None,
                                                                                    data: None,
                                                                                    help: None,
                                                                                    location: ErrorLocation {
                                                                                        file: "tests/result_test.rs",
                                                                                        line: 28,
                                                                                        column: 16,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                    ),
                                                                    tags: None,
                                                                    data: None,
                                                                    help: None,
                                                                    location: ErrorLocation {
                                                                        file: "tests/result_test.rs",
                                                                        line: 30,
                                                                        column: 14,
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    ),
                                                    tags: None,
                                                    data: Some(
                                                        Data {
                                                            user_id: 123,
                                                            user_name: "ajo",
                                                        },
                                                    ),
                                                    help: None,
                                                    location: Some(
                                                        ErrorLocation {
                                                            file: "tests/result_test.rs",
                                                            line: 36,
                                                            column: 33,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ],
                                    ),
                                    tags: None,
                                    data: None,
                                    help: None,
                                    location: Some(
                                        ErrorLocation {
                                            file: "tests/result_test.rs",
                                            line: 54,
                                            column: 35,
                                        },
                                    ),
                                },
                            ),
                        ],
                    ),
                    tags: None,
                    data: None,
                    help: None,
                    location: Some(
                        ErrorLocation {
                            file: "tests/result_test.rs",
                            line: 61,
                            column: 9,
                        },
                    ),
                },
            ),
        ],
    ),
    tags: Some(
        [
            "http",
            "handler",
            "email",
            "age",
        ],
    ),
    data: None,
    help: Some(
        "please pass valid email and age",
    ),
    location: ErrorLocation {
        file: "tests/result_test.rs",
        line: 66,
        column: 46,
    },
}"#;

#[cfg(feature = "serde")]
const EXPECTED_DEBUG: &str = r#"GErr {
    id: AutoID,
    prefix: Some(
        "AutoPrefix",
    ),
    message: "handler get email and age error",
    sources: Some(
        [
            GErr(
                GErrSource {
                    id: "E-001",
                    id_json: String("E-001"),
                    prefix: Some(
                        "AutoPrefix",
                    ),
                    message: "failed getting age or email",
                    sources: Some(
                        [
                            GErr(
                                GErrSource {
                                    id: NoID,
                                    id_json: Null,
                                    prefix: Some(
                                        "AutoPrefix",
                                    ),
                                    message: "failed both get age and email",
                                    sources: Some(
                                        [
                                            GErr(
                                                GErrSource {
                                                    id: AutoID,
                                                    id_json: String("AutoID"),
                                                    prefix: Some(
                                                        "AutoPrefix",
                                                    ),
                                                    message: "failed getting age",
                                                    sources: Some(
                                                        [
                                                            GErr(
                                                                GErrSource {
                                                                    id: 4235,
                                                                    id_json: Number(4235),
                                                                    prefix: Some(
                                                                        "[REPO]",
                                                                    ),
                                                                    message: "failed parsing age",
                                                                    sources: Some(
                                                                        [
                                                                            Err(
                                                                                ParseIntError {
                                                                                    kind: InvalidDigit,
                                                                                },
                                                                            ),
                                                                        ],
                                                                    ),
                                                                    tags: None,
                                                                    data: Some(
                                                                        Data {
                                                                            user_id: 123,
                                                                            user_name: "ajo",
                                                                        },
                                                                    ),
                                                                    data_json: Some(
                                                                        Object {
                                                                            "user_id": Number(123),
                                                                            "user_name": String("ajo"),
                                                                        },
                                                                    ),
                                                                    help: None,
                                                                    location: Some(
                                                                        ErrorLocation {
                                                                            file: "tests/result_test.rs",
                                                                            line: 10,
                                                                            column: 10,
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    ),
                                                    tags: None,
                                                    data: Some(
                                                        (
                                                            "age",
                                                            "52!",
                                                        ),
                                                    ),
                                                    data_json: Some(
                                                        Array [
                                                            String("age"),
                                                            String("52!"),
                                                        ],
                                                    ),
                                                    help: None,
                                                    location: Some(
                                                        ErrorLocation {
                                                            file: "tests/result_test.rs",
                                                            line: 21,
                                                            column: 10,
                                                        },
                                                    ),
                                                },
                                            ),
                                            GErr(
                                                GErrSource {
                                                    id: NoID,
                                                    id_json: Null,
                                                    prefix: Some(
                                                        "AutoPrefix",
                                                    ),
                                                    message: "failed getting email: dono.the_dono_gmail.com",
                                                    sources: Some(
                                                        [
                                                            Err(
                                                                GErr {
                                                                    id: AutoID,
                                                                    prefix: None,
                                                                    message: "failed parsing email",
                                                                    sources: Some(
                                                                        [
                                                                            Err(
                                                                                GErr {
                                                                                    id: AutoID,
                                                                                    prefix: Some(
                                                                                        "[REPO]",
                                                                                    ),
                                                                                    message: "invalid email: dono.the_dono_gmail.com",
                                                                                    sources: None,
                                                                                    tags: None,
                                                                                    data: None,
                                                                                    help: None,
                                                                                    location: ErrorLocation {
                                                                                        file: "tests/result_test.rs",
                                                                                        line: 28,
                                                                                        column: 16,
                                                                                    },
                                                                                    backtrace: <disabled>,
                                                                                },
                                                                            ),
                                                                        ],
                                                                    ),
                                                                    tags: None,
                                                                    data: None,
                                                                    help: None,
                                                                    location: ErrorLocation {
                                                                        file: "tests/result_test.rs",
                                                                        line: 30,
                                                                        column: 14,
                                                                    },
                                                                    backtrace: <disabled>,
                                                                },
                                                            ),
                                                        ],
                                                    ),
                                                    tags: None,
                                                    data: Some(
                                                        Data {
                                                            user_id: 123,
                                                            user_name: "ajo",
                                                        },
                                                    ),
                                                    data_json: Some(
                                                        Object {
                                                            "user_id": Number(123),
                                                            "user_name": String("ajo"),
                                                        },
                                                    ),
                                                    help: None,
                                                    location: Some(
                                                        ErrorLocation {
                                                            file: "tests/result_test.rs",
                                                            line: 36,
                                                            column: 33,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ],
                                    ),
                                    tags: None,
                                    data: None,
                                    data_json: None,
                                    help: None,
                                    location: Some(
                                        ErrorLocation {
                                            file: "tests/result_test.rs",
                                            line: 54,
                                            column: 35,
                                        },
                                    ),
                                },
                            ),
                        ],
                    ),
                    tags: None,
                    data: None,
                    data_json: None,
                    help: None,
                    location: Some(
                        ErrorLocation {
                            file: "tests/result_test.rs",
                            line: 61,
                            column: 9,
                        },
                    ),
                },
            ),
        ],
    ),
    tags: Some(
        [
            "http",
            "handler",
            "email",
            "age",
        ],
    ),
    data: None,
    help: Some(
        "please pass valid email and age",
    ),
    location: ErrorLocation {
        file: "tests/result_test.rs",
        line: 66,
        column: 46,
    },
    backtrace: <disabled>,
}"#;

#[test]
fn test_result_ext() {
    let email = "dono.the_dono_gmail.com";
    let age = "52!";
    let ret = handler(email, age).unwrap_err();

    dbg!(&ret);
    assert_eq!(ret.id(), &AutoID);
    assert_eq!(ret.prefix().unwrap(), "AutoPrefix");
    let debug = format!("{:#?}", ret);
    assert_eq!(debug, EXPECTED_DEBUG);
}
