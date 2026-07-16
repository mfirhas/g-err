pub mod test_pretty_report_data {
    #[cfg(not(feature = "backtrace"))]
    pub const EXPECTED_REPORT: &str = r#"Error Report
============
ID: AutoID
Code: AutoCode
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
Location: tests/report_test.rs:18:43
Caused by:
  1: invalid digit found in string
  2: input is invalid: qwe
     id: 40
     code: 400
     at: tests/report_test.rs:28:14
     tags: bad_request, invalid_input
     help: pass valid input
     data:
       (
           "user_name",
           "ajo",
       )
     caused by:
      - invalid digit found in string
      - upstream error
        id: -
        code: [OUTBOUND]
        at: tests/report_test.rs:37:18
        caused by:
         - got error from user service
           id: -
           code: -
           at: tests/report_test.rs:37:66
           help: contact user service steward
           data:
             (
                 "caused by:",
                 "timout",
             )
  3: timeout checks
     id: AutoID
     code: -
     at: tests/report_test.rs:39:14
     tags: user_service, timeout
     caused by:
      - too many open files
        id: -
        code: -
        at: tests/report_test.rs:42:18
        tags: tmof
        data:
          (
              "MAX",
              50000,
          )
"#;

    #[cfg(feature = "backtrace")]
    pub const EXPECTED_REPORT: &str = r#"Error Report
============
ID: AutoID
Code: AutoCode
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
Location: tests/report_test.rs:18:43
Caused by:
  1: invalid digit found in string
  2: input is invalid: qwe
     id: 40
     code: 400
     at: tests/report_test.rs:28:14
     tags: bad_request, invalid_input
     help: pass valid input
     data:
       (
           "user_name",
           "ajo",
       )
     caused by:
      - invalid digit found in string
      - upstream error
        id: -
        code: [OUTBOUND]
        at: tests/report_test.rs:37:18
        caused by:
         - got error from user service
           id: -
           code: -
           at: tests/report_test.rs:37:66
           help: contact user service steward
           data:
             (
                 "caused by:",
                 "timout",
             )
  3: timeout checks
     id: AutoID
     code: -
     at: tests/report_test.rs:39:14
     tags: user_service, timeout
     caused by:
      - too many open files
        id: -
        code: -
        at: tests/report_test.rs:42:18
        tags: tmof
        data:
          (
              "MAX",
              50000,
          )
Backtrace:
<disabled>
"#;
}

pub mod test_markdown_report_data {
    #[cfg(not(feature = "backtrace"))]
    pub const EXPECTED_MARKDOWN_REPORT: &str = r#"# Error Report

## ID: AutoID

## Code: AutoCode

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

tests/report_test.rs:59:43

## Causes

### 1. invalid digit found in string

### 2. input is invalid: qwe

- **ID:** `40`

- **Code:** `400`

- **Location:** `tests/report_test.rs:69:14`

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

    2. upstream error
        - **ID:** `-`
        - **Code:** `[OUTBOUND]`
        - **Location:** `tests/report_test.rs:78:18`
        - **Causes:**
            1. got error from user service
                - **ID:** `-`
                - **Code:** `-`
                - **Location:** `tests/report_test.rs:78:66`
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

- **Code:** `-`

- **Location:** `tests/report_test.rs:80:14`

- **Tags:** *user_service, timeout*

- **Causes:**

    1. too many open files
        - **ID:** `-`
        - **Code:** `-`
        - **Location:** `tests/report_test.rs:83:18`
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
    pub const EXPECTED_MARKDOWN_REPORT: &str = r#"# Error Report

## ID: AutoID

## Code: AutoCode

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

tests/report_test.rs:59:43

## Causes

### 1. invalid digit found in string

### 2. input is invalid: qwe

- **ID:** `40`

- **Code:** `400`

- **Location:** `tests/report_test.rs:69:14`

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

    2. upstream error
        - **ID:** `-`
        - **Code:** `[OUTBOUND]`
        - **Location:** `tests/report_test.rs:78:18`
        - **Causes:**
            1. got error from user service
                - **ID:** `-`
                - **Code:** `-`
                - **Location:** `tests/report_test.rs:78:66`
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

- **Code:** `-`

- **Location:** `tests/report_test.rs:80:14`

- **Tags:** *user_service, timeout*

- **Causes:**

    1. too many open files
        - **ID:** `-`
        - **Code:** `-`
        - **Location:** `tests/report_test.rs:83:18`
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
}

pub mod test_trace_report_data {
    pub const EXPECTED_TRACE_REPORT: &str = r#"[AutoID][AutoCode] pretty error: l2k3mr2l3r
├─ invalid digit found in string
├─ [40][400] input is invalid: qwe
│  ├─ invalid digit found in string
│  └─ [-][[OUTBOUND]] upstream error
│     └─ [-][-] got error from user service
└─ [AutoID][-] timeout checks
   └─ [-][-] too many open files
"#;

    pub const EXPECTED_TRACE_REPORT_NO_CODE: &str = r#"[AutoID][-] pretty error: l2k3mr2l3r
├─ invalid digit found in string
├─ [40][400] input is invalid: qwe
│  ├─ invalid digit found in string
│  └─ [-][[OUTBOUND]] upstream error
│     └─ [-][-] got error from user service
└─ [AutoID][-] timeout checks
   └─ [-][-] too many open files
"#;
}
