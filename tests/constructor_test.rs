// extern crate alloc;
// use core::fmt::Debug;
// use g_err::{GErr, Id, NoData, NoID, NoPrefix, Prefix};
// use uuid::Uuid;

// /// Mock ID type for testing
// #[derive(Debug, PartialEq, Eq, Clone)]
// struct MockId(u32);

// impl core::fmt::Display for MockId {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "MockId({})", self.0)
//     }
// }

// /// Auto-generated ID type for testing
// #[derive(Debug, PartialEq, Eq, Clone)]
// struct AutoId(Uuid);

// impl core::fmt::Display for AutoId {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "AutoId({})", self.0)
//     }
// }

// impl Id for AutoId {
//     fn id() -> Self {
//         AutoId(Uuid::new_v4())
//     }
// }

// /// Prefix type for testing
// struct TestPrefix;
// impl Prefix for TestPrefix {
//     const PREFIX: Option<&'static str> = Some("[TEST]");
// }

// /// Data type for testing
// #[derive(Debug, PartialEq, Eq, Clone)]
// struct TestData {
//     code: u32,
// }

// // ============================================================================
// // Tests for GErr::new() - with auto-generated ID
// // ============================================================================

// #[test]
// fn test_new_with_autogen_id_string_message() {
//     let err: GErr<NoID> = GErr::new("test error message");

//     assert_eq!(err.message(), "test error message");
//     assert_eq!(err.prefix(), None);
//     assert!(err.data().is_none());
//     assert_eq!(err.help(), None);
//     assert_eq!(err.tags(), None);
//     assert!(err.sources().is_none());
// }

// #[test]
// fn test_new_with_autogen_id_cow_message() {
//     use alloc::borrow::Cow;

//     let msg = Cow::Borrowed("borrowed message");
//     let err: GErr<NoID> = GErr::new(msg);

//     assert_eq!(err.message(), "borrowed message");
// }

// #[test]
// fn test_new_with_autogen_id_owned_string() {
//     let msg = String::from("owned message");
//     let err: GErr<NoID> = GErr::new(msg);

//     assert_eq!(err.message(), "owned message");
// }

// #[test]
// fn test_new_with_autogen_id_custom_id_type() {
//     let err1: GErr<AutoId> = GErr::new("error1");
//     let err2: GErr<AutoId> = GErr::new("error2");

//     // IDs should be different (auto-generated)
//     assert_ne!(err1.id().0, err2.id().0);
//     assert_eq!(err1.message(), "error1");
//     assert_eq!(err2.message(), "error2");
// }

// #[test]
// fn test_new_with_autogen_id_and_prefix() {
//     let err: GErr<NoID, TestPrefix> = GErr::new("error with prefix");

//     assert_eq!(err.message(), "error with prefix");
//     assert_eq!(err.prefix(), Some("[TEST]"));
// }

// #[test]
// fn test_new_location_is_set() {
//     let err: GErr<NoID> = GErr::new("test error");
//     let location = err.location();

//     assert_eq!(location.file(), file!());
//     assert!(location.line() > 0);
//     assert!(location.column() > 0);
// }

// // ============================================================================
// // Tests for GErr::from_error() - from Error trait
// // ============================================================================

// #[test]
// fn test_from_error_with_standard_error() {
//     let std_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");

//     let err: GErr<NoID> = GErr::from_error(std_err);

//     assert_eq!(err.message(), "file not found");
//     assert!(err.sources().is_some());
//     assert_eq!(err.sources().unwrap().len(), 1);
// }

// #[test]
// fn test_from_error_message_extracted() {
//     use std::num::ParseIntError;

//     let parse_err: ParseIntError = "not a number".parse::<i32>().unwrap_err();
//     let err: GErr<NoID> = GErr::from_error(parse_err);

//     // Message should contain the error string representation
//     assert!(!err.message().is_empty());
//     assert!(err.sources().is_some());
// }

// #[test]
// fn test_from_error_with_custom_error() {
//     #[derive(Debug)]
//     struct CustomError {
//         msg: &'static str,
//     }

//     impl core::fmt::Display for CustomError {
//         fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//             write!(f, "Custom: {}", self.msg)
//         }
//     }

//     impl std::error::Error for CustomError {}

//     let custom_err = CustomError {
//         msg: "custom problem",
//     };
//     let err: GErr<NoID> = GErr::from_error(custom_err);

//     assert_eq!(err.message(), "Custom: custom problem");
//     assert!(err.sources().is_some());
// }

// #[test]
// fn test_from_error_location_is_set() {
//     let std_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");

//     let err: GErr<NoID> = GErr::from_error(std_err);
//     let location = err.location();

//     assert_eq!(location.file(), file!());
//     assert!(location.line() > 0);
// }

// #[test]
// fn test_from_error_with_autogen_id() {
//     let std_err = std::io::Error::new(std::io::ErrorKind::Other, "some error");

//     let err1: GErr<AutoId> = GErr::from_error(std_err);
//     let err2: GErr<AutoId> = GErr::from_error(Box::new(std::io::Error::new(
//         std::io::ErrorKind::Other,
//         "some error",
//     )));

//     // IDs should be different
//     assert_ne!(err1.id().0, err2.id().0);
// }

// // ============================================================================
// // Tests for GErr::new_with_id() - with manual ID
// // ============================================================================

// #[test]
// fn test_new_with_id_integer() {
//     let err: GErr<u32> = GErr::new_with_id(42, "error with numeric id");

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error with numeric id");
// }

