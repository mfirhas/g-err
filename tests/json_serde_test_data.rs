pub mod test_json_serde_data_data {
    #[cfg(feature = "serde")]
    pub const EXPECTED_JSON_DATA_DEBUG: &str = r#"{
  "id": "AutoID",
  "code": "AutoCode",
  "message": "pretty error: l2k3mr2l3r",
  "tags": [
    "tag1",
    "tag2",
    "tag3"
  ],
  "data": {
    "user_id": 123,
    "user_name": "ajo"
  },
  "location": {
    "file": "tests/json_serde_test.rs",
    "line": 20,
    "column": 43
  },
  "sources": [
    {
      "id": null,
      "code": null,
      "message": "invalid digit found in string",
      "tags": null,
      "data": null,
      "location": null,
      "sources": null,
      "help": null
    },
    {
      "id": 40,
      "code": "400",
      "message": "input is invalid: qwe",
      "tags": [
        "bad_request",
        "invalid_input"
      ],
      "data": [
        "user_name",
        "ajo"
      ],
      "location": {
        "file": "tests/json_serde_test.rs",
        "line": 30,
        "column": 14
      },
      "sources": [
        {
          "id": null,
          "code": null,
          "message": "invalid digit found in string",
          "tags": null,
          "data": null,
          "location": null,
          "sources": null,
          "help": null
        },
        {
          "id": null,
          "code": "[OUTBOUND]",
          "message": "upstream error",
          "tags": null,
          "data": null,
          "location": {
            "file": "tests/json_serde_test.rs",
            "line": 39,
            "column": 18
          },
          "sources": [
            {
              "id": null,
              "code": null,
              "message": "got error from user service",
              "tags": null,
              "data": [
                "caused by:",
                "timout"
              ],
              "location": {
                "file": "tests/json_serde_test.rs",
                "line": 39,
                "column": 66
              },
              "sources": null,
              "help": "contact user service steward"
            }
          ],
          "help": null
        }
      ],
      "help": "pass valid input"
    },
    {
      "id": "AutoID",
      "code": null,
      "message": "timeout checks",
      "tags": [
        "user_service",
        "timeout"
      ],
      "data": null,
      "location": {
        "file": "tests/json_serde_test.rs",
        "line": 41,
        "column": 14
      },
      "sources": [
        {
          "id": null,
          "code": null,
          "message": "too many open files",
          "tags": [
            "tmof"
          ],
          "data": [
            "MAX",
            50000
          ],
          "location": {
            "file": "tests/json_serde_test.rs",
            "line": 44,
            "column": 18
          },
          "sources": null,
          "help": null
        }
      ],
      "help": null
    },
    {
      "id": "AutoID",
      "code": null,
      "message": "connection timeout",
      "tags": null,
      "data": null,
      "location": {
        "file": "tests/json_serde_test.rs",
        "line": 45,
        "column": 14
      },
      "sources": null,
      "help": null
    }
  ],
  "help": "send valid request",
  "backtrace": "<disabled>"
}"#;
}

pub mod test_json_serialize_data_data {
    #[cfg(feature = "serde")]
    pub const EXPECTED_DISPLAY_DATA_JSON: &str = r#"{
  "gerr_public": {
    "id": "AutoID",
    "code": "AutoCode",
    "message": "pretty error: l2k3mr2l3r",
    "tags": [
      "tag1",
      "tag2",
      "tag3"
    ],
    "data": {
      "user_id": 123,
      "user_name": "ajo"
    },
    "help": "send valid request",
    "causes": [
      {
        "message": "invalid digit found in string",
        "caused_by": null
      },
      {
        "message": "input is invalid: qwe",
        "caused_by": [
          {
            "message": "invalid digit found in string",
            "caused_by": null
          },
          {
            "message": "upstream error",
            "caused_by": [
              {
                "message": "got error from user service",
                "caused_by": null
              }
            ]
          }
        ]
      },
      {
        "message": "timeout checks",
        "caused_by": [
          {
            "message": "too many open files",
            "caused_by": null
          }
        ]
      },
      {
        "message": "connection timeout",
        "caused_by": null
      }
    ]
  }
}"#;
}

pub mod test_json_serde_data_failed_deser_json_data_data {
    #[cfg(feature = "serde")]
    pub const WRONG_JSON_DATA: &str = r#"{
  "id": 123,
  "code": "AutoCode",
  "message": "pretty error: l2k3mr2l3r",
  "tags": [
    "tag1",
    "tag2",
    "tag3"
  ],
  "data": {
    "user_id": 123,
    "user_name": "ajo"
  },
  "location": {
    "file": "tests/json_serde_test.rs",
    "line": 861,
    "column": 43
  },
  "sources": [
    {
      "id": null,
      "code": null,
      "message": "invalid digit found in string",
      "tags": null,
      "data": null,
      "location": null,
      "sources": null,
      "help": null
    },
    {
      "id": 40,
      "code": "400",
      "message": "input is invalid: qwe",
      "tags": [
        "bad_request",
        "invalid_input"
      ],
      "data": [
        "user_name",
        "ajo"
      ],
      "location": {
        "file": "tests/json_serde_test.rs",
        "line": 871,
        "column": 14
      },
      "sources": [
        {
          "id": null,
          "code": null,
          "message": "invalid digit found in string",
          "tags": null,
          "data": null,
          "location": null,
          "sources": null,
          "help": null
        },
        {
          "id": null,
          "code": "[OUTBOUND]",
          "message": "upstream error",
          "tags": null,
          "data": null,
          "location": {
            "file": "tests/json_serde_test.rs",
            "line": 880,
            "column": 18
          },
          "sources": [
            {
              "id": null,
              "code": null,
              "message": "got error from user service",
              "tags": null,
              "data": [
                "caused by:",
                "timout"
              ],
              "location": {
                "file": "tests/json_serde_test.rs",
                "line": 880,
                "column": 66
              },
              "sources": null,
              "help": "contact user service steward"
            }
          ],
          "help": null
        }
      ],
      "help": "pass valid input"
    },
    {
      "id": "AutoID",
      "code": null,
      "message": "timeout checks",
      "tags": [
        "user_service",
        "timeout"
      ],
      "data": null,
      "location": {
        "file": "tests/json_serde_test.rs",
        "line": 882,
        "column": 14
      },
      "sources": [
        {
          "id": null,
          "code": null,
          "message": "too many open files",
          "tags": [
            "tmof"
          ],
          "data": [
            "MAX",
            50000
          ],
          "location": {
            "file": "tests/json_serde_test.rs",
            "line": 885,
            "column": 18
          },
          "sources": null,
          "help": null
        }
      ],
      "help": null
    },
    {
      "id": null,
      "code": null,
      "message": "connection timeout",
      "tags": null,
      "data": null,
      "location": {
        "file": "tests/json_serde_test.rs",
        "line": 886,
        "column": 14
      },
      "sources": null,
      "help": null
    }
  ],
  "help": 123,
  "backtrace": "<disabled>"
}"#;
}
