pub mod test_gerr_debug_data {
    #[cfg(not(feature = "serde"))]
    pub const EXPECTED_DEBUG: &str = r#"GErr { id: Some("AJO-123"), code: Some("AutoCode"), message: "asd", sources: Some([Err(ParseIntError { kind: InvalidDigit }), GErr(GErrSource { id: Some(123), code: Some("SOURCE-2"), message: "source 2", sources: None, tags: Some(["qwe", "wex"]), data: None, help: None, location: Some(ErrorLocation { file: "tests/error_display_debug_test.rs", line: 20, column: 13 }) })]), tags: Some(["tag1", "tag2"]), data: Some(Data { user_id: 234, user_name: "ajo_sidi" }), help: Some("please halp!!"), location: ErrorLocation { file: "tests/error_display_debug_test.rs", line: 13, column: 46 } }"#;
    #[cfg(feature = "serde")]
    pub const EXPECTED_DEBUG: &str = r#"GErr { id: Some("AJO-123"), code: Some("AutoCode"), message: "asd", sources: Some([Err(ParseIntError { kind: InvalidDigit }), GErr(GErrSource { id: Some(123), id_json: Some(Number(123)), code: Some("SOURCE-2"), message: "source 2", sources: None, tags: Some(["qwe", "wex"]), data: None, data_json: None, help: None, location: Some(ErrorLocation { file: "tests/error_display_debug_test.rs", line: 20, column: 13 }) })]), tags: Some(["tag1", "tag2"]), data: Some(Data { user_id: 234, user_name: "ajo_sidi" }), help: Some("please halp!!"), location: ErrorLocation { file: "tests/error_display_debug_test.rs", line: 13, column: 46 }, backtrace: <disabled> }"#;
    #[cfg(not(feature = "serde"))]
    pub const EXPECTED_DEBUG_FORMAT: &str = r#"GErr {
    id: Some(
        "AJO-123",
    ),
    code: Some(
        "AutoCode",
    ),
    message: "asd",
    sources: Some(
        [
            Err(
                ParseIntError {
                    kind: InvalidDigit,
                },
            ),
            GErr(
                GErrSource {
                    id: Some(
                        123,
                    ),
                    code: Some(
                        "SOURCE-2",
                    ),
                    message: "source 2",
                    sources: None,
                    tags: Some(
                        [
                            "qwe",
                            "wex",
                        ],
                    ),
                    data: None,
                    help: None,
                    location: Some(
                        ErrorLocation {
                            file: "tests/error_display_debug_test.rs",
                            line: 20,
                            column: 13,
                        },
                    ),
                },
            ),
        ],
    ),
    tags: Some(
        [
            "tag1",
            "tag2",
        ],
    ),
    data: Some(
        Data {
            user_id: 234,
            user_name: "ajo_sidi",
        },
    ),
    help: Some(
        "please halp!!",
    ),
    location: ErrorLocation {
        file: "tests/error_display_debug_test.rs",
        line: 13,
        column: 46,
    },
}"#;
    #[cfg(feature = "serde")]
    pub const EXPECTED_DEBUG_FORMAT: &str = r#"GErr {
    id: Some(
        "AJO-123",
    ),
    code: Some(
        "AutoCode",
    ),
    message: "asd",
    sources: Some(
        [
            Err(
                ParseIntError {
                    kind: InvalidDigit,
                },
            ),
            GErr(
                GErrSource {
                    id: Some(
                        123,
                    ),
                    id_json: Some(
                        Number(123),
                    ),
                    code: Some(
                        "SOURCE-2",
                    ),
                    message: "source 2",
                    sources: None,
                    tags: Some(
                        [
                            "qwe",
                            "wex",
                        ],
                    ),
                    data: None,
                    data_json: None,
                    help: None,
                    location: Some(
                        ErrorLocation {
                            file: "tests/error_display_debug_test.rs",
                            line: 20,
                            column: 13,
                        },
                    ),
                },
            ),
        ],
    ),
    tags: Some(
        [
            "tag1",
            "tag2",
        ],
    ),
    data: Some(
        Data {
            user_id: 234,
            user_name: "ajo_sidi",
        },
    ),
    help: Some(
        "please halp!!",
    ),
    location: ErrorLocation {
        file: "tests/error_display_debug_test.rs",
        line: 13,
        column: 46,
    },
    backtrace: <disabled>,
}"#;
}

