#[path = "setup_test.rs"]
mod setup_test;

#[cfg(feature = "serde")]
use setup_test::*;

#[cfg(feature = "serde")]
use g_err::*;

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
    "file": "tests/json_serde_test.rs",
    "line": 167,
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
        "file": "tests/json_serde_test.rs",
        "line": 178,
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
            "file": "tests/json_serde_test.rs",
            "line": 187,
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
                "file": "tests/json_serde_test.rs",
                "line": 187,
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
        "file": "tests/json_serde_test.rs",
        "line": 189,
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
            "file": "tests/json_serde_test.rs",
            "line": 192,
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
        "file": "tests/json_serde_test.rs",
        "line": 193,
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
fn test_json_serde() {
    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<AutoID, AutoPrefix, Data> = gerr!("pretty error: {req_id}";
        id_auto,
        prefix_auto,
        tag="tag1",
        tags=["tag2", "tag3"],
        data= Data {
            user_id,
            user_name,
        },
        help="send valid request",
        source=input_err.clone(),
        gerr=gerr!("input is invalid: {}", input;
            id=40,
            prefix="[400]",
            tag="bad_request",
            tag="invalid_input",
            help="pass valid input",
            aprefix = "[NOT_FOUND]",
            data=("user_name".to_string(), "ajo".to_string()),
            source = input_err,
            gerr=gerr!("upstream error"; prefix="[OUTBOUND]", gerr=gerr!("got error from user service"; data=("caused by:".to_string(), "timout".to_string()), help="contact user service steward")),
        ),
        gerr=gerr!("timeout checks";
            id_auto=AutoID,
            tags=["user_service", "timeout"],
            gerr=gerr!("too many open files"; tag="tmof", data=("MAX", 50000))),
        gerr=gerr!("connection timeout"; id_auto=NoID, data=NoData),
    );

    const EXPECTED_STR_SER: &str = "\"AutoPrefix pretty error: l2k3mr2l3r\"";
    let json_data_ser = serde_json::to_string(&gerr).unwrap();
    assert_eq!(json_data_ser, EXPECTED_STR_SER);

    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();
    assert_eq!(json_data_ser, EXPECTED_JSON_DATA_DEBUG);

    // deserialize json data into GErr
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<AutoID, AutoPrefix, Data> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();

    // serialize back deserialized json data, into string
    let json_data_de = serde_json::to_string_pretty(&err.json_data()).unwrap();

    // compare both, must be equal
    assert_eq!(
        json_data_ser, json_data_de,
        "serialized and deserialized json data must be equal"
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_json_serde_noid_nodata() {
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<NoID, AutoPrefix, NoData> = gerr!("pretty error: {req_id}";
        id_auto,
        prefix_auto,
        tag="tag1",
        tags=["tag2", "tag3"],
        data_type,
        help="send valid request",
        source=input_err.clone(),
        gerr=gerr!("input is invalid: {}", input;
            id=40,
            prefix="[400]",
            tag="bad_request",
            tag="invalid_input",
            help="pass valid input",
            aprefix = "[NOT_FOUND]",
            data=("user_name".to_string(), "ajo".to_string()),
            source = input_err,
            gerr=gerr!("upstream error"; prefix="[OUTBOUND]", gerr=gerr!("got error from user service"; data=("caused by:".to_string(), "timout".to_string()), help="contact user service steward")),
        ),
        gerr=gerr!("timeout checks";
            id_auto=AutoID,
            tags=["user_service", "timeout"],
            gerr=gerr!("too many open files"; tag="tmof", data=("MAX", 50000))),
        gerr=gerr!("connection timeout"; id_auto=NoID, data=NoData),
    );

    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<NoID, AutoPrefix, NoData> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();

    // serialize back deserialized json data, into string
    let json_data_de = serde_json::to_string_pretty(&err.json_data()).unwrap();

    // compare both, must be equal
    assert_eq!(
        json_data_ser, json_data_de,
        "serialized and deserialized json data must be equal"
    );

    let _no_id: NoID = serde_json::from_str("null").unwrap();
    let _no_data: NoData = serde_json::from_str("null").unwrap();
}

#[cfg(feature = "serde")]
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
struct Serde {
    thing: String,
    #[serde(with = "g_err::serde::json")]
    gerr_internal: GErr<AutoID, AutoPrefix, Data>,
}

#[cfg(feature = "serde")]
#[derive(Debug, ::serde::Serialize)]
struct Ser {
    #[serde(serialize_with = "g_err::serde::display_json::serialize")]
    gerr_public: GErr<AutoID, AutoPrefix, Data>,
}

#[cfg(feature = "serde")]
#[test]
fn test_json_serde_data() {
    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<AutoID, AutoPrefix, Data> = gerr!("pretty error: {req_id}";
        id_auto,
        prefix_auto,
        tag="tag1",
        tags=["tag2", "tag3"],
        data= Data {
            user_id,
            user_name,
        },
        help="send valid request",
        source=input_err.clone(),
        gerr=gerr!("input is invalid: {}", input;
            id=40,
            prefix="[400]",
            tag="bad_request",
            tag="invalid_input",
            help="pass valid input",
            aprefix = "[NOT_FOUND]",
            data=("user_name".to_string(), "ajo".to_string()),
            source = input_err,
            gerr=gerr!("upstream error"; prefix="[OUTBOUND]", gerr=gerr!("got error from user service"; data=("caused by:".to_string(), "timout".to_string()), help="contact user service steward")),
        ),
        gerr=gerr!("timeout checks";
            id_auto=AutoID,
            tags=["user_service", "timeout"],
            gerr=gerr!("too many open files"; tag="tmof", data=("MAX", 50000))),
        gerr=gerr!("connection timeout"; id_auto=NoID, data=NoData),
    );

    let ser_de = Serde {
        thing: "test".into(),
        gerr_internal: gerr,
    };
    let ser = serde_json::to_string_pretty(&ser_de).unwrap();
    let de: Serde = serde_json::from_str(&ser).unwrap();
    let deser = serde_json::to_string_pretty(&de).unwrap();

    assert_eq!(
        ser, deser,
        "serialized and deserialized json data must be equal"
    );
}

#[cfg(feature = "serde")]
const EXPECTED_DISPLAY_DATA_JSON: &str = r#"{
  "gerr_public": {
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
      },
      {
        "message": "connection timeout",
        "caused_by": null
      }
    ]
  }
}"#;