// #[test]
// fn test_new_with_id_string() {
//     let err: GErr<&'static str> = GErr::new_with_id("ERR_001", "named error");

//     assert_eq!(err.id(), &"ERR_001");
//     assert_eq!(err.message(), "named error");
// }

// #[test]
// fn test_new_with_id_custom_struct() {
//     let err: GErr<MockId> = GErr::new_with_id(MockId(123), "error with custom id");

//     assert_eq!(err.id(), &MockId(123));
//     assert_eq!(err.message(), "error with custom id");
// }

// #[test]
// fn test_new_with_id_different_ids() {
//     let err1: GErr<u32> = GErr::new_with_id(1, "error 1");
//     let err2: GErr<u32> = GErr::new_with_id(2, "error 2");

//     assert_eq!(err1.id(), &1);
//     assert_eq!(err2.id(), &2);
//     assert_ne!(err1.id(), err2.id());
// }

// #[test]
// fn test_new_with_id_message_types() {
//     use alloc::borrow::Cow;

//     // Test with string slice
//     let err1: GErr<u32> = GErr::new_with_id(1, "static message");
//     assert_eq!(err1.message(), "static message");

//     // Test with owned String
//     let msg = String::from("owned message");
//     let err2: GErr<u32> = GErr::new_with_id(2, msg);
//     assert_eq!(err2.message(), "owned message");

//     // Test with Cow
//     let msg = Cow::Borrowed("cow message");
//     let err3: GErr<u32> = GErr::new_with_id(3, msg);
//     assert_eq!(err3.message(), "cow message");
// }

// #[test]
// fn test_new_with_id_and_prefix() {
//     let err: GErr<u32, TestPrefix> = GErr::new_with_id(99, "error with prefix");

//     assert_eq!(err.id(), &99);
//     assert_eq!(err.message(), "error with prefix");
//     assert_eq!(err.prefix(), Some("[TEST]"));
// }

// #[test]
// fn test_new_with_id_location_is_set() {
//     let err: GErr<u32> = GErr::new_with_id(1, "test error");
//     let location = err.location();

//     assert_eq!(location.file(), file!());
//     assert!(location.line() > 0);
// }

// // ============================================================================
// // Tests for internal methods: new_untracked() and with_id_untracked()
// // ============================================================================

// #[test]
// fn test_new_untracked_basic() {
//     let err: GErr<NoID> = GErr::new("untracked error");

//     assert_eq!(err.message(), "untracked error");
//     assert_eq!(err.location().file(), file!());
//     assert_eq!(err.prefix(), None);
// }

// #[test]
// fn test_with_id_untracked_basic() {
//     let err: GErr<u32> = GErr::new_with_id(42, "untracked with id");

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "untracked with id");
//     assert_eq!(err.location().file(), file!());
// }

// #[test]
// fn test_with_id_untracked_custom_location() {
//     let err: GErr<&str> = GErr::new_with_id("ERR_CODE", "error with custom location");

//     assert_eq!(err.id(), &"ERR_CODE");
//     assert_eq!(err.message(), "error with custom location");
//     assert_eq!(err.location().file(), file!());
// }

// // ============================================================================
// // Tests for constructor variations with different generic parameters
// // ============================================================================

// #[test]
// fn test_constructor_no_id_no_prefix_no_data() {
//     let err: GErr<NoID, NoPrefix, NoData> = GErr::new("basic error");

//     assert_eq!(err.message(), "basic error");
//     assert_eq!(err.prefix(), None);
//     assert!(err.data().is_none());
// }

// #[test]
// fn test_constructor_with_id_type_only() {
//     let err: GErr<u64> = GErr::new_with_id(999, "error with id");

//     assert_eq!(err.id(), &999);
//     assert_eq!(err.message(), "error with id");
//     assert_eq!(err.prefix(), None);
//     assert_eq!(err.data().is_none(), true);
// }

// #[test]
// fn test_constructor_with_id_and_prefix() {
//     let err: GErr<u32, TestPrefix> = GErr::new_with_id(42, "error with id and prefix");

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error with id and prefix");
//     assert_eq!(err.prefix(), Some("[TEST]"));
//     assert!(err.data().is_none());
// }

// #[test]
// fn test_constructor_with_id_prefix_and_data() {
//     let err: GErr<u32, TestPrefix, TestData> =
//         GErr::new_with_id(42, "error with id, prefix, and data");

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error with id, prefix, and data");
//     assert_eq!(err.prefix(), Some("[TEST]"));
//     assert_eq!(err.data(), None);
// }

// #[test]
// fn test_constructor_autogen_id_different_each_time() {
//     let err1: GErr<AutoId> = GErr::new("first");
//     let err2: GErr<AutoId> = GErr::new("second");
//     let err3: GErr<AutoId> = GErr::new("third");

//     // All IDs should be unique
//     assert_ne!(err1.id().0, err2.id().0);
//     assert_ne!(err2.id().0, err3.id().0);
//     assert_ne!(err1.id().0, err3.id().0);
// }

// // ============================================================================
// // Tests for message handling edge cases
// // ============================================================================

// #[test]
// fn test_new_with_empty_message() {
//     let err: GErr<NoID> = GErr::new("");

//     assert_eq!(err.message(), "");
// }

