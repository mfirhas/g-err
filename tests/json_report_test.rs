#[cfg(feature = "serde")]
#[path = "setup_test.rs"]
mod setup_test;
#[cfg(feature = "serde")]
use setup_test::*;

#[cfg(feature = "serde")]
use g_err::*;

#[cfg(feature = "serde")]
use g_err::json::JsonReport;

#[cfg(feature = "serde")]
#[test]
fn test_json_report() {
    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoIDCode, Data> = gerr!("pretty error: {req_id}";
        config,
        tag="tag1",
        tags=["tag2", "tag3"],
        data= Data {
            user_id,
            user_name,
        },
        help="send valid request",
        source=input_err.clone(),
        gerr=gerr!("input is invalid: {}", input;
            config=ErrIDi32,
            id=40,
            code="400",
            tag="bad_request",
            tag="invalid_input",
            help="pass valid input",
            data=("user_name".to_string(), "ajo".to_string()),
            source = input_err,
            gerr=gerr!("upstream error"; code="[OUTBOUND]", gerr=gerr!("got error from user service"; data=("caused by:".to_string(), "timout".to_string()), help="contact user service steward")),
        ),
        gerr=gerr!("timeout checks";
            config=ErrAutoID,
            tags=["user_service", "timeout"],
            gerr=gerr!("too many open files"; tag="tmof", data=("MAX", 50000))),
    );

    let json_report = gerr.report_as::<JsonReport>();
    let json_data = serde_json::to_string_pretty(&gerr.json_data()).unwrap();
    assert_eq!(json_report, json_data);
}

#[cfg(feature = "serde")]
const EXPECTED_DISPLAY_JSON_DATA_DEBUG: &str = r#"{
  "id": "AutoID",
  "prefix": "AutoPrefix",
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
      "message": "[400][NOT_FOUND] input is invalid: qwe",
      "caused_by": [
        {
          "message": "invalid digit found in string",
          "caused_by": null
        },
        {
          "message": "[OUTBOUND] upstream error",
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
    }
  ]
}"#;

#[cfg(feature = "serde")]
#[test]
fn test_display_json_data() {
    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoIDCode, Data> = gerr!("pretty error: {req_id}";
        config=ErrAutoIDCode,
        tag="tag1",
        tags=["tag2", "tag3"],
        data= Data {
            user_id,
            user_name,
        },
        help="send valid request",
        source=input_err.clone(),
        gerr=gerr!("input is invalid: {}", input;
            config=ErrIDi32,
            id=40,
            code="400",
            tag="bad_request",
            tag="invalid_input",
            help="pass valid input",
            data=("user_name".to_string(), "ajo".to_string()),
            source = input_err,
            gerr=gerr!("upstream error"; code="[OUTBOUND]", gerr=gerr!("got error from user service"; data=("caused by:".to_string(), "timout".to_string()), help="contact user service steward")),
        ),
        gerr=gerr!("timeout checks";
            config=ErrAutoID,
            tags=["user_service", "timeout"],
            gerr=gerr!("too many open files"; tag="tmof", data=("MAX", 50000))),
    );

    let display_data = gerr.display_json_data();
    let display_data_str = ::serde_json::to_string_pretty(&display_data).unwrap();
    assert_eq!(display_data_str, EXPECTED_DISPLAY_JSON_DATA_DEBUG);
}

#[cfg(feature = "serde")]
const EXPECTED_JSON_DATA_DEBUG: &str = r#"{
  "id": "AutoID",
  "prefix": "AutoPrefix",
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
    "file": "tests/json_report_test.rs",
    "line": 302,
    "column": 48
  },
  "sources": [
    {
      "id": null,
      "prefix": null,
      "message": "invalid digit found in string",
      "tags": null,
      "data": null,
      "location": null,
      "sources": null,
      "help": null
    },
    {
      "id": 40,
      "prefix": "[400][NOT_FOUND]",
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
        "file": "tests/json_report_test.rs",
        "line": 313,
        "column": 14
      },
      "sources": [
        {
          "id": null,
          "prefix": null,
          "message": "invalid digit found in string",
          "tags": null,
          "data": null,
          "location": null,
          "sources": null,
          "help": null
        },
        {
          "id": null,
          "prefix": "[OUTBOUND]",
          "message": "upstream error",
          "tags": null,
          "data": null,
          "location": {
            "file": "tests/json_report_test.rs",
            "line": 322,
            "column": 18
          },
          "sources": [
            {
              "id": null,
              "prefix": null,
              "message": "got error from user service",
              "tags": null,
              "data": [
                "caused by:",
                "timout"
              ],
              "location": {
                "file": "tests/json_report_test.rs",
                "line": 322,
                "column": 68
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
      "prefix": null,
      "message": "timeout checks",
      "tags": [
        "user_service",
        "timeout"
      ],
      "data": null,
      "location": {
        "file": "tests/json_report_test.rs",
        "line": 324,
        "column": 14
      },
      "sources": [
        {
          "id": null,
          "prefix": null,
          "message": "too many open files",
          "tags": [
            "tmof"
          ],
          "data": [
            "MAX",
            50000
          ],
          "location": {
            "file": "tests/json_report_test.rs",
            "line": 327,
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
      "prefix": null,
      "message": "connection timeout",
      "tags": null,
      "data": null,
      "location": {
        "file": "tests/json_report_test.rs",
        "line": 328,
        "column": 14
      },
      "sources": null,
      "help": null
    }
  ],
  "help": "send valid request",
  "backtrace": "<disabled>"
}"#;

#[cfg(feature = "serde")]
#[test]
fn test_json_data() {
    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoIDCode, Data> = gerr!("pretty error: {req_id}";
        config=ErrAutoIDCode,
        tag="tag1",
        tags=["tag2", "tag3"],
        data= Data {
            user_id,
            user_name,
        },
        help="send valid request",
        source=input_err.clone(),
        gerr=gerr!("input is invalid: {}", input;
            config=ErrIDi32,
            id=40,
            code="400",
            tag="bad_request",
            tag="invalid_input",
            help="pass valid input",
            data=("user_name".to_string(), "ajo".to_string()),
            source = input_err,
            gerr=gerr!("upstream error"; code="[OUTBOUND]", gerr=gerr!("got error from user service"; data=("caused by:".to_string(), "timout".to_string()), help="contact user service steward")),
        ),
        gerr=gerr!("timeout checks";
            config=ErrAutoID,
            tags=["user_service", "timeout"],
            gerr=gerr!("too many open files"; tag="tmof", data=("MAX", 50000))),
        gerr=gerr!("connection timeout"; config=ErrAutoID, data=NoData),
    );

    let json_data = serde_json::to_string_pretty(&gerr.json_data()).unwrap();
    assert_eq!(json_data, EXPECTED_JSON_DATA_DEBUG);
}
