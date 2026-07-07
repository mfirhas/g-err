#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

use g_err::*;

#[cfg(not(feature = "backtrace"))]
const EXPECTED_REPORT: &str = r#"Error Report
============
ID: AutoID
Prefix: AutoPrefix
Message: pretty error: l2k3mr2l3r
Data:
   Data {
       user_id: 123,
       user_name: "ajo",
   }
Help: send valid request
Tags:
  - tag1
  - tag2
  - tag3
Location: tests/report_test.rs:136:48
Caused by:
  1: invalid digit found in string
  2: [400] input is invalid: qwe
     id: 40
     at: tests/report_test.rs:147:14
     tags: bad_request, invalid_input
     help: pass valid input
     data:
       (
           "user_name",
           "ajo",
       )
     caused by:
      - invalid digit found in string
      - [OUTBOUND] upstream error
        id: NoID
        at: tests/report_test.rs:155:18
        caused by:
         - got error from user service
           id: NoID
           at: tests/report_test.rs:155:68
           help: contact user service steward
           data:
             (
                 "caused by:",
                 "timout",
             )
  3: timeout checks
     id: AutoID
     at: tests/report_test.rs:157:14
     tags: user_service, timeout
     caused by:
      - too many open files
        id: NoID
        at: tests/report_test.rs:160:18
        tags: tmof
        data:
          (
              "MAX",
              50000,
          )
"#;

#[cfg(feature = "backtrace")]
const EXPECTED_REPORT: &str = r#"Error Report
============
ID: AutoID
Prefix: AutoPrefix
Message: pretty error: l2k3mr2l3r
Data:
   Data {
       user_id: 123,
       user_name: "ajo",
   }
Help: send valid request
Tags:
  - tag1
  - tag2
  - tag3
Location: tests/report_test.rs:136:48
Caused by:
  1: invalid digit found in string
  2: [400] input is invalid: qwe
     id: 40
     at: tests/report_test.rs:147:14
     tags: bad_request, invalid_input
     help: pass valid input
     data:
       (
           "user_name",
           "ajo",
       )
     caused by:
      - invalid digit found in string
      - [OUTBOUND] upstream error
        id: NoID
        at: tests/report_test.rs:155:18
        caused by:
         - got error from user service
           id: NoID
           at: tests/report_test.rs:155:68
           help: contact user service steward
           data:
             (
                 "caused by:",
                 "timout",
             )
  3: timeout checks
     id: AutoID
     at: tests/report_test.rs:157:14
     tags: user_service, timeout
     caused by:
      - too many open files
        id: NoID
        at: tests/report_test.rs:160:18
        tags: tmof
        data:
          (
              "MAX",
              50000,
          )
Backtrace:
<disabled>
"#;

#[test]
fn test_pretty_report() {
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
            data=("user_name".to_string(), "ajo".to_string()),
            source = input_err,
            gerr=gerr!("upstream error"; prefix="[OUTBOUND]", gerr=gerr!("got error from user service"; data=("caused by:".to_string(), "timout".to_string()), help="contact user service steward")),
        ),
        gerr=gerr!("timeout checks";
            id_auto=AutoID,
            tags=["user_service", "timeout"],
            gerr=gerr!("too many open files"; tag="tmof", data=("MAX", 50000))),
    );

    let pretty_report = gerr.report();
    // println!("{}", pretty_report);
    assert_eq!(pretty_report, EXPECTED_REPORT);
    let pretty_report = gerr.report_as::<PrettyReport>();
    assert_eq!(pretty_report, EXPECTED_REPORT);
}