// #[test]
// fn test_new_with_very_long_message() {
//     let long_msg = "a".repeat(1000);
//     let err: GErr<NoID> = GErr::new(long_msg.clone());

//     assert_eq!(err.message(), long_msg.as_str());
//     assert_eq!(err.message().len(), 1000);
// }

// #[test]
// fn test_new_with_special_characters_in_message() {
//     let special_msg = "Error: ñ, 中文, Ελληνικά, 🦀";
//     let err: GErr<NoID> = GErr::new(special_msg);

//     assert_eq!(err.message(), special_msg);
// }

// #[test]
// fn test_new_with_newlines_in_message() {
//     let multiline_msg = "Line 1\nLine 2\nLine 3";
//     let err: GErr<NoID> = GErr::new(multiline_msg);

//     assert_eq!(err.message(), multiline_msg);
// }

// // ============================================================================
// // Tests for combining constructors with builder methods
// // ============================================================================

// #[test]
// fn test_new_then_set_prefix() {
//     let err: GErr<NoID> = GErr::new("error").set_prefix("[HTTP]");

//     assert_eq!(err.message(), "error");
//     assert_eq!(err.prefix(), Some("[HTTP]"));
// }

// #[test]
// fn test_new_with_id_then_set_data() {
//     let err: GErr<u32, NoPrefix, TestData> =
//         GErr::new_with_id(42, "error").set_data(TestData { code: 404 });

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error");
//     assert_eq!(err.data(), Some(&TestData { code: 404 }));
// }

// #[test]
// fn test_new_then_add_help() {
//     let err: GErr<NoID> = GErr::new("error").set_help("Try restarting the service");

//     assert_eq!(err.message(), "error");
//     assert_eq!(err.help(), Some("Try restarting the service"));
// }

// #[test]
// fn test_from_error_then_add_tag() {
//     let std_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");

//     let err: GErr<NoID> = GErr::from_error(std_err)
//         .add_tag("filesystem")
//         .add_tag("critical");

//     assert_eq!(err.message(), "file not found");
//     assert_eq!(err.tags().unwrap().len(), 2);
// }

// #[test]
// fn test_constructor_chain_all_modifiers() {
//     let err: GErr<u32, TestPrefix, TestData> = GErr::new_with_id(42, "complex error")
//         .set_data(TestData { code: 500 })
//         .set_help("Contact support")
//         .add_tag("server")
//         .add_tag("error");

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "complex error");
//     assert_eq!(err.prefix(), Some("[TEST]"));
//     assert_eq!(err.data(), Some(&TestData { code: 500 }));
//     assert_eq!(err.help(), Some("Contact support"));
//     assert_eq!(err.tags().unwrap().len(), 2);
// }

// // ============================================================================
// // Tests for Display trait with different constructors
// // ============================================================================

// #[test]
// fn test_display_new_without_prefix() {
//     let err: GErr<NoID> = GErr::new("error message");

//     assert_eq!(format!("{}", err), "error message");
// }

// #[test]
// fn test_display_new_with_prefix() {
//     let err: GErr<NoID, TestPrefix> = GErr::new("error message");

//     assert_eq!(format!("{}", err), "[TEST] error message");
// }

// #[test]
// fn test_display_new_with_id_set_prefix() {
//     let err: GErr<u32> = GErr::new_with_id(42, "error").set_prefix("[APP]");

//     assert_eq!(format!("{}", err), "[APP] error");
// }

// // ============================================================================
// // Tests for Debug trait with different constructors
// // ============================================================================

// #[test]
// fn test_debug_impl_has_fields() {
//     let err: GErr<NoID> = GErr::new("test error");
//     let debug_str = format!("{:?}", err);

//     // Debug should include key fields
//     assert!(debug_str.contains("Err"));
//     assert!(debug_str.contains("message"));
//     assert!(debug_str.contains("test error"));
// }

// #[test]
// fn test_debug_with_all_generics() {
//     let err: GErr<u32, TestPrefix, TestData> =
//         GErr::new_with_id(42, "error").set_data(TestData { code: 404 });

//     let debug_str = format!("{:?}", err);

//     assert!(debug_str.contains("Err"));
//     assert!(debug_str.contains("42"));
//     assert!(debug_str.contains("error"));
// }

// // ============================================================================
// // Tests for builder method: set_id()
// // ============================================================================

// #[test]
// fn test_set_id_basic() {
//     let err: GErr<u32> = GErr::new_with_id(1, "error").set_id(99);

//     assert_eq!(err.id(), &99);
//     assert_eq!(err.message(), "error");
// }

// #[test]
// fn test_set_id_multiple_times() {
//     let err: GErr<u32> = GErr::new_with_id(1, "error").set_id(2).set_id(3).set_id(4);

//     assert_eq!(err.id(), &4);
// }

// #[test]
// fn test_set_id_preserves_other_fields() {
//     let err: GErr<u32> = GErr::new_with_id(1, "error")
//         .set_prefix("[PREFIX]")
//         .set_help("help text")
//         .set_id(99);

//     assert_eq!(err.id(), &99);
//     assert_eq!(err.message(), "error");
//     assert_eq!(err.prefix(), Some("[PREFIX]"));
//     assert_eq!(err.help(), Some("help text"));
// }

// // ============================================================================
// // Tests for builder method: set_prefix()
// // ============================================================================

// #[test]
// fn test_set_prefix_basic() {
//     let err: GErr<NoID> = GErr::new("error").set_prefix("[ERROR]");

