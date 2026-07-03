// import:
// #[path = "setup_test.rs"]
// mod setup_test;
// use setup_test::*;

use g_err::{Id, Prefix, SetField};

#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Debug, PartialEq, Eq)]
pub struct AutoID;

impl Id for AutoID {
    #[inline]
    fn id() -> Self {
        Self
    }
}

impl core::fmt::Display for AutoID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AutoID")
    }
}

pub struct AutoPrefix;

impl Prefix for AutoPrefix {
    const PREFIX: Option<&'static str> = Some("AutoPrefix");
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Data {
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
