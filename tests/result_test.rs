use g_err::*;
#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

fn parse_age(str_age: &str) -> Result<i32, ErrIDi32, Data> {
    str_age
        .parse()
        .context(4235, "failed parsing age")
        .map_err(|gerr| {
            gerr.set_code("[REPO]").set_data(Data {
                user_id: 123,
                user_name: "ajo".into(),
            })
        })
}

fn get_age(str_age: &str) -> Result<i32, ErrAutoIDCode, (String, String)> {
    let age = parse_age(str_age)
        .gerr_auto("failed getting age")
        .map_err(|gerr| gerr.set_data(("age".into(), str_age.into())))?;
    Ok(age)
}

fn parse_email(email: &str) -> Result<String, ErrAutoID, NoData> {
    if !email.contains("@") {
        return gerr!("invalid email: {}", email; config=ErrAutoID, code="[REPO]")
            .result()
            .context_auto("failed parsing email");
    }
    Ok(email.into())
}

fn get_email(email: &str) -> Result<String, ErrAutoCode, Data> {
    parse_email(email).wrap_err(gerr!("failed getting email: {}", email; config=ErrAutoCode, data=Data { user_id: 123, user_name: "ajo".into() }))
}

fn use_case(
    email: &str,
    age: &str,
) -> core::result::Result<(String, i32), GErr<ErrIDStrAutoCode, NoData>> {
    let age = get_age(age);

    let email = get_email(email);

    let ret = match (age, email) {
        (Ok(a), Ok(e)) => Ok((e, a)),
        (Ok(_), Err(ee)) => Err(ee),
        (Err(ae), Ok(_)) => Err(ae.with_config::<ErrAutoCode>().with_data(Data {
            user_id: 34,
            user_name: "asd".into(),
        })),
        (Err(ae), Err(ee)) => Err(
            GErr::<ErrAutoCode, Data>::new("failed both get age and email")
                .add_source_gerr(ae)
                .add_source_gerr(ee),
        ),
    };

    ret.gerr("E-001", "failed getting age or email")
}

#[rustfmt::skip]
fn handler(email: &str, age: &str) -> core::result::Result<String, GErrBox<ErrAutoIDCode, NoData>> {
    let ret = use_case(email, age).wrap_gerr(gerr!("handler get email and age error";
        config,
        tags=["http", "handler", "email", "age"],
        help="please pass valid email and age",
    )).boxed()?;

    Ok(format!("email: {} and age: {}", ret.0, ret.1))
}