//     assert_eq!(err.prefix(), Some("[ERROR]"));
// }

// #[test]
// fn test_set_prefix_override() {
//     let err: GErr<NoID> = GErr::new("error")
//         .set_prefix("[FIRST]")
//         .set_prefix("[SECOND]");

//     assert_eq!(err.prefix(), Some("[SECOND]"));
// }

// #[test]
// fn test_set_prefix_with_string() {
//     let err: GErr<NoID> = GErr::new("error").set_prefix(String::from("[DYNAMIC]"));

//     assert_eq!(err.prefix(), Some("[DYNAMIC]"));
// }

// #[test]
// fn test_set_prefix_empty_string() {
//     let err: GErr<NoID> = GErr::new("error").set_prefix("");

//     assert_eq!(err.prefix(), Some(""));
// }

// #[test]
// fn test_set_prefix_preserves_other_fields() {
//     let err: GErr<u32> = GErr::new_with_id(42, "error")
//         .set_help("help")
//         .set_prefix("[PREFIX]");

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error");
//     assert_eq!(err.prefix(), Some("[PREFIX]"));
//     assert_eq!(err.help(), Some("help"));
// }

// // ============================================================================
// // Tests for builder method: prepend_prefix()
// // ============================================================================

// #[test]
// fn test_prepend_prefix_to_empty() {
//     let err: GErr<NoID> = GErr::new("error").prepend_prefix("[BEFORE]");

//     assert_eq!(err.prefix(), Some("[BEFORE]"));
// }

// #[test]
// fn test_prepend_prefix_to_existing() {
//     let err: GErr<NoID> = GErr::new("error")
//         .set_prefix("[MIDDLE]")
//         .prepend_prefix("[BEFORE]");

//     assert_eq!(err.prefix(), Some("[BEFORE][MIDDLE]"));
// }

// #[test]
// fn test_prepend_prefix_multiple_times() {
//     let err: GErr<NoID> = GErr::new("error")
//         .set_prefix("[C]")
//         .prepend_prefix("[B]")
//         .prepend_prefix("[A]");

//     assert_eq!(err.prefix(), Some("[A][B][C]"));
// }

// #[test]
// fn test_prepend_prefix_with_owned_string() {
//     let err: GErr<NoID> = GErr::new("error")
//         .set_prefix("MIDDLE")
//         .prepend_prefix(String::from("BEFORE_"));

//     assert_eq!(err.prefix(), Some("BEFORE_MIDDLE"));
// }

// #[test]
// fn test_prepend_prefix_preserves_other_fields() {
//     let err: GErr<u32> = GErr::new_with_id(42, "error")
//         .set_prefix("[MIDDLE]")
//         .add_tag("tag1")
//         .prepend_prefix("[BEFORE]");

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error");
//     assert_eq!(err.prefix(), Some("[BEFORE][MIDDLE]"));
//     assert_eq!(err.tags().unwrap().len(), 1);
// }

// // ============================================================================
// // Tests for builder method: append_prefix()
// // ============================================================================

// #[test]
// fn test_append_prefix_to_empty() {
//     let err: GErr<NoID> = GErr::new("error").append_prefix("[AFTER]");

//     assert_eq!(err.prefix(), Some("[AFTER]"));
// }

// #[test]
// fn test_append_prefix_to_existing() {
//     let err: GErr<NoID> = GErr::new("error")
//         .set_prefix("[MIDDLE]")
//         .append_prefix("[AFTER]");

//     assert_eq!(err.prefix(), Some("[MIDDLE][AFTER]"));
// }

// #[test]
// fn test_append_prefix_multiple_times() {
//     let err: GErr<NoID> = GErr::new("error")
//         .set_prefix("[A]")
//         .append_prefix("[B]")
//         .append_prefix("[C]");

//     assert_eq!(err.prefix(), Some("[A][B][C]"));
// }

// #[test]
// fn test_append_prefix_with_owned_string() {
//     let err: GErr<NoID> = GErr::new("error")
//         .set_prefix("FIRST_")
//         .append_prefix(String::from("SECOND"));

//     assert_eq!(err.prefix(), Some("FIRST_SECOND"));
// }

// #[test]
// fn test_prepend_and_append_prefix_combined() {
//     let err: GErr<NoID> = GErr::new("error")
//         .set_prefix("[MIDDLE]")
//         .prepend_prefix("[BEFORE]")
//         .append_prefix("[AFTER]");

//     assert_eq!(err.prefix(), Some("[BEFORE][MIDDLE][AFTER]"));
// }

// #[test]
// fn test_append_prefix_preserves_other_fields() {
//     let err: GErr<u32> = GErr::new_with_id(42, "error")
//         .set_prefix("[MIDDLE]")
//         .set_help("help")
//         .append_prefix("[AFTER]");

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error");
//     assert_eq!(err.prefix(), Some("[MIDDLE][AFTER]"));
//     assert_eq!(err.help(), Some("help"));
// }

// // ============================================================================
// // Tests for builder method: set_data()
// // ============================================================================

// #[test]
// fn test_set_data_basic() {
//     let err: GErr<NoID, NoPrefix, TestData> = GErr::new("error").set_data(TestData { code: 500 });

//     assert_eq!(err.data(), Some(&TestData { code: 500 }));
// }

// #[test]
// fn test_set_data_override() {
//     let err: GErr<NoID, NoPrefix, TestData> = GErr::new("error")
//         .set_data(TestData { code: 400 })
//         .set_data(TestData { code: 500 });