#[cfg(feature = "serde")]
#[test]
fn test_json_serialize_data() {
    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<AutoID, AutoPrefix, Data> = gerr!("pretty error: {req_id}";
        id_auto,
        prefix_auto,
        tag="tag1",
        tags=["tag2", "tag3"],
        data= Data {
            user_id,
            user_name,
        },
        help="send valid request",
        source=input_err.clone(),
        gerr=gerr!("input is invalid: {}", input;
            id=40,
            prefix="[400]",
            tag="bad_request",
            tag="invalid_input",
            help="pass valid input",
            aprefix = "[NOT_FOUND]",
            data=("user_name".to_string(), "ajo".to_string()),
            source = input_err,
            gerr=gerr!("upstream error"; prefix="[OUTBOUND]", gerr=gerr!("got error from user service"; data=("caused by:".to_string(), "timout".to_string()), help="contact user service steward")),
        ),
        gerr=gerr!("timeout checks";
            id_auto=AutoID,
            tags=["user_service", "timeout"],
            gerr=gerr!("too many open files"; tag="tmof", data=("MAX", 50000))),
        gerr=gerr!("connection timeout"; id_auto=NoID, data=NoData),
    );

    let ser = Ser { gerr_public: gerr };
    let ser = serde_json::to_string_pretty(&ser).unwrap();

    assert_eq!(ser, EXPECTED_DISPLAY_DATA_JSON);
}
