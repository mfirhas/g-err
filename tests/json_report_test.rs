#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

use g_err::*;

#[cfg(feature = "serde")]
use g_err::json::DisplayJsonReport;

#[cfg(feature = "serde")]
#[test]
fn test_display_json_report() {
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
    );

    let display_json_report = gerr.report_as::<DisplayJsonReport>();
    let display_json_data = serde_json::to_string_pretty(&gerr.display_json_data()).unwrap();
    println!("{}", display_json_report);
    println!("{}", display_json_data);
    assert_eq!(display_json_report, display_json_data);
}