//     assert_eq!(err.data(), Some(&TestData { code: 500 }));
// }

// #[test]
// fn test_set_data_preserves_other_fields() {
//     let err: GErr<u32, NoPrefix, TestData> = GErr::new_with_id(42, "error")
//         .set_prefix("[PREFIX]")
//         .set_help("help")
//         .add_tag("tag")
//         .set_data(TestData { code: 404 });

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error");
//     assert_eq!(err.prefix(), Some("[PREFIX]"));
//     assert_eq!(err.help(), Some("help"));
//     assert_eq!(err.data(), Some(&TestData { code: 404 }));
//     assert_eq!(err.tags().unwrap().len(), 1);
// }

// // ============================================================================
// // Tests for builder method: set_help()
// // ============================================================================

// #[test]
// fn test_set_help_basic() {
//     let err: GErr<NoID> = GErr::new("error").set_help("Try restarting");

//     assert_eq!(err.help(), Some("Try restarting"));
// }

// #[test]
// fn test_set_help_override() {
//     let err: GErr<NoID> = GErr::new("error")
//         .set_help("First help")
//         .set_help("Second help");

//     assert_eq!(err.help(), Some("Second help"));
// }

// #[test]
// fn test_set_help_with_owned_string() {
//     let err: GErr<NoID> = GErr::new("error").set_help(String::from("Dynamic help text"));

//     assert_eq!(err.help(), Some("Dynamic help text"));
// }

// #[test]
// fn test_set_help_empty_string() {
//     let err: GErr<NoID> = GErr::new("error").set_help("");

//     assert_eq!(err.help(), Some(""));
// }

// #[test]
// fn test_set_help_preserves_other_fields() {
//     let err: GErr<u32> = GErr::new_with_id(42, "error")
//         .set_prefix("[PREFIX]")
//         .add_tag("tag")
//         .set_help("helpful text");

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error");
//     assert_eq!(err.prefix(), Some("[PREFIX]"));
//     assert_eq!(err.help(), Some("helpful text"));
//     assert_eq!(err.tags().unwrap().len(), 1);
// }

// // ============================================================================
// // Tests for builder method: add_tag()
// // ============================================================================

// #[test]
// fn test_add_tag_single() {
//     let err: GErr<NoID> = GErr::new("error").add_tag("important");

//     assert_eq!(err.tags().unwrap().len(), 1);
//     assert_eq!(err.tags().unwrap()[0].as_ref(), "important");
// }

// #[test]
// fn test_add_tag_multiple() {
//     let err: GErr<NoID> = GErr::new("error")
//         .add_tag("tag1")
//         .add_tag("tag2")
//         .add_tag("tag3");

//     assert_eq!(err.tags().unwrap().len(), 3);
//     assert_eq!(err.tags().unwrap()[0].as_ref(), "tag1");
//     assert_eq!(err.tags().unwrap()[1].as_ref(), "tag2");
//     assert_eq!(err.tags().unwrap()[2].as_ref(), "tag3");
// }

// #[test]
// fn test_add_tag_with_owned_string() {
//     let err: GErr<NoID> = GErr::new("error")
//         .add_tag("static")
//         .add_tag(String::from("dynamic"));

//     assert_eq!(err.tags().unwrap().len(), 2);
// }

// #[test]
// fn test_add_tag_duplicate_tags() {
//     let err: GErr<NoID> = GErr::new("error")
//         .add_tag("tag")
//         .add_tag("tag")
//         .add_tag("tag");

//     assert_eq!(err.tags().unwrap().len(), 3);
// }

// #[test]
// fn test_add_tag_preserves_other_fields() {
//     let err: GErr<u32> = GErr::new_with_id(42, "error")
//         .set_prefix("[PREFIX]")
//         .set_help("help")
//         .add_tag("tag1")
//         .add_tag("tag2");

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error");
//     assert_eq!(err.prefix(), Some("[PREFIX]"));
//     assert_eq!(err.help(), Some("help"));
//     assert_eq!(err.tags().unwrap().len(), 2);
// }

// // ============================================================================
// // Tests for builder method: add_tags()
// // ============================================================================

// #[test]
// fn test_add_tags_vector() {
//     let tags = vec!["tag1", "tag2", "tag3"];
//     let err: GErr<NoID> = GErr::new("error").add_tags(tags);

//     assert_eq!(err.tags().unwrap().len(), 3);
//     assert_eq!(err.tags().unwrap()[0].as_ref(), "tag1");
//     assert_eq!(err.tags().unwrap()[1].as_ref(), "tag2");
//     assert_eq!(err.tags().unwrap()[2].as_ref(), "tag3");
// }

// #[test]
// fn test_add_tags_array() {
//     let err: GErr<NoID> = GErr::new("error").add_tags(["tag1", "tag2"]);

//     assert_eq!(err.tags().unwrap().len(), 2);
// }

// #[test]
// fn test_add_tags_iterator() {
//     let tags = vec!["a", "b", "c"];
//     let err: GErr<NoID> = GErr::new("error").add_tags(tags.into_iter());

//     assert_eq!(err.tags().unwrap().len(), 3);
// }

// #[test]
// fn test_add_tags_combined_with_add_tag() {
//     let err: GErr<NoID> = GErr::new("error")
//         .add_tag("single")
//         .add_tags(vec!["bulk1", "bulk2"])
//         .add_tag("another");