#[cfg(not(feature = "serde"))]
const EXPECTED_DEBUG: &str = r#"GErr {
    id: Some(
        AutoID,
    ),
    code: Some(
        "AutoCode",
    ),
    message: "handler get email and age error",
    sources: Some(
        [
            GErr(
                GErrSource {
                    id: Some(
                        "E-001",
                    ),
                    code: Some(
                        "AutoCode",
                    ),
                    message: "failed getting age or email",
                    sources: Some(
                        [
                            GErr(
                                GErrSource {
                                    id: None,
                                    code: Some(
                                        "AutoCode",
                                    ),
                                    message: "failed both get age and email",
                                    sources: Some(
                                        [
                                            GErr(
                                                GErrSource {
                                                    id: Some(
                                                        AutoID,
                                                    ),
                                                    code: Some(
                                                        "AutoCode",
                                                    ),
                                                    message: "failed getting age",
                                                    sources: Some(
                                                        [
                                                            GErr(
                                                                GErrSource {
                                                                    id: Some(
                                                                        4235,
                                                                    ),
                                                                    code: Some(
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
                                                                            line: 9,
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
                                                            line: 20,
                                                            column: 10,
                                                        },
                                                    ),
                                                },
                                            ),
                                            GErr(
                                                GErrSource {
                                                    id: None,
                                                    code: Some(
                                                        "AutoCode",
                                                    ),
                                                    message: "failed getting email: dono.the_dono_gmail.com",
                                                    sources: Some(
                                                        [
                                                            Err(
                                                                GErr {
                                                                    id: Some(
                                                                        AutoID,
                                                                    ),
                                                                    code: None,
                                                                    message: "failed parsing email",
                                                                    sources: Some(
                                                                        [
                                                                            Err(
                                                                                GErr {
                                                                                    id: Some(
                                                                                        AutoID,
                                                                                    ),
                                                                                    code: Some(
                                                                                        "[REPO]",
                                                                                    ),
                                                                                    message: "invalid email: dono.the_dono_gmail.com",
                                                                                    sources: None,
                                                                                    tags: None,
                                                                                    data: None,
                                                                                    help: None,
                                                                                    location: ErrorLocation {
                                                                                        file: "tests/result_test.rs",
                                                                                        line: 27,
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
                                                                        line: 29,
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
                                                            line: 35,
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
                                            column: 13,
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
                            line: 60,
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
        line: 65,
        column: 46,
    },
}"#;

#[cfg(feature = "serde")]
const EXPECTED_DEBUG: &str = r#"GErr {
    id: Some(
        AutoID,
    ),
    code: Some(
        "AutoCode",
    ),
    message: "handler get email and age error",
    sources: Some(
        [
            GErr(
                GErrSource {
                    id: Some(
                        "E-001",
                    ),
                    id_json: Some(
                        String("E-001"),
                    ),
                    code: Some(
                        "AutoCode",
                    ),
                    message: "failed getting age or email",
                    sources: Some(
                        [
                            GErr(
                                GErrSource {
                                    id: None,
                                    id_json: None,
                                    code: Some(
                                        "AutoCode",
                                    ),
                                    message: "failed both get age and email",
                                    sources: Some(
                                        [
                                            GErr(
                                                GErrSource {
                                                    id: Some(
                                                        AutoID,
                                                    ),
                                                    id_json: Some(
                                                        String("AutoID"),
                                                    ),
                                                    code: Some(
                                                        "AutoCode",
                                                    ),
                                                    message: "failed getting age",
                                                    sources: Some(
                                                        [
                                                            GErr(
                                                                GErrSource {
                                                                    id: Some(
                                                                        4235,
                                                                    ),
                                                                    id_json: Some(
                                                                        Number(4235),
                                                                    ),
                                                                    code: Some(
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
                                                                            line: 9,
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
                                                            line: 20,
                                                            column: 10,
                                                        },
                                                    ),
                                                },
                                            ),
                                            GErr(
                                                GErrSource {
                                                    id: None,
                                                    id_json: None,
                                                    code: Some(
                                                        "AutoCode",
                                                    ),
                                                    message: "failed getting email: dono.the_dono_gmail.com",
                                                    sources: Some(
                                                        [
                                                            Err(
                                                                GErr {
                                                                    id: Some(
                                                                        AutoID,
                                                                    ),
                                                                    code: None,
                                                                    message: "failed parsing email",
                                                                    sources: Some(
                                                                        [
                                                                            Err(
                                                                                GErr {
                                                                                    id: Some(
                                                                                        AutoID,
                                                                                    ),
                                                                                    code: Some(
                                                                                        "[REPO]",
                                                                                    ),
                                                                                    message: "invalid email: dono.the_dono_gmail.com",
                                                                                    sources: None,
                                                                                    tags: None,
                                                                                    data: None,
                                                                                    help: None,
                                                                                    location: ErrorLocation {
                                                                                        file: "tests/result_test.rs",
                                                                                        line: 27,
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
                                                                        line: 29,
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
                                                            line: 35,
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
                                            column: 13,
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
                            line: 60,
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
        line: 65,
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
    assert_eq!(ret.id().unwrap(), &AutoID);
    assert_eq!(ret.code().unwrap(), "AutoCode");
    let debug = format!("{:#?}", ret);
    println!("==>{}", debug);
    assert_eq!(debug, EXPECTED_DEBUG);
}
