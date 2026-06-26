extern crate alloc;
use core::fmt::Debug;
use g_err::{GErr, GErrSource, GResultExt, Id, NoData, NoID, NoPrefix, Prefix, Result, ResultExt};
use uuid::Uuid;

/// Auto-generated ID type for testing
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
struct AutoId(Uuid);

impl core::fmt::Display for AutoId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AutoId({})", self.0)
    }
}

impl Id for AutoId {
    fn id() -> Self {
        AutoId(Uuid::new_v4())
    }
}

/// Prefix type for testing
struct TestPrefix;
impl Prefix for TestPrefix {
    const PREFIX: Option<&'static str> = Some("[TEST]");
}

/// Data type for testing
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
struct TestData {
    code: u32,
}

// ============================================================================
// Tests for Into<GErrSource> - From GErr
// ============================================================================

#[test]
fn test_gerr_into_gerr_source_basic() {
    let err: GErr<u32> = GErr::new_with_id(42, "test error");
    let source: GErrSource = err.into();

    assert_eq!(source.message, "test error");
}

#[test]
fn test_gerr_into_gerr_source_preserves_fields() {
    let err: GErr<u32> = GErr::new_with_id(42, "test error")
        .set_prefix("[ERROR]")
        .set_help("help text")
        .append_prefix("*")
        .add_tag("critical")
        .prepend_prefix("*");

    let source: GErrSource = err.into();

    assert_eq!(source.message, "test error");
    assert_eq!(source.prefix, Some("*[ERROR]*".into()));
    assert_eq!(source.help, Some("help text".into()));
    assert_eq!(source.tags.unwrap().len(), 1);
}

#[test]
fn test_gerr_into_gerr_source_with_data() {
    let err: GErr<u32, NoPrefix, TestData> =
        GErr::new_with_id(42, "error with data").set_data(TestData { code: 404 });

    let source: GErrSource = err.into();

    assert_eq!(source.message, "error with data");
    assert!(source.data.is_some());
}

#[test]
fn test_gerr_into_gerr_source_with_sources() {
    let err: GErr<u32> = GErr::new_with_id(42, "parent error").add_source(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "child error",
    ));

    let source: GErrSource = err.into();

    assert_eq!(source.message, "parent error");
    assert!(source.sources.is_some());
    assert_eq!(source.sources.unwrap().len(), 1);
}

#[test]
fn test_gerr_into_gerr_source_with_all_fields() {
    let err: GErr<u32, TestPrefix, TestData> = GErr::new_with_id(42, "complete error")
        .set_prefix("[CRITICAL]")
        .set_help("restart the service")
        .add_tag("production")
        .add_tag("urgent")
        .set_data(TestData { code: 500 })
        .add_source(std::io::Error::new(std::io::ErrorKind::Other, "root cause"));

    let source: GErrSource = err.into();

    assert_eq!(source.message, "complete error");
    assert_eq!(source.prefix, Some("[CRITICAL]".into()));
    assert_eq!(source.help, Some("restart the service".into()));
    assert_eq!(source.tags.unwrap().len(), 2);
    assert!(source.data.is_some());
    assert_eq!(source.sources.unwrap().len(), 1);
}

// ============================================================================
// Tests for GErr::into_gerr_source() method
// ============================================================================

#[test]
fn test_gerr_into_gerr_source_method_basic() {
    let err: GErr<u32> = GErr::new_with_id(42, "test error");
    let source = err.into_gerr_source();

    assert_eq!(source.message, "test error");
}

#[test]
fn test_gerr_into_gerr_source_method_equivalent_to_into() {
    let err1: GErr<u32> = GErr::new_with_id(42, "test")
        .set_prefix("[P]")
        .add_tag("tag");
    let err2 = GErr::<i32>::new_with_id(42, "test")
        .set_prefix("[P]")
        .add_tag("tag");

    let source1 = err1.into_gerr_source();
    let source2: GErrSource = err2.into();

    assert_eq!(source1.message, source2.message);
    assert_eq!(source1.prefix, source2.prefix);
}

// ============================================================================
// Tests for ResultExt trait: context()
// ============================================================================