//     assert_eq!(err.tags().unwrap().len(), 4);
//     assert_eq!(err.tags().unwrap()[0].as_ref(), "single");
//     assert_eq!(err.tags().unwrap()[3].as_ref(), "another");
// }

// #[test]
// fn test_add_tags_empty() {
//     let err: GErr<NoID> = GErr::new("error").add_tags::<Vec<&str>, &str>(vec![]);
//     assert!(err.tags().is_none());
// }

// #[test]
// fn test_add_tags_preserves_other_fields() {
//     let err: GErr<u32> = GErr::new_with_id(42, "error")
//         .set_help("help")
//         .add_tags(vec!["tag1", "tag2"]);

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.message(), "error");
//     assert_eq!(err.help(), Some("help"));
//     assert_eq!(err.tags().unwrap().len(), 2);
// }

// // ============================================================================
// // Tests for builder method: add_source()
// // ============================================================================

// #[test]
// fn test_add_source_single() {
//     let source = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
//     let err: GErr<NoID> = GErr::new("error").add_source(source);

//     assert_eq!(err.sources().unwrap().len(), 1);
// }

// #[test]
// fn test_add_source_multiple() {
//     let err: GErr<NoID> = GErr::new("error")
//         .add_source(std::io::Error::new(
//             std::io::ErrorKind::NotFound,
//             "not found",
//         ))
//         .add_source(std::io::Error::new(
//             std::io::ErrorKind::PermissionDenied,
//             "denied",
//         ))
//         .add_source(std::io::Error::new(std::io::ErrorKind::Other, "other"));

//     assert_eq!(err.sources().unwrap().len(), 3);
// }

// #[test]
// fn test_add_source_chain() {
//     let err: GErr<NoID> = GErr::new("main error").add_source(std::io::Error::new(
//         std::io::ErrorKind::NotFound,
//         "root cause",
//     ));

//     assert!(err.sources().is_some());
//     assert_eq!(err.sources().unwrap().len(), 1);
// }

// #[test]
// fn test_add_source_preserves_other_fields() {
//     let err: GErr<u32> = GErr::new_with_id(42, "error")
//         .set_prefix("[PREFIX]")
//         .set_help("help")
//         .add_tag("tag")
//         .add_source(std::io::Error::new(std::io::ErrorKind::Other, "source"));

//     assert_eq!(err.id(), &42);
//     assert_eq!(err.prefix(), Some("[PREFIX]"));
//     assert_eq!(err.help(), Some("help"));
//     assert_eq!(err.tags().unwrap().len(), 1);
//     assert_eq!(err.sources().unwrap().len(), 1);
// }

// // ============================================================================
// // Tests for builder method: with_id()
// // ============================================================================

// #[test]
// fn test_with_id_changes_id_type() {
//     let err1: GErr<u32> = GErr::new_with_id(42, "error");
//     let err2: GErr<&'static str> = err1.with_id("NEW_ID");

//     assert_eq!(err2.id(), &"NEW_ID");
//     assert_eq!(err2.message(), "error");
// }

// #[test]
// fn test_with_id_preserves_other_fields() {
//     let err1: GErr<u32> = GErr::new_with_id(42, "error")
//         .set_prefix("[PREFIX]")
//         .set_help("help")
//         .add_tag("tag");

//     let err2: GErr<&'static str> = err1.with_id("NEW_ID");

//     assert_eq!(err2.id(), &"NEW_ID");
//     assert_eq!(err2.message(), "error");
//     assert_eq!(err2.prefix(), Some("[PREFIX]"));
//     assert_eq!(err2.help(), Some("help"));
//     assert_eq!(err2.tags().unwrap().len(), 1);
// }

// #[test]
// fn test_with_id_chain() {
//     let err1: GErr<u32> = GErr::new_with_id(1, "error");
//     let err2: GErr<&'static str> = err1.with_id("STR_ID");
//     let err3: GErr<MockId> = err2.with_id(MockId(999));

//     assert_eq!(err3.id(), &MockId(999));
//     assert_eq!(err3.message(), "error");
// }

// // ============================================================================
// // Tests for builder method: with_prefix()
// // ============================================================================

// #[test]
// fn test_with_prefix_changes_prefix_type() {
//     struct NewPrefix;
//     impl Prefix for NewPrefix {
//         const PREFIX: Option<&'static str> = Some("[NEW]");
//     }

//     let err1: GErr<NoID, NoPrefix> = GErr::new("error");
//     let err2: GErr<NoID, NewPrefix> = err1.with_prefix_auto();

//     assert_eq!(err2.prefix(), Some("[NEW]"));
//     assert_eq!(err2.message(), "error");
// }

// #[test]
// fn test_with_prefix_preserves_other_fields() {
//     struct NewPrefix;
//     impl Prefix for NewPrefix {
//         const PREFIX: Option<&'static str> = Some("[NEW]");
//     }

//     let err1: GErr<u32, NoPrefix> = GErr::new_with_id(42, "error")
//         .set_help("help")
//         .add_tag("tag");

//     let err2: GErr<u32, NewPrefix> = err1.with_prefix_auto();

//     assert_eq!(err2.id(), &42);
//     assert_eq!(err2.message(), "error");
//     assert_eq!(err2.prefix(), Some("[NEW]"));
//     assert_eq!(err2.help(), Some("help"));
//     assert_eq!(err2.tags().unwrap().len(), 1);
// }

