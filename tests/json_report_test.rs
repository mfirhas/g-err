#[cfg(feature = "serde")]
#[path = "setup_test.rs"]
mod setup_test;
#[cfg(feature = "serde")]
use setup_test::*;

#[cfg(feature = "serde")]
use g_err::{json::JsonReport, *};

#[path = "json_report_test_data.rs"]
mod json_report_test_data;

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
#[test]
fn test_display_json_data() {
    use json_report_test_data::test_display_json_data_data;
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
    assert_eq!(
        display_data_str,
        test_display_json_data_data::EXPECTED_DISPLAY_JSON_DATA_DEBUG
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_json_data() {
    use json_report_test_data::test_json_data_data;
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
    assert_eq!(json_data, test_json_data_data::EXPECTED_JSON_DATA_DEBUG);
}