#[test]
fn test_result_context_with_standard_error() {
    let result: core::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "file not found",
    ));

    let gerr_result: Result<i32, NoID> = result.context("failed to read file");

    assert!(gerr_result.is_err());
    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "failed to read file");
    assert!(err.sources().is_some());
}

#[test]
fn test_result_context_with_owned_string() {
    let result: core::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "access denied",
    ));

    let msg = String::from("operation failed");
    let gerr_result: Result<i32, NoID> = result.context(msg);

    assert!(gerr_result.is_err());
    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "operation failed");
}

#[test]
fn test_result_context_with_custom_id_type() {
    let result: core::result::Result<i32, std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "error"));

    let gerr_result: Result<i32, AutoId> = result.context("wrapped error");

    assert!(gerr_result.is_err());
    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "wrapped error");
    // ID should be auto-generated
    assert!(err.id().0.to_string().len() > 0);
}

#[test]
fn test_result_context_with_prefix() {
    let result: core::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "not found",
    ));

    let gerr_result: Result<i32, u32, TestPrefix> = result.into_gerr_with_id(123);

    assert!(gerr_result.is_err());
    let err = gerr_result.unwrap_err();
    assert_eq!(err.prefix(), Some("[TEST]"));
}

#[test]
fn test_result_context_success_case() {
    let result: core::result::Result<i32, std::io::Error> = Ok(42);

    let gerr_result: Result<i32, NoID> = result.context("should not be used");

    assert!(gerr_result.is_ok());
    assert_eq!(gerr_result.unwrap(), 42);
}

#[test]
fn test_result_context_preserves_source() {
    let original_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let result: core::result::Result<i32, std::io::Error> = Err(original_error);

    let gerr_result: Result<i32, NoID> = result.context("wrapped");

    let err = gerr_result.unwrap_err();
    assert!(err.sources().is_some());
    assert_eq!(err.sources().unwrap().len(), 1);
}

// ============================================================================
// Tests for ResultExt trait: with_context()
// ============================================================================

#[test]
fn test_result_with_context_basic() {
    let result: core::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "not found",
    ));

    let gerr_result: Result<i32, NoID> = result.with_context(|| "dynamic message");

    assert!(gerr_result.is_err());
    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "dynamic message");
}

#[test]
fn test_result_with_context_closure_called_only_on_error() {
    let mut called = false;
    let result: core::result::Result<i32, std::io::Error> = Ok(42);

    let gerr_result: Result<i32, NoID> = result.with_context(|| {
        called = true;
        "should not be called"
    });

    assert!(!called);
    assert_eq!(gerr_result.unwrap(), 42);
}

#[test]
fn test_result_with_context_closure_formatting() {
    let error_code = 404;
    let result: core::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "not found",
    ));

    let gerr_result: Result<i32, NoID> =
        result.with_context(|| format!("error code: {}", error_code));

    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "error code: 404");
}

#[test]
fn test_result_with_context_owned_string_return() {
    let result: core::result::Result<i32, std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "error"));

    let gerr_result: Result<i32, NoID> = result.with_context(|| String::from("allocated message"));

    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "allocated message");
}

#[test]
fn test_result_with_context_with_prefix() {
    let result: core::result::Result<i32, std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "error"));

    let gerr_result: Result<i32, NoID, TestPrefix> = result.with_context(|| "contextual error");

    let err = gerr_result.unwrap_err();
    assert_eq!(err.prefix(), Some("[TEST]"));
    assert_eq!(err.message(), "contextual error");
}

// ============================================================================
// Tests for ResultExt trait: into_gerr()
// ============================================================================

#[test]
fn test_result_into_gerr_basic() {
    let result: core::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "file not found",
    ));

    let gerr_result: Result<i32, NoID> = result.into_gerr();

    assert!(gerr_result.is_err());
    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "file not found");
}

#[test]
fn test_result_into_gerr_with_autogen_id() {
    let result: core::result::Result<i32, std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "error"));

    let gerr_result: Result<i32, AutoId> = result.into_gerr();

    assert!(gerr_result.is_err());
    let err = gerr_result.unwrap_err();
    // ID should be auto-generated
    assert!(err.id().0.to_string().len() > 0);
}