// #[test]
// fn test_with_prefix_overrides_manual_prefix() {
//     struct NewPrefix;
//     impl Prefix for NewPrefix {
//         const PREFIX: Option<&'static str> = Some("[NEW]");
//     }

//     let err1: GErr<NoID, NoPrefix> = GErr::new("error").set_prefix("[MANUAL]");
//     let err2: GErr<NoID, NewPrefix> = err1.with_prefix_auto();

//     // Manual prefix should be replaced with static prefix
//     assert_eq!(err2.prefix(), Some("[NEW]"));
// }

// // ============================================================================
// // Tests for builder method: with_data()
// // ============================================================================

// #[test]
// fn test_with_data_changes_data_type() {
//     let err1: GErr<NoID, NoPrefix, NoData> = GErr::new("error");
//     let err2: GErr<NoID, NoPrefix, TestData> = err1.with_data(TestData { code: 404 });

//     assert_eq!(err2.data(), Some(&TestData { code: 404 }));
//     assert_eq!(err2.message(), "error");
// }

// #[test]
// fn test_with_data_preserves_other_fields() {
//     let err1: GErr<u32, TestPrefix, NoData> = GErr::new_with_id(42, "error")
//         .set_help("help")
//         .add_tag("tag");

//     let err2: GErr<u32, TestPrefix, TestData> = err1.with_data(TestData { code: 500 });

//     assert_eq!(err2.id(), &42);
//     assert_eq!(err2.message(), "error");
//     assert_eq!(err2.prefix(), Some("[TEST]"));
//     assert_eq!(err2.help(), Some("help"));
//     assert_eq!(err2.data(), Some(&TestData { code: 500 }));
//     assert_eq!(err2.tags().unwrap().len(), 1);
// }

// #[test]
// fn test_with_data_chain() {
//     #[derive(Debug, PartialEq, Eq, Clone)]
//     struct DataType1 {
//         val: u32,
//     }
//     #[derive(Debug, PartialEq, Eq, Clone)]
//     struct DataType2 {
//         msg: &'static str,
//     }

//     let err1: GErr<NoID, NoPrefix, NoData> = GErr::new("error");
//     let err2: GErr<NoID, NoPrefix, DataType1> = err1.with_data(DataType1 { val: 42 });
//     let err3: GErr<NoID, NoPrefix, DataType2> = err2.with_data(DataType2 { msg: "new data" });

//     assert_eq!(err3.data(), Some(&DataType2 { msg: "new data" }));
//     assert_eq!(err3.message(), "error");
// }

// // ============================================================================
// // Tests for complex builder chains
// // ============================================================================

// #[test]
// fn test_complex_builder_chain_with_all_methods() {
//     let err: GErr<u32, TestPrefix, TestData> = GErr::new_with_id(1, "base error")
//         .set_prefix("[HTTP]")
//         .prepend_prefix("[API]")
//         .append_prefix("[FAILED]")
//         .set_help("Contact support immediately")
//         .add_tag("critical")
//         .add_tags(vec!["production", "urgent"])
//         .set_data(TestData { code: 500 })
//         .set_id(500);

//     assert_eq!(err.id(), &500);
//     assert_eq!(err.message(), "base error");
//     assert_eq!(err.prefix(), Some("[API][HTTP][FAILED]"));
//     assert_eq!(err.help(), Some("Contact support immediately"));
//     assert_eq!(err.data(), Some(&TestData { code: 500 }));
//     assert_eq!(err.tags().unwrap().len(), 3);
// }

// #[test]
// fn test_builder_chain_with_type_conversions() {
//     let err1: GErr<u32, NoPrefix, NoData> = GErr::new_with_id(42, "error");
//     let err2: GErr<&'static str, TestPrefix, TestData> = err1
//         .with_id("ERR_ID")
//         .with_prefix_auto()
//         .with_data(TestData { code: 400 })
//         .set_help("Invalid request");

//     assert_eq!(err2.id(), &"ERR_ID");
//     assert_eq!(err2.message(), "error");
//     assert_eq!(err2.prefix(), Some("[TEST]"));
//     assert_eq!(err2.data(), Some(&TestData { code: 400 }));
//     assert_eq!(err2.help(), Some("Invalid request"));
// }

// #[test]
// fn test_builder_with_from_error_chain() {
//     let std_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
//     let err: GErr<u32> = GErr::from_error_with_id(123, std_err)
//         .set_help("Check the file path")
//         .add_tag("filesystem");
//     let err = err
//         .with_id(404_i64)
//         .with_prefix_auto::<TestPrefix>()
//         .with_data(TestData { code: 404 });

//     assert_eq!(err.id(), &404);
//     assert_eq!(err.message(), "file not found");
//     assert_eq!(err.prefix(), Some("[TEST]"));
//     assert_eq!(err.data(), Some(&TestData { code: 404 }));
//     assert_eq!(err.help(), Some("Check the file path"));
//     assert_eq!(err.tags().unwrap().len(), 1);
//     assert!(err.sources().is_some());
// }

// #[test]
// fn test_builder_immutability() {
//     let base: GErr<NoID> = GErr::new("error");
//     assert_eq!(base.prefix(), None);
//     assert_eq!(base.tags(), None);
//     let modified1 = base.set_prefix("[A]").add_tag("tag1");
//     assert_eq!(modified1.prefix(), Some("[A]"));
//     let modified2 = modified1.set_prefix("[B]").add_tag("tag2");
//     assert_eq!(modified2.prefix(), Some("[B]"));
// }

