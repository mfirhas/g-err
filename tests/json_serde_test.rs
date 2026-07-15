#[path = "setup_test.rs"]
mod setup_test;
#[cfg(feature = "serde")]
use g_err::*;
#[cfg(feature = "serde")]
use setup_test::*;

#[cfg(feature = "serde")]
const EXPECTED_JSON_DATA_DEBUG: &str = r#"{
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
    "line": 165,
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
        "line": 175,
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
            "line": 184,
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
                "line": 184,
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
        "line": 186,
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
            "line": 189,
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
        "line": 190,
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
        gerr=gerr!("connection timeout"; config=ErrAutoID, data=NoData),
    );

    const EXPECTED_STR_SER: &str = "\"[AutoID][AutoCode] pretty error: l2k3mr2l3r\"";
    let json_data_ser = serde_json::to_string(&gerr).unwrap();
    assert_eq!(json_data_ser, EXPECTED_STR_SER);

    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();
    assert_eq!(json_data_ser, EXPECTED_JSON_DATA_DEBUG);

    // deserialize json data into GErr
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrAutoIDCode, Data> =
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
    let gerr: GErr<ErrAutoCode, NoData> = gerr!("pretty error: {req_id}";
        config,
        tag="tag1",
        tags=["tag2", "tag3"],
        data_type,
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
        gerr=gerr!("connection timeout";),
    );

    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrAutoCode, NoData> =
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
#[test]
fn test_json_serde_failed_deser() {
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoCode, NoData> = gerr!("pretty error: {req_id}";
        config,
        tag="tag1",
        tags=["tag2", "tag3"],
        data_type,
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
        gerr=gerr!("connection timeout"),
    );

    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: core::result::Result<GErr<ErrAutoIDCode, NoData>, serde_json::Error> =
        g_err::serde::json::deserialize(&mut deserializer);
    assert!(err.is_err());

    let gerr = gerr.with_data(("test", "data"));
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: core::result::Result<GErr<ErrAutoCode, NoData>, serde_json::Error> =
        g_err::serde::json::deserialize(&mut deserializer);
    assert!(err.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_json_serde_id_types() {
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoCode, NoData> = gerr!("pretty error: {req_id}";
        config,
        tag="tag1",
        tags=["tag2", "tag3"],
        data_type,
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
        gerr=gerr!("connection timeout"; config=()),
    );

    // id = bool
    let gerr = gerr.with_config::<ErrIDBoolCode>().set_id(true);
    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrIDBoolCode> = g_err::serde::json::deserialize(&mut deserializer).unwrap();
    assert_eq!(err.id().unwrap(), &true);

    // id = array
    let gerr = gerr.with_config::<ErrIDArrCode>().set_id([123, 234]);
    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrIDArrCode, NoData> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();
    assert_eq!(err.id().unwrap(), &[123, 234]);

    // id = object
    let gerr = gerr.with_config::<ErrIDDataCode>().set_id(Data {
        user_id: 123,
        user_name: String::from("ajo"),
    });
    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrIDDataCode, NoData> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();
    assert_eq!(
        err.id().unwrap(),
        &Data {
            user_id: 123,
            user_name: "ajo".into()
        }
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_json_serde_sources_id_types() {
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<_, NoData> = gerr!("pretty error: {req_id}";
        config=ErrAutoCode,
        tag="tag1",
        tags=["tag2", "tag3"],
        data_type,
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
        gerr=gerr!("connection timeout"),
    );

    // id = bool
    let gerr = gerr.add_source_gerr(gerr!("id bool"; config=ErrIDBool, id=true));
    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrAutoCode, NoData> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();
    if let Source::GErr(ref gerr) = err.sources().unwrap()[4]
        && let Some(id) =
            (&**gerr.id.as_ref().unwrap() as &dyn core::any::Any).downcast_ref::<bool>()
    {
        assert_eq!(id, &true);
    } else {
        panic!("expected Source::GErr");
    }

    #[derive(Debug)]
    struct ArrID([i32; 2]);
    impl core::fmt::Display for ArrID {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}, {}]", self.0[0], self.0[1])
        }
    }
    impl ::serde::Serialize for ArrID {
        fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ArrID {
        fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            <[i32; 2]>::deserialize(deserializer).map(ArrID)
        }
    }
    impl Config for ArrID {
        type Id = Self;
    }
    // id = array
    let gerr = gerr.add_source_gerr(gerr!("id array"; config=ArrID, id=ArrID([123, 234])));
    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrAutoCode, NoData> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();
    if let Source::GErr(ref gerr) = err.sources().unwrap()[5]
        && let serde_json::Value::Array(arr) = &gerr.id_json
    {
        assert_eq!(arr.len(), 2);
        if let serde_json::Value::Number(num) = &arr[0]
            && let serde_json::Value::Number(num2) = &arr[1]
        {
            assert_eq!(num.as_i64().unwrap(), 123);
            assert_eq!(num2.as_i64().unwrap(), 234);
        } else {
            panic!("expected id array 123 and 234");
        }
    } else {
        panic!("expected Source::GErr");
    }

    // id = object
    let gerr = gerr.add_source_gerr(gerr!("id data"; config=ErrIDDataCode, id=Data {
        user_id: 123,
        user_name: String::from("ajo"),
    }));
    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrAutoCode, NoData> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();
    if let Source::GErr(ref gerr) = err.sources().unwrap()[6]
        && let Some(id) =
            (&**gerr.id.as_ref().unwrap() as &dyn core::any::Any).downcast_ref::<String>()
    {
        assert_eq!(id, r#"{"user_id":123,"user_name":"ajo"}"#);

        if let serde_json::Value::Object(obj) = &gerr.id_json
            && let serde_json::Value::Number(num) = obj.get("user_id".into()).unwrap()
            && let serde_json::Value::String(s) = obj.get("user_name".into()).unwrap()
        {
            assert_eq!(num.as_i64().unwrap(), 123);
            assert_eq!(s, "ajo");
        } else {
            panic!("expected json value as object");
        }
    } else {
        panic!("expected Source::GErr");
    }
}

#[cfg(feature = "serde")]
#[test]
fn test_json_serde_sources_data_types() {
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoCode, NoData> = gerr!("pretty error: {req_id}";
        config,
        tag="tag1",
        tags=["tag2", "tag3"],
        data_type,
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
        gerr=gerr!("connection timeout"),
    );

    // data = bool
    let gerr = gerr.add_source_gerr(gerr!("data bool"; data=true));
    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrAutoCode, NoData> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();
    if let Source::GErr(ref gerr) = err.sources().unwrap()[4]
        && let serde_json::Value::Bool(b) = gerr.data_json.as_ref().unwrap()
    {
        assert_eq!(b, &true);
    } else {
        panic!("expected Source::GErr");
    }

    // data = num
    let gerr = gerr.add_source_gerr(gerr!("data bool"; data=123));
    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrAutoCode, NoData> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();
    if let Source::GErr(ref gerr) = err.sources().unwrap()[5]
        && let serde_json::Value::Number(num) = gerr.data_json.as_ref().unwrap()
    {
        assert_eq!(num.as_i64().unwrap(), 123);
    } else {
        panic!("expected Source::GErr");
    }

    // data = string
    let gerr = gerr.add_source_gerr(gerr!("data bool"; data="dono"));
    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrAutoCode, NoData> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();
    if let Source::GErr(ref gerr) = err.sources().unwrap()[6]
        && let serde_json::Value::String(s) = gerr.data_json.as_ref().unwrap()
    {
        assert_eq!(s, "dono");
    } else {
        panic!("expected Source::GErr");
    }

    // data = data
    let gerr = gerr
        .add_source_gerr(gerr!("data bool"; data=Data{user_id:22, user_name: "anu".to_string()}));
    // serialize json data into string
    let json_data_ser = serde_json::to_string_pretty(&gerr.json_data()).unwrap();

    // deserialize json data into GErr with mismatch id and data types
    let mut deserializer = serde_json::Deserializer::from_str(&json_data_ser);
    let err: GErr<ErrAutoCode, NoData> =
        g_err::serde::json::deserialize(&mut deserializer).unwrap();
    if let Source::GErr(ref gerr) = err.sources().unwrap()[7]
        && let serde_json::Value::Object(s) = gerr.data_json.as_ref().unwrap()
    {
        if let serde_json::Value::Number(num) = s.get("user_id").unwrap()
            && let serde_json::Value::String(s) = s.get("user_name").unwrap()
        {
            assert_eq!(num.as_i64().unwrap(), 22);
            assert_eq!(s, "anu");
        } else {
            panic!("user_id is expected number")
        }
    } else {
        panic!("expected Source::GErr");
    }
}

#[cfg(feature = "serde")]
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
struct Serde {
    thing: String,
    #[serde(with = "g_err::serde::json")]
    gerr_internal: GErr<ErrAutoIDCode, Data>,
}

#[cfg(feature = "serde")]
#[derive(Debug, ::serde::Serialize)]
struct Ser {
    #[serde(serialize_with = "g_err::serde::display_json::serialize")]
    gerr_public: GErr<ErrAutoIDCode, Data>,
}

#[cfg(feature = "serde")]
#[test]
fn test_json_serde_data() {
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
        gerr=gerr!("connection timeout"),
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
        "message": "400 - input is invalid: qwe",
        "caused_by": [
          {
            "message": "invalid digit found in string",
            "caused_by": null
          },
          {
            "message": "[OUTBOUND] - upstream error",
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
        gerr=gerr!("connection timeout"; config=()),
    );

    let ser = Ser { gerr_public: gerr };
    let ser = serde_json::to_string_pretty(&ser).unwrap();

    assert_eq!(ser, EXPECTED_DISPLAY_DATA_JSON);
}

#[cfg(feature = "serde")]
#[test]
fn test_json_serde_data_failed_deser() {
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
        gerr=gerr!("connection timeout"),
    );

    struct U64;
    impl Config for U64 {
        type Id = u64;
    }

    #[cfg(feature = "serde")]
    #[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
    struct Serde2 {
        thing: String,
        #[serde(with = "g_err::serde::json")]
        gerr_internal: GErr<U64, Data>,
    }

    let ser_de = Serde {
        thing: "test".into(),
        gerr_internal: gerr,
    };
    let ser = serde_json::to_string_pretty(&ser_de).unwrap();
    let de: core::result::Result<Serde2, _> = serde_json::from_str(&ser);

    assert!(de.is_err());
}