#[test]
fn test_result_into_gerr_success_case() {
    let result: core::result::Result<i32, std::io::Error> = Ok(42);

    let gerr_result: Result<i32, NoID> = result.into_gerr();

    assert!(gerr_result.is_ok());
    assert_eq!(gerr_result.unwrap(), 42);
}

#[test]
fn test_result_into_gerr_error_message_extracted() {
    use std::num::ParseIntError;

    let parse_result: core::result::Result<i32, ParseIntError> = "not a number".parse();
    let gerr_result: Result<i32, NoID> = parse_result.into_gerr();

    let err = gerr_result.unwrap_err();
    assert!(!err.message().is_empty());
}

#[test]
fn test_result_into_gerr_with_prefix() {
    let result: core::result::Result<i32, std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "error"));

    let gerr_result: Result<i32, NoID, TestPrefix> = result.into_gerr();

    let err = gerr_result.unwrap_err();
    assert_eq!(err.prefix(), Some("[TEST]"));
}

// ============================================================================
// Tests for GResultExt trait: gerr()
// ============================================================================

#[test]
fn test_result_gerr_with_gerr_source() {
    let source_err: GErr<u32> = GErr::new_with_id(100, "source error");
    let result: core::result::Result<i32, GErrSource> = Err(source_err.into());

    let gerr_result: Result<i32, NoID> = result.gerr("wrapped with gerr");

    assert!(gerr_result.is_err());
    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "wrapped with gerr");
    assert!(err.sources().is_some());
}

#[test]
fn test_result_gerr_preserves_gerr_source_details() {
    let source_err: GErr<u32, TestPrefix, TestData> = GErr::new_with_id(100, "detailed error")
        .set_prefix("[SOURCE]")
        .set_help("source help")
        .add_tag("source_tag")
        .set_data(TestData { code: 100 });

    let result: core::result::Result<i32, GErrSource> = Err(source_err.into());
    let gerr_result: Result<i32, NoID> = result.gerr("wrapper message");

    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "wrapper message");
    // Source should preserve its details
    assert!(err.sources().is_some());
}

#[test]
fn test_result_gerr_success_case() {
    let result: core::result::Result<i32, GErrSource> = Ok(42);

    let gerr_result: Result<i32, NoID> = result.gerr("should not be used");

    assert!(gerr_result.is_ok());
    assert_eq!(gerr_result.unwrap(), 42);
}

#[test]
fn test_result_gerr_with_autogen_id() {
    let source_err: GErr<u32> = GErr::new_with_id(100, "error");
    let result: core::result::Result<i32, GErrSource> = Err(source_err.into());

    let gerr_result: Result<i32, AutoId> = result.gerr("wrapped");

    let err = gerr_result.unwrap_err();
    assert!(err.id().0.to_string().len() > 0);
}

#[test]
fn test_result_gerr_with_prefix() {
    let source_err: GErr<u32> = GErr::new_with_id(100, "error");
    let result: core::result::Result<i32, GErrSource> = Err(source_err.into());

    let gerr_result: Result<i32, NoID, TestPrefix> = result.gerr("wrapped");

    let err = gerr_result.unwrap_err();
    assert_eq!(err.prefix(), Some("[TEST]"));
}

// ============================================================================
// Tests for GResultExt trait: with_gerr()
// ============================================================================

#[test]
fn test_result_with_gerr_basic() {
    let source_err: GErr<u32> = GErr::new_with_id(100, "source error");
    let result: core::result::Result<i32, GErrSource> = Err(source_err.into());

    let gerr_result: Result<i32, NoID> = result.with_gerr(|| "dynamic gerr message");

    assert!(gerr_result.is_err());
    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "dynamic gerr message");
}

#[test]
fn test_result_with_gerr_closure_not_called_on_success() {
    let mut called = false;
    let result: core::result::Result<i32, GErrSource> = Ok(42);

    let gerr_result: Result<i32, NoID> = result.with_gerr(|| {
        called = true;
        "should not be called"
    });

    assert!(!called);
    assert_eq!(gerr_result.unwrap(), 42);
}

