extern crate alloc;
use core::fmt::Debug;
use g_err::{GErr, Id, NoData, NoID, NoPrefix, Prefix};
use uuid::Uuid;

/// Mock ID type for testing
#[derive(Debug, PartialEq, Eq, Clone)]
struct MockId(u32);

impl core::fmt::Display for MockId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MockId({})", self.0)
    }
}

/// Auto-generated ID type for testing
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
#[derive(Debug, PartialEq, Eq)]
struct TestData {
    code: u32,
}

// ============================================================================
// Tests for GErr::new() - with auto-generated ID
// ============================================================================

#[test]
fn test_new_with_autogen_id_string_message() {
    let err: GErr<NoID> = GErr::new("test error message");

    assert_eq!(err.message(), "test error message");
    assert_eq!(err.prefix(), None);
    assert!(err.data().is_none());
    assert_eq!(err.help(), None);
    assert_eq!(err.tags(), None);
    assert!(err.sources().is_none());
}

#[test]
fn test_new_with_autogen_id_cow_message() {
    use alloc::borrow::Cow;

    let msg = Cow::Borrowed("borrowed message");
    let err: GErr<NoID> = GErr::new(msg);

    assert_eq!(err.message(), "borrowed message");
}

#[test]
fn test_new_with_autogen_id_owned_string() {
    let msg = String::from("owned message");
    let err: GErr<NoID> = GErr::new(msg);

    assert_eq!(err.message(), "owned message");
}

#[test]
fn test_new_with_autogen_id_custom_id_type() {
    let err1: GErr<AutoId> = GErr::new("error1");
    let err2: GErr<AutoId> = GErr::new("error2");

    // IDs should be different (auto-generated)
    assert_ne!(err1.id().0, err2.id().0);
    assert_eq!(err1.message(), "error1");
    assert_eq!(err2.message(), "error2");
}

#[test]
fn test_new_with_autogen_id_and_prefix() {
    let err: GErr<NoID, TestPrefix> = GErr::new("error with prefix");

    assert_eq!(err.message(), "error with prefix");
    assert_eq!(err.prefix(), Some("[TEST]"));
}

#[test]
fn test_new_location_is_set() {
    let err: GErr<NoID> = GErr::new("test error");
    let location = err.location();

    assert_eq!(location.file(), file!());
    assert!(location.line() > 0);
    assert!(location.column() > 0);
}

// ============================================================================
// Tests for GErr::from_error() - from Error trait
// ============================================================================

#[test]
fn test_from_error_with_standard_error() {
    let std_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");

    let err: GErr<NoID> = GErr::from_error(std_err);

    assert_eq!(err.message(), "file not found");
    assert!(err.sources().is_some());
    assert_eq!(err.sources().unwrap().len(), 1);
}

#[test]
fn test_from_error_message_extracted() {
    use std::num::ParseIntError;

    let parse_err: ParseIntError = "not a number".parse::<i32>().unwrap_err();
    let err: GErr<NoID> = GErr::from_error(parse_err);

    // Message should contain the error string representation
    assert!(!err.message().is_empty());
    assert!(err.sources().is_some());
}

#[test]
fn test_from_error_with_custom_error() {
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

    let custom_err = CustomError {
        msg: "custom problem",
    };
    let err: GErr<NoID> = GErr::from_error(custom_err);

    assert_eq!(err.message(), "Custom: custom problem");
    assert!(err.sources().is_some());
}

#[test]
fn test_from_error_location_is_set() {
    let std_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");

    let err: GErr<NoID> = GErr::from_error(std_err);
    let location = err.location();

    assert_eq!(location.file(), file!());
    assert!(location.line() > 0);
}

#[test]
fn test_from_error_with_autogen_id() {
    let std_err = std::io::Error::new(std::io::ErrorKind::Other, "some error");

    let err1: GErr<AutoId> = GErr::from_error(std_err);
    let err2: GErr<AutoId> = GErr::from_error(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "some error",
    )));

    // IDs should be different
    assert_ne!(err1.id().0, err2.id().0);
}

// ============================================================================
// Tests for GErr::new_with_id() - with manual ID
// ============================================================================

#[test]
fn test_new_with_id_integer() {
    let err: GErr<u32> = GErr::new_with_id(42, "error with numeric id");

    assert_eq!(err.id(), &42);
    assert_eq!(err.message(), "error with numeric id");
}

#[test]
fn test_new_with_id_string() {
    let err: GErr<&'static str> = GErr::new_with_id("ERR_001", "named error");

    assert_eq!(err.id(), &"ERR_001");
    assert_eq!(err.message(), "named error");
}

