#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

use g_err::*;

#[path = "report_test_data.rs"]
mod report_test_data;

#[test]
fn test_pretty_report() {
    use report_test_data::test_pretty_report_data;
    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoIDCode, Data> = gerr!("pretty error: {}", req_id;
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

    let pretty_report = gerr.report();
    assert_eq!(pretty_report, test_pretty_report_data::EXPECTED_REPORT);
    let pretty_report = gerr.report_as::<PrettyReport>();
    assert_eq!(pretty_report, test_pretty_report_data::EXPECTED_REPORT);
}

#[test]
fn test_markdown_report() {
    use report_test_data::test_markdown_report_data;
    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoIDCode, Data> = gerr!("pretty error: {}", req_id;
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

    let markdown_report = gerr.report_as::<MarkdownReport>();
    assert_eq!(
        markdown_report,
        test_markdown_report_data::EXPECTED_MARKDOWN_REPORT
    );
}

#[test]
fn test_trace_report() {
    use report_test_data::test_trace_report_data;
    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoIDCode, Data> = gerr!("pretty error: {}", req_id;
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

    let trace_report = gerr.report_as::<TraceReport>();
    assert_eq!(trace_report, test_trace_report_data::EXPECTED_TRACE_REPORT);

    // ----------------------------------------------------------

    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoID, Data> = gerr!("pretty error: {}", req_id;
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

    let trace_report = gerr.report_as::<TraceReport>();
    assert_eq!(
        trace_report,
        test_trace_report_data::EXPECTED_TRACE_REPORT_NO_CODE
    );
}