#[test]
fn test_result_with_gerr_closure_with_formatting() {
    let error_code = 500;
    let source_err: GErr<u32> = GErr::new_with_id(100, "error");
    let result: core::result::Result<i32, GErrSource> = Err(source_err.into());

    let gerr_result: Result<i32, NoID> = result.with_gerr(|| format!("error code: {}", error_code));

    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "error code: 500");
}

#[test]
fn test_result_with_gerr_owned_string() {
    let source_err: GErr<u32> = GErr::new_with_id(100, "error");
    let result: core::result::Result<i32, GErrSource> = Err(source_err.into());

    let gerr_result: Result<i32, NoID> = result.with_gerr(|| String::from("allocated message"));

    let err = gerr_result.unwrap_err();
    assert_eq!(err.message(), "allocated message");
}

#[test]
fn test_result_with_gerr_with_prefix() {
    let source_err: GErr<u32> = GErr::new_with_id(100, "error");
    let result: core::result::Result<i32, GErrSource> = Err(source_err.into());

    let gerr_result: Result<i32, NoID, TestPrefix> = result.with_gerr(|| "gerr message");

    let err = gerr_result.unwrap_err();
    assert_eq!(err.prefix(), Some("[TEST]"));
}

// ============================================================================
// Tests for GErrSource::from_error()
// ============================================================================

#[test]
fn test_gerr_source_from_standard_error() {
    let std_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let source = GErrSource::from_error(std_err);

    assert_eq!(source.message, "file not found");
}

#[test]
fn test_gerr_source_from_custom_error() {
    #[derive(Debug)]
    struct CustomError {
        msg: &'static str,
    }

    impl core::fmt::Display for CustomError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Custom: {}", self.msg)
        }
    }

    impl std::error::Error for CustomError {}

    let custom_err = CustomError { msg: "problem" };
    let source = GErrSource::from_error(custom_err);

    assert_eq!(source.message, "Custom: problem");
}

#[test]
fn test_gerr_source_from_parse_error() {
    use std::num::ParseIntError;

    let parse_err: ParseIntError = "not a number".parse::<i32>().unwrap_err();
    let source = GErrSource::from_error(parse_err);

    assert!(!source.message.is_empty());
}

#[test]
fn test_gerr_source_from_gerr_source() {
    let original_source = GErrSource::from_error(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "not found",
    ));
    let converted_source = GErrSource::from_error(original_source);

    assert_eq!(converted_source.message, "not found");
}

#[test]
fn test_gerr_source_display() {
    let source = GErrSource::from_error(std::io::Error::new(
        std::io::ErrorKind::Other,
        "error message",
    ));

    let display_str = format!("{}", source);
    assert_eq!(display_str, "error message");
}

#[test]
fn test_gerr_source_display_with_prefix() {
    let err: GErr<u32> = GErr::new_with_id(42, "error message").set_prefix("[WARN]");
    let source: GErrSource = err.into();

    let display_str = format!("{}", source);
    assert!(display_str.contains("error message"));
}

// ============================================================================
// Tests for GErr Error trait implementation
// ============================================================================

#[test]
fn test_gerr_error_trait_source() {
    let err: GErr<NoID> = GErr::new("parent error").add_source(std::io::Error::new(
        std::io::ErrorKind::Other,
        "child error",
    ));

    // Error::source() should return the first source
    let source = std::error::Error::source(&err);
    assert!(source.is_some());
}

#[test]
fn test_gerr_error_trait_source_chain() {
    use std::error::Error;

    let err: GErr<NoID> = GErr::new("parent")
        .add_source(std::io::Error::new(std::io::ErrorKind::Other, "child1"))
        .add_source(std::io::Error::new(std::io::ErrorKind::Other, "child2"));

    // Should return first source
    let first_source = Error::source(&err);
    assert!(first_source.is_some());
}

// ============================================================================
// Tests for complex conversion chains
// ============================================================================

