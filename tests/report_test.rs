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

    let pretty_report = gerr.report();
    // println!("{}", pretty_report);
    assert_eq!(pretty_report, EXPECTED_REPORT);
    let pretty_report = gerr.report_as::<PrettyReport>();
    assert_eq!(pretty_report, EXPECTED_REPORT);
}

#[cfg(not(feature = "backtrace"))]
const EXPECTED_MARKDOWN_REPORT: &str = r#"# Error Report

## ID: AutoID

## Prefix: AutoPrefix

## Message

> pretty error: l2k3mr2l3r

## Data

```
Data {
    user_id: 123,
    user_name: "ajo",
}
```

## Help

> send valid request

## Tags

- tag1
- tag2
- tag3

## Location

tests/report_test.rs:397:48

## Causes

### 1. invalid digit found in string

### 2. [400] input is invalid: qwe

- **ID:** `40`

- **Location:** `tests/report_test.rs:408:14`

- **Tags:** *bad_request, invalid_input*

- **Help:** *pass valid input*

- **Data:**

```
(
    "user_name",
    "ajo",
)
```
- **Causes:**

    1. invalid digit found in string

    2. [OUTBOUND] upstream error

        - **ID:** `NoID`
        - **Location:** `tests/report_test.rs:416:18`
        - **Causes:**
            1. got error from user service

                - **ID:** `NoID`
                - **Location:** `tests/report_test.rs:416:68`
                - **Help:** *contact user service steward*

                - **Data:**

                ```text
                (
                    "caused by:",
                    "timout",
                )
                ```


### 3. timeout checks

- **ID:** `AutoID`

- **Location:** `tests/report_test.rs:418:14`

- **Tags:** *user_service, timeout*

- **Causes:**

    1. too many open files

        - **ID:** `NoID`
        - **Location:** `tests/report_test.rs:421:18`
        - **Tags:** *tmof*

        - **Data:**

        ```text
        (
            "MAX",
            50000,
        )
        ```

"#;
#[cfg(feature = "backtrace")]
const EXPECTED_MARKDOWN_REPORT: &str = r#"# Error Report

## ID: AutoID

## Prefix: AutoPrefix

## Message

> pretty error: l2k3mr2l3r

## Data

```
Data {
    user_id: 123,
    user_name: "ajo",
}
```

## Help

> send valid request

## Tags

- tag1
- tag2
- tag3

## Location

tests/report_test.rs:397:48

## Causes

### 1. invalid digit found in string

### 2. [400] input is invalid: qwe

- **ID:** `40`

- **Location:** `tests/report_test.rs:408:14`

- **Tags:** *bad_request, invalid_input*

- **Help:** *pass valid input*

- **Data:**

```
(
    "user_name",
    "ajo",
)
```
- **Causes:**

    1. invalid digit found in string

    2. [OUTBOUND] upstream error

        - **ID:** `NoID`
        - **Location:** `tests/report_test.rs:416:18`
        - **Causes:**
            1. got error from user service

                - **ID:** `NoID`
                - **Location:** `tests/report_test.rs:416:68`
                - **Help:** *contact user service steward*

                - **Data:**

                ```text
                (
                    "caused by:",
                    "timout",
                )
                ```


### 3. timeout checks

- **ID:** `AutoID`

- **Location:** `tests/report_test.rs:418:14`

- **Tags:** *user_service, timeout*

- **Causes:**

    1. too many open files

        - **ID:** `NoID`
        - **Location:** `tests/report_test.rs:421:18`
        - **Tags:** *tmof*

        - **Data:**

        ```text
        (
            "MAX",
            50000,
        )
        ```

## Backtrace

```
<disabled>
```
"#;

#[test]
fn test_markdown_report() {
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

    let markdown_report = gerr.report_as::<MarkdownReport>();
    println!("{}", markdown_report);
    assert_eq!(markdown_report, EXPECTED_MARKDOWN_REPORT);
}

const EXPECTED_TRACE_REPORT: &str = r#"[AutoID] AutoPrefix pretty error: l2k3mr2l3r
├─ invalid digit found in string
├─ [40] [400][NOT_FOUND] input is invalid: qwe
│  ├─ invalid digit found in string
│  └─ [NoID] [OUTBOUND] upstream error
│     └─ [NoID] got error from user service
└─ [AutoID] timeout checks
   └─ [NoID] too many open files
"#;

const EXPECTED_TRACE_REPORT_NO_PREFIX: &str = r#"[AutoID] pretty error: l2k3mr2l3r
├─ invalid digit found in string
├─ [40] [400][NOT_FOUND] input is invalid: qwe
│  ├─ invalid digit found in string
│  └─ [NoID] [OUTBOUND] upstream error
│     └─ [NoID] got error from user service
└─ [AutoID] timeout checks
   └─ [NoID] too many open files
"#;

#[test]
fn test_trace_report() {
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

    let trace_report = gerr.report_as::<TraceReport>();
    assert_eq!(trace_report, EXPECTED_TRACE_REPORT);

    // ----------------------------------------------------------

    let user_id = 123;
    let user_name = "ajo".into();
    let req_id = "l2k3mr2l3r";
    let input = "qwe";
    let input_err = input.parse::<u64>().unwrap_err();
    let gerr: GErr<ErrAutoID, Data> = gerr!("pretty error: {req_id}";
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
    assert_eq!(trace_report, EXPECTED_TRACE_REPORT_NO_PREFIX);
}