#[test]
fn test_new_with_id_custom_struct() {
    let err: GErr<MockId> = GErr::new_with_id(MockId(123), "error with custom id");

    assert_eq!(err.id(), &MockId(123));
    assert_eq!(err.message(), "error with custom id");
}

#[test]
fn test_new_with_id_different_ids() {
    let err1: GErr<u32> = GErr::new_with_id(1, "error 1");
    let err2: GErr<u32> = GErr::new_with_id(2, "error 2");

    assert_eq!(err1.id(), &1);
    assert_eq!(err2.id(), &2);
    assert_ne!(err1.id(), err2.id());
}

#[test]
fn test_new_with_id_message_types() {
    use alloc::borrow::Cow;

    // Test with string slice
    let err1: GErr<u32> = GErr::new_with_id(1, "static message");
    assert_eq!(err1.message(), "static message");

    // Test with owned String
    let msg = String::from("owned message");
    let err2: GErr<u32> = GErr::new_with_id(2, msg);
    assert_eq!(err2.message(), "owned message");

    // Test with Cow
    let msg = Cow::Borrowed("cow message");
    let err3: GErr<u32> = GErr::new_with_id(3, msg);
    assert_eq!(err3.message(), "cow message");
}

#[test]
fn test_new_with_id_and_prefix() {
    let err: GErr<u32, TestPrefix> = GErr::new_with_id(99, "error with prefix");

    assert_eq!(err.id(), &99);
    assert_eq!(err.message(), "error with prefix");
    assert_eq!(err.prefix(), Some("[TEST]"));
}

#[test]
fn test_new_with_id_location_is_set() {
    let err: GErr<u32> = GErr::new_with_id(1, "test error");
    let location = err.location();

    assert_eq!(location.file(), file!());
    assert!(location.line() > 0);
}

// ============================================================================
// Tests for internal methods: new_untracked() and with_id_untracked()
// ============================================================================

#[test]
fn test_new_untracked_basic() {
    let err: GErr<NoID> = GErr::new("untracked error");

    assert_eq!(err.message(), "untracked error");
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.prefix(), None);
}

#[test]
fn test_with_id_untracked_basic() {
    let err: GErr<u32> = GErr::new_with_id(42, "untracked with id");

    assert_eq!(err.id(), &42);
    assert_eq!(err.message(), "untracked with id");
    assert_eq!(err.location().file(), file!());
}

#[test]
fn test_with_id_untracked_custom_location() {
    let err: GErr<&str> = GErr::new_with_id("ERR_CODE", "error with custom location");

    assert_eq!(err.id(), &"ERR_CODE");
    assert_eq!(err.message(), "error with custom location");
    assert_eq!(err.location().file(), file!());
}

// ============================================================================
// Tests for constructor variations with different generic parameters
// ============================================================================

#[test]
fn test_constructor_no_id_no_prefix_no_data() {
    let err: GErr<NoID, NoPrefix, NoData> = GErr::new("basic error");

    assert_eq!(err.message(), "basic error");
    assert_eq!(err.prefix(), None);
    assert!(err.data().is_none());
}

#[test]
fn test_constructor_with_id_type_only() {
    let err: GErr<u64> = GErr::new_with_id(999, "error with id");

    assert_eq!(err.id(), &999);
    assert_eq!(err.message(), "error with id");
    assert_eq!(err.prefix(), None);
    assert_eq!(err.data().is_none(), true);
}

#[test]
fn test_constructor_with_id_and_prefix() {
    let err: GErr<u32, TestPrefix> = GErr::new_with_id(42, "error with id and prefix");

    assert_eq!(err.id(), &42);
    assert_eq!(err.message(), "error with id and prefix");
    assert_eq!(err.prefix(), Some("[TEST]"));
    assert!(err.data().is_none());
}

#[test]
fn test_constructor_with_id_prefix_and_data() {
    let err: GErr<u32, TestPrefix, TestData> =
        GErr::new_with_id(42, "error with id, prefix, and data");

    assert_eq!(err.id(), &42);
    assert_eq!(err.message(), "error with id, prefix, and data");
    assert_eq!(err.prefix(), Some("[TEST]"));
    assert_eq!(err.data(), None); // Not set during construction
}

#[test]
fn test_constructor_autogen_id_different_each_time() {
    let err1: GErr<AutoId> = GErr::new("first");
    let err2: GErr<AutoId> = GErr::new("second");
    let err3: GErr<AutoId> = GErr::new("third");

    // All IDs should be unique
    assert_ne!(err1.id().0, err2.id().0);
    assert_ne!(err2.id().0, err3.id().0);
    assert_ne!(err1.id().0, err3.id().0);
}