#[test]
fn test_conversion_chain_standard_error_to_gerr_to_gerr_source() {
    let std_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let result: core::result::Result<i32, std::io::Error> = Err(std_err);

    let gerr_result: Result<i32, NoID> = result.context("wrapped in gerr");
    assert!(gerr_result.is_err());

    let gerr = gerr_result.unwrap_err();
    let source: GErrSource = gerr.into();

    assert_eq!(source.message, "wrapped in gerr");
    assert!(source.sources.is_some());
}

#[test]
fn test_conversion_chain_result_ext_to_gerr_ext() {
    let std_err = std::io::Error::new(std::io::ErrorKind::Other, "error");
    let result1: core::result::Result<i32, std::io::Error> = Err(std_err);

    let gerr_result1: Result<i32> = result1.context("step 1");

    assert!(gerr_result1.is_err());

    let inner_err = gerr_result1.unwrap_err().into_gerr_source();
    let result2: core::result::Result<i32, GErrSource> = Err(inner_err);

    let gerr_result2: Result<i32, NoID> = result2.gerr("step 2");
    let final_err = gerr_result2.unwrap_err();

    assert_eq!(final_err.message(), "step 2");
    assert!(final_err.sources().is_some());
}

#[test]
fn test_conversion_with_type_parameters() {
    let std_err = std::io::Error::new(std::io::ErrorKind::Other, "error");
    let result: core::result::Result<i32, std::io::Error> = Err(std_err);

    let gerr_result: Result<i32, NoID, TestPrefix, TestData> = result.context("error occurred");

    let err = gerr_result.unwrap_err();
    assert_eq!(err.prefix(), Some("[TEST]"));
}

#[test]
fn test_conversion_preserves_error_details() {
    let detail_err: GErr<u32, TestPrefix, TestData> = GErr::new_with_id(42, "detailed error")
        .set_prefix("[DETAIL]")
        .set_help("help text")
        .add_tag("important")
        .set_data(TestData { code: 100 });

    let result: core::result::Result<i32, GErrSource> = Err(detail_err.into());
    let gerr_result: Result<i32, NoID> = result.gerr("wrapper");

    let final_err = gerr_result.unwrap_err();
    assert_eq!(final_err.message(), "wrapper");
    // Source should have the detailed information
    assert!(final_err.sources().is_some());
}

// ============================================================================
// Tests for From/Into implementations
// ============================================================================

#[test]
fn test_display_trait_on_gerr() {
    let err: GErr<NoID> = GErr::new("error message");

    let display_str = format!("{}", err);
    assert_eq!(display_str, "error message");
}

#[test]
fn test_display_trait_on_gerr_source() {
    let source = GErrSource::from_error(std::io::Error::new(
        std::io::ErrorKind::Other,
        "source error",
    ));

    let display_str = format!("{}", source);
    assert_eq!(display_str, "source error");
}

#[test]
fn test_debug_trait_on_gerr() {
    let err: GErr<NoID> = GErr::new("debug test");
    let debug_str = format!("{:?}", err);

    assert!(debug_str.contains("Err"));
    assert!(debug_str.contains("message"));
}

#[test]
fn test_debug_trait_on_gerr_source() {
    let source =
        GErrSource::from_error(std::io::Error::new(std::io::ErrorKind::Other, "debug test"));

    let debug_str = format!("{:?}", source);
    assert!(debug_str.contains("GErrSource"));
}

// ============================================================================
// Tests for Type compatibility
// ============================================================================

#[test]
fn test_result_type_conversion_compatibility() {
    let std_result: core::result::Result<u32, std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "error"));

    let gerr_result: Result<u32, NoID, NoPrefix, NoData> = std_result.into_gerr();

    assert!(gerr_result.is_err());
}

#[test]
fn test_result_type_conversion_with_custom_types() {
    let std_result: core::result::Result<String, std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "error"));

    let gerr_result: Result<String, AutoId, TestPrefix, TestData> = std_result.context("wrapped");

    let err = gerr_result.unwrap_err();
    assert_eq!(err.prefix(), Some("[TEST]"));
}

#[test]
fn test_multiple_error_types_conversion() {
    use std::num::ParseIntError;

    let parse_result: core::result::Result<i32, ParseIntError> = "invalid".parse();
    let gerr_result: Result<i32, NoID> = parse_result.into_gerr();

    assert!(gerr_result.is_err());
}

