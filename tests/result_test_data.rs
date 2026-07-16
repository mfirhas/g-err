pub mod test_result_ext_data {
    #[cfg(not(feature = "serde"))]
    pub const EXPECTED_DEBUG: &str = r#"GErr {
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
                                                                            line: 12,
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
                                                            line: 23,
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
                                                                                        line: 30,
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
                                                                        line: 32,
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
                                                            line: 38,
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
                                            line: 57,
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
                            line: 63,
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
        line: 68,
        column: 46,
    },
}"#;

    #[cfg(feature = "serde")]
    pub const EXPECTED_DEBUG: &str = r#"GErr {
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
                                                                            line: 12,
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
                                                            line: 23,
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
                                                                                        line: 30,
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
                                                                        line: 32,
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
                                                            line: 38,
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
                                            line: 57,
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
                            line: 63,
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
        line: 68,
        column: 46,
    },
    backtrace: <disabled>,
}"#;
}