pub mod test_gerr_source_debug_data {
    #[cfg(not(feature = "serde"))]
    pub const EXPECTED_DEBUG: &str = r#"GErrSource { id: Some("AJO-123"), code: Some("AutoCode"), message: "asd", sources: Some([Err(ParseIntError { kind: InvalidDigit }), GErr(GErrSource { id: Some(123), code: Some("SOURCE-2"), message: "source 2", sources: None, tags: Some(["qwe", "wex"]), data: None, help: None, location: Some(ErrorLocation { file: "tests/error_display_debug_test.rs", line: 82, column: 13 }) })]), tags: Some(["tag1", "tag2"]), data: Some(Data { user_id: 234, user_name: "ajo_sidi" }), help: Some("please halp!!"), location: Some(ErrorLocation { file: "tests/error_display_debug_test.rs", line: 75, column: 16 }) }"#;
    #[cfg(feature = "serde")]
    pub const EXPECTED_DEBUG: &str = r#"GErrSource { id: Some("AJO-123"), id_json: Some(String("AJO-123")), code: Some("AutoCode"), message: "asd", sources: Some([Err(ParseIntError { kind: InvalidDigit }), GErr(GErrSource { id: Some(123), id_json: Some(Number(123)), code: Some("SOURCE-2"), message: "source 2", sources: None, tags: Some(["qwe", "wex"]), data: None, data_json: None, help: None, location: Some(ErrorLocation { file: "tests/error_display_debug_test.rs", line: 82, column: 13 }) })]), tags: Some(["tag1", "tag2"]), data: Some(Data { user_id: 234, user_name: "ajo_sidi" }), data_json: Some(Object {"user_id": Number(234), "user_name": String("ajo_sidi")}), help: Some("please halp!!"), location: Some(ErrorLocation { file: "tests/error_display_debug_test.rs", line: 75, column: 16 }) }"#;
    #[cfg(not(feature = "serde"))]
    pub const EXPECTED_DEBUG_FORMAT: &str = r#"GErrSource {
    id: Some(
        "AJO-123",
    ),
    code: Some(
        "AutoCode",
    ),
    message: "asd",
    sources: Some(
        [
            Err(
                ParseIntError {
                    kind: InvalidDigit,
                },
            ),
            GErr(
                GErrSource {
                    id: Some(
                        123,
                    ),
                    code: Some(
                        "SOURCE-2",
                    ),
                    message: "source 2",
                    sources: None,
                    tags: Some(
                        [
                            "qwe",
                            "wex",
                        ],
                    ),
                    data: None,
                    help: None,
                    location: Some(
                        ErrorLocation {
                            file: "tests/error_display_debug_test.rs",
                            line: 82,
                            column: 13,
                        },
                    ),
                },
            ),
        ],
    ),
    tags: Some(
        [
            "tag1",
            "tag2",
        ],
    ),
    data: Some(
        Data {
            user_id: 234,
            user_name: "ajo_sidi",
        },
    ),
    help: Some(
        "please halp!!",
    ),
    location: Some(
        ErrorLocation {
            file: "tests/error_display_debug_test.rs",
            line: 75,
            column: 16,
        },
    ),
}"#;
    #[cfg(feature = "serde")]
    pub const EXPECTED_DEBUG_FORMAT: &str = r#"GErrSource {
    id: Some(
        "AJO-123",
    ),
    id_json: Some(
        String("AJO-123"),
    ),
    code: Some(
        "AutoCode",
    ),
    message: "asd",
    sources: Some(
        [
            Err(
                ParseIntError {
                    kind: InvalidDigit,
                },
            ),
            GErr(
                GErrSource {
                    id: Some(
                        123,
                    ),
                    id_json: Some(
                        Number(123),
                    ),
                    code: Some(
                        "SOURCE-2",
                    ),
                    message: "source 2",
                    sources: None,
                    tags: Some(
                        [
                            "qwe",
                            "wex",
                        ],
                    ),
                    data: None,
                    data_json: None,
                    help: None,
                    location: Some(
                        ErrorLocation {
                            file: "tests/error_display_debug_test.rs",
                            line: 82,
                            column: 13,
                        },
                    ),
                },
            ),
        ],
    ),
    tags: Some(
        [
            "tag1",
            "tag2",
        ],
    ),
    data: Some(
        Data {
            user_id: 234,
            user_name: "ajo_sidi",
        },
    ),
    data_json: Some(
        Object {
            "user_id": Number(234),
            "user_name": String("ajo_sidi"),
        },
    ),
    help: Some(
        "please halp!!",
    ),
    location: Some(
        ErrorLocation {
            file: "tests/error_display_debug_test.rs",
            line: 75,
            column: 16,
        },
    ),
}"#;
}