// ============================================================================
// Tests for Edge cases
// ============================================================================

#[test]
fn test_empty_message_conversion() {
    let std_err = std::io::Error::new(std::io::ErrorKind::Other, "");
    let source = GErrSource::from_error(std_err);

    assert_eq!(source.message, "");
}

#[test]
fn test_very_long_message_conversion() {
    let long_msg = "a".repeat(10000);
    let std_err = std::io::Error::new(std::io::ErrorKind::Other, long_msg.clone());
    let source = GErrSource::from_error(std_err);

    assert_eq!(source.message.len(), 10000);
}

#[test]
fn test_special_characters_in_conversion() {
    let special_msg = "Error: ñ, 中文, 🦀, Ελληνικά";
    let std_err = std::io::Error::new(std::io::ErrorKind::Other, special_msg);
    let source = GErrSource::from_error(std_err);

    assert_eq!(source.message.as_ref(), special_msg);
}

#[test]
fn test_conversion_with_nested_sources() {
    let inner_err = std::io::Error::new(std::io::ErrorKind::NotFound, "inner");
    let result1: core::result::Result<i32, std::io::Error> = Err(inner_err);

    let gerr1: Result<i32, u32> = result1.into_gerr_with_id(4);

    assert!(gerr1.is_err());

    let source1: GErrSource = gerr1.unwrap_err().into_gerr_source();
    let result2: core::result::Result<i32, GErrSource> = Err(source1);

    let gerr2: Result<i32, AutoId> = result2.gerr("level 2");

    let err = gerr2.unwrap_err();
    assert_eq!(err.message(), "level 2");
    assert!(err.sources().is_some());
}

// location tests

#[test]
fn test_result_context_location() {
    let result: core::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "not found",
    ));

    let line = line!();
    let gerr_result: Result<i32, NoID> = result.context("errored");

    let line = line + 1;
    let column = 49;

    assert!(gerr_result.is_err());
    assert_eq!(gerr_result.as_ref().unwrap_err().location().file(), file!());
    assert_eq!(gerr_result.as_ref().unwrap_err().location().line(), line);
    assert_eq!(
        gerr_result.as_ref().unwrap_err().location().column(),
        column
    );
}

#[test]
fn test_result_with_context_location() {
    let mut called = false;
    let result: core::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "not found",
    ));

    let line = line!();
    let gerr_result: Result<i32, NoID> = result.with_context(|| {
        called = true;
        "should be called"
    });

    let line = line + 1;
    let column = 49;

    assert!(gerr_result.is_err());
    assert_eq!(gerr_result.as_ref().unwrap_err().location().file(), file!());
    assert_eq!(gerr_result.as_ref().unwrap_err().location().line(), line);
    assert_eq!(
        gerr_result.as_ref().unwrap_err().location().column(),
        column
    );
    assert!(called);
}

#[test]
fn test_result_to_gerr_location() {
    let result: core::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "not found",
    ));

    let line = line!();
    let gerr_result: Result<i32, NoID> = result.into_gerr();

    let line = line + 1;
    let column = 49;

    assert!(gerr_result.is_err());
    assert_eq!(gerr_result.as_ref().unwrap_err().location().file(), file!());
    assert_eq!(gerr_result.as_ref().unwrap_err().location().line(), line);
    assert_eq!(
        gerr_result.as_ref().unwrap_err().location().column(),
        column
    );
}

#[test]
fn test_result_to_gerr_with_id_location() {
    let result: core::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "not found",
    ));

    let line = line!();
    let gerr_result: Result<i32, u32> = result.into_gerr_with_id(123);

    let line = line + 1;
    let column = 48;

    assert!(gerr_result.is_err());
    assert_eq!(gerr_result.as_ref().unwrap_err().id(), &123);
    assert_eq!(gerr_result.as_ref().unwrap_err().location().file(), file!());
    assert_eq!(gerr_result.as_ref().unwrap_err().location().line(), line);
    assert_eq!(
        gerr_result.as_ref().unwrap_err().location().column(),
        column
    );
}