// #[test]
// fn test_builder_none_returns_new_error() {
//     let err: GErr<NoID> = GErr::new("error");
//     let result = err.result::<()>();

//     assert!(result.is_err());
// }

// #[test]
// fn test_set_field_with_data_impl() {
//     use g_err::SetField;

//     #[derive(Debug, Default, Clone)]
//     struct CustomData {
//         fields: std::collections::HashMap<String, String>,
//     }

//     impl SetField<String, String> for CustomData {
//         fn set_field(&mut self, key: String, value: String) {
//             self.fields.insert(key, value);
//         }
//     }

//     let err: GErr<NoID, NoPrefix, CustomData> = GErr::new("error")
//         .set_field("key1".to_string(), "value1".to_string())
//         .set_field("key2".to_string(), "value2".to_string());

//     assert_eq!(err.data().unwrap().fields.len(), 2);
//     assert_eq!(
//         err.data().unwrap().fields.get("key1").map(|s| s.as_str()),
//         Some("value1")
//     );
// }
//

// =====================================

use g_err::{GErr, Id, NoID, Prefix, SetField};

#[test]
fn test_new_auto_default() {
    let gerr: GErr = GErr::new("auto default");

    assert_eq!(gerr.message(), "auto default");
    assert_eq!(gerr.id(), &NoID);
    assert!(gerr.prefix().is_none());
    assert!(gerr.data().is_none());
}

#[test]
fn test_new_manual_id() {
    let gerr: GErr<u32> = GErr::new_with_id(123, "manual id");

    assert_eq!(gerr.message(), "manual id");
    assert_eq!(gerr.id(), &123);
    assert!(gerr.prefix().is_none());
    assert!(gerr.data().is_none());
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Debug, PartialEq, Eq)]
struct AutoID;

impl Id for AutoID {
    fn id() -> Self {
        Self
    }
}

impl core::fmt::Display for AutoID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AutoID")
    }
}

struct AutoPrefix;

impl Prefix for AutoPrefix {
    const PREFIX: Option<&'static str> = Some("AutoPrefix");
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Debug, Default, PartialEq, Eq)]
struct Data {
    pub user_id: u64,
    pub user_name: String,
}

impl SetField<&'static str, u64> for Data {
    fn set_field(&mut self, key: &'static str, value: u64) {
        if key == "user_id" {
            self.user_id = value;
        }
    }
}

impl SetField<&'static str, String> for Data {
    fn set_field(&mut self, key: &'static str, value: String) {
        if key == "user_name" {
            self.user_name = value;
        }
    }
}

// test new with auto-generated id.
#[test]
fn test_new_auto() {
    // auto id, no prefix, no data
    let gerr = GErr::<AutoID>::new("error");
    assert_eq!(gerr.message(), "error");
    assert_eq!(gerr.id(), &AutoID);

    // no id, auto prefix, no data
    let gerr = GErr::<NoID, AutoPrefix>::new("auto prefix");
    assert_eq!(gerr.id(), &NoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");

    // auto id, auto prefix
    let gerr: GErr<AutoID, AutoPrefix> = GErr::new("id and prefix are auto");
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");
    assert_eq!(gerr.message(), "id and prefix are auto");

    // auto id, auto prefix, with data
    let gerr: GErr<AutoID, AutoPrefix, Data> = GErr::new("all").set_data(Data {
        user_id: 234,
        user_name: "xxx".into(),
    });
    assert_eq!(gerr.message(), "all");
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");
    assert_eq!(gerr.data().unwrap().user_id, 234);
    assert_eq!(gerr.data().unwrap().user_name, "xxx");

    let gerr: GErr<AutoID, AutoPrefix, Data> = GErr::new("all")
        .set_field("user_id", 400)
        .set_field("user_name", "yyy".to_string());
    assert_eq!(gerr.message(), "all");
    assert_eq!(gerr.id(), &AutoID);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");
    assert_eq!(gerr.data().unwrap().user_id, 400);
    assert_eq!(gerr.data().unwrap().user_name, "yyy");
}

// test new with manually-set id.
#[test]
fn test_new_manual() {
    let gerr = GErr::<_>::new_with_id(23, "test");
    assert_eq!(gerr.id(), &23);
    assert_eq!(gerr.message(), "test");
    assert!(gerr.prefix().is_none());

    let gerr = GErr::<&'static str>::new_with_id("zxc", "test").set_prefix("prefix");
    assert_eq!(gerr.id(), &"zxc");
    assert_eq!(gerr.message(), "test");
    assert_eq!(gerr.prefix().unwrap(), "prefix");
}

#[test]
fn test_from_non_gerr() {
    let err = "asd".parse::<i32>().unwrap_err();
    let gerr = GErr::<AutoID>::from_error(err);
    assert_eq!(gerr.id(), &AutoID);
    assert!(gerr.prefix().is_none());
    assert_eq!(gerr.message(), "invalid digit found in string");
}

#[test]
fn test_from_non_gerr_id() {
    let err = "asd".parse::<i32>().unwrap_err();
    let gerr: GErr<_, AutoPrefix> = GErr::from_error_with_id(123, err);
    assert_eq!(gerr.id(), &123);
    assert_eq!(gerr.prefix().unwrap(), "AutoPrefix");
    assert_eq!(gerr.message(), "invalid digit found in string");
}