// ============================================================================
// Tests for message handling edge cases
// ============================================================================

#[test]
fn test_new_with_empty_message() {
    let err: GErr<NoID> = GErr::new("");

    assert_eq!(err.message(), "");
}

#[test]
fn test_new_with_very_long_message() {
    let long_msg = "a".repeat(1000);
    let err: GErr<NoID> = GErr::new(long_msg.clone());

    assert_eq!(err.message(), long_msg.as_str());
    assert_eq!(err.message().len(), 1000);
}

#[test]
fn test_new_with_special_characters_in_message() {
    let special_msg = "Error: ñ, 中文, Ελληνικά, 🦀";
    let err: GErr<NoID> = GErr::new(special_msg);

    assert_eq!(err.message(), special_msg);
}

#[test]
fn test_new_with_newlines_in_message() {
    let multiline_msg = "Line 1\nLine 2\nLine 3";
    let err: GErr<NoID> = GErr::new(multiline_msg);

    assert_eq!(err.message(), multiline_msg);
}

// ============================================================================
// Tests for combining constructors with builder methods
// ============================================================================

#[test]
fn test_new_then_set_prefix() {
    let err: GErr<NoID> = GErr::new("error").set_prefix("[HTTP]");

    assert_eq!(err.message(), "error");
    assert_eq!(err.prefix(), Some("[HTTP]"));
}

#[test]
fn test_new_with_id_then_set_data() {
    let err: GErr<u32, NoPrefix, TestData> =
        GErr::new_with_id(42, "error").set_data(TestData { code: 404 });

    assert_eq!(err.id(), &42);
    assert_eq!(err.message(), "error");
    assert_eq!(err.data(), Some(&TestData { code: 404 }));
}

#[test]
fn test_new_then_add_help() {
    let err: GErr<NoID> = GErr::new("error").set_help("Try restarting the service");

    assert_eq!(err.message(), "error");
    assert_eq!(err.help(), Some("Try restarting the service"));
}

#[test]
fn test_from_error_then_add_tag() {
    let std_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");

    let err: GErr<NoID> = GErr::from_error(std_err)
        .add_tag("filesystem")
        .add_tag("critical");

    assert_eq!(err.message(), "file not found");
    assert_eq!(err.tags().unwrap().len(), 2);
}

#[test]
fn test_constructor_chain_all_modifiers() {
    let err: GErr<u32, TestPrefix, TestData> = GErr::new_with_id(42, "complex error")
        .set_data(TestData { code: 500 })
        .set_help("Contact support")
        .add_tag("server")
        .add_tag("error");

    assert_eq!(err.id(), &42);
    assert_eq!(err.message(), "complex error");
    assert_eq!(err.prefix(), Some("[TEST]"));
    assert_eq!(err.data(), Some(&TestData { code: 500 }));
    assert_eq!(err.help(), Some("Contact support"));
    assert_eq!(err.tags().unwrap().len(), 2);
}

// ============================================================================
// Tests for Display trait with different constructors
// ============================================================================

#[test]
fn test_display_new_without_prefix() {
    let err: GErr<NoID> = GErr::new("error message");

    assert_eq!(format!("{}", err), "error message");
}

#[test]
fn test_display_new_with_prefix() {
    let err: GErr<NoID, TestPrefix> = GErr::new("error message");

    assert_eq!(format!("{}", err), "[TEST] error message");
}

#[test]
fn test_display_new_with_id_set_prefix() {
    let err: GErr<u32> = GErr::new_with_id(42, "error").set_prefix("[APP]");

    assert_eq!(format!("{}", err), "[APP] error");
}

// ============================================================================
// Tests for Debug trait with different constructors
// ============================================================================

#[test]
fn test_debug_impl_has_fields() {
    let err: GErr<NoID> = GErr::new("test error");
    let debug_str = format!("{:?}", err);

    // Debug should include key fields
    assert!(debug_str.contains("Err"));
    assert!(debug_str.contains("message"));
    assert!(debug_str.contains("test error"));
}

#[test]
fn test_debug_with_all_generics() {
    let err: GErr<u32, TestPrefix, TestData> =
        GErr::new_with_id(42, "error").set_data(TestData { code: 404 });

    let debug_str = format!("{:?}", err);

    assert!(debug_str.contains("Err"));
    assert!(debug_str.contains("42"));
    assert!(debug_str.contains("error"));
}
