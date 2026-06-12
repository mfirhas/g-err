extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::String;
use core::error::Error;
use core::fmt::Debug;
use core::fmt::Write;

use crate::GErrSource;
use crate::{GErr, Prefix};

impl<ID: Debug, P: Prefix, D: Debug> GErr<ID, P, D>
where
    Self: Error + 'static,
{
    pub fn pretty(&self) -> String {
        let mut out = String::new();

        let _ = writeln!(out, "Error Report");
        let _ = writeln!(out, "============");

        let _ = writeln!(out, "ID: {:?}", self.id);
        if let Some(prefix) = self.prefix() {
            let _ = writeln!(out, "Prefix: {prefix}");
        }
        let _ = writeln!(out, "Message: {}", self.message);

        if let Some(tags) = self.tags()
            && !tags.is_empty()
        {
            let _ = writeln!(out, "Tags:");
            for tag in tags {
                let _ = writeln!(out, "  - {tag}");
            }
        }

        if let Some(data) = &self.data {
            let _ = writeln!(out, "Data: {data:?}");
        }

        let _ = writeln!(
            out,
            "Location: {}:{}:{}",
            self.location.file(),
            self.location.line(),
            self.location.column()
        );

        let mut chain = self.chain();

        if chain.next().is_some() {
            let causes: Vec<_> = chain.collect();

            if !causes.is_empty() {
                let _ = writeln!(out, "Caused by:");

                for (i, err) in causes.iter().enumerate() {
                    let i = i + 1;
                    if let Some(gerr) = err.downcast_ref::<GErrSource>() {
                        match gerr.prefix {
                            Some(prefix) => {
                                let _ = writeln!(out, "  {i}: {prefix} {}", gerr.message);
                            }
                            None => {
                                let _ = writeln!(out, "  {i}: {}", gerr.message);
                            }
                        }

                        let _ = writeln!(out, "     id: {:?}", gerr.id);

                        let _ = writeln!(
                            out,
                            "     at: {}:{}:{}",
                            gerr.location.file(),
                            gerr.location.line(),
                            gerr.location.column()
                        );

                        if let Some(tags) = &gerr.tags
                            && !tags.is_empty()
                        {
                            let _ = writeln!(out, "     tags: {}", tags.join(", "));
                        }

                        if let Some(data) = &gerr.data {
                            let _ = writeln!(out, "     data: {data:?}");
                        }

                        let _ = writeln!(out);
                    } else {
                        let _ = writeln!(out, "  {i}: {err}");
                    }
                }
            }
        }

        #[cfg(feature = "backtrace")]
        {
            let _ = writeln!(out, "Backtrace:");
            let _ = writeln!(out, "{:?}", self.backtrace);
        }

        out
    }

    pub fn markdown(&self) -> String
    where
        ID: Debug,
        D: Debug,
    {
        let mut out = String::new();

        let _ = writeln!(out, "# Error Report\n");

        let _ = writeln!(out, "## ID: {:#?}\n", self.id());

        let _ = writeln!(out, "## Prefix: {}\n", self.prefix().unwrap_or("-"));

        let _ = writeln!(out, "## Message\n");
        let _ = writeln!(out, "> {}\n", self.message());

        let _ = writeln!(
            out,
            "## Location\n\n{}:{}:{}\n",
            self.location().file(),
            self.location().line(),
            self.location().column()
        );

        if let Some(tags) = self.tags()
            && !tags.is_empty()
        {
            let _ = writeln!(out, "## Tags\n");

            for tag in tags {
                let _ = writeln!(out, "- {tag}");
            }

            let _ = writeln!(out);
        }

        if let Some(data) = &self.data {
            let _ = writeln!(out, "## Data\n");

            let _ = writeln!(out, "```");
            let _ = writeln!(out, "{data:#?}");
            let _ = writeln!(out, "```");
            let _ = writeln!(out);
        }

        let causes: Vec<_> = self.chain().skip(1).collect();

        if !causes.is_empty() {
            let _ = writeln!(out, "## Causes\n");

            for (i, err) in causes.iter().enumerate() {
                let i = i + 1;

                if let Some(gerr) = err.downcast_ref::<GErrSource>() {
                    let msg = match gerr.prefix {
                        Some(prefix) => format!("{prefix} {}", gerr.message),
                        None => gerr.message.to_string(),
                    };

                    let _ = writeln!(out, "### {}. {}\n", i, msg);

                    let _ = writeln!(out, "- **ID:** `{:?}`\n", gerr.id);

                    let loc = &gerr.location;
                    let _ = writeln!(
                        out,
                        "- **Location:** `{}:{}:{}`\n",
                        loc.file(),
                        loc.line(),
                        loc.column()
                    );

                    if let Some(tags) = &gerr.tags
                        && !tags.is_empty()
                    {
                        let _ = writeln!(out, "- **Tags:** *{}*\n", tags.join(", "));
                    }

                    if let Some(data) = &gerr.data {
                        let _ = writeln!(out, "- **Data:**\n");
                        let _ = writeln!(out, "```");
                        let _ = writeln!(out, "{data:#?}");
                        let _ = writeln!(out, "```");
                    }

                    let _ = writeln!(out);
                } else {
                    let _ = writeln!(out, "### {}. {}\n\n", i + 1, err);
                }
            }

            let _ = writeln!(out);
        }

        #[cfg(feature = "backtrace")]
        {
            let _ = writeln!(out, "## Backtrace\n");
            let _ = writeln!(out, "```");
            let _ = writeln!(out, "{:#?}", self.backtrace);
            let _ = writeln!(out, "```");
        }

        out
    }

    pub fn trace(&self) -> String
    where
        Self: Error + 'static,
    {
        let mut out = String::new();

        for (depth, err) in self.chain().enumerate() {
            let err_msg = if let Some(gerr) = err.downcast_ref::<GErrSource>() {
                if let Some(prefix) = gerr.prefix {
                    format!("{} {} ({:?})", prefix, gerr.message, gerr.id)
                } else {
                    format!("{} ({:?})", gerr.message, gerr.id)
                }
            } else {
                format!("{err}")
            };

            if depth == 0 {
                let _ = writeln!(out, "{err_msg}");
            } else {
                let indent = "   ".repeat(depth - 1);
                let _ = writeln!(out, "{indent}└─ {err_msg}");
            }
        }

        out
    }
}

#[cfg(feature = "serde")]
mod json_data {
    use super::*;

    #[derive(serde::Serialize)]
    pub struct DisplayJsonReport<'a, ID, D> {
        pub id: &'a ID,
        pub prefix: Option<&'static str>,
        pub message: &'a str,
        pub tags: Option<&'a [Cow<'static, str>]>,
        pub data: &'a Option<D>,
    }

    #[derive(serde::Serialize)]
    pub struct LocationReport<'a> {
        pub file: &'a str,
        pub line: u32,
        pub column: u32,
    }

    #[derive(serde::Serialize)]
    pub struct JsonReport<'a, ID, D> {
        pub id: &'a ID,
        pub prefix: Option<&'static str>,
        pub message: &'a str,
        pub tags: Option<&'a [Cow<'static, str>]>,
        pub data: &'a Option<D>,
        pub location: LocationReport<'a>,
        pub chain: Vec<String>,

        #[cfg(feature = "backtrace")]
        pub backtrace: String,
    }
}

#[cfg(feature = "serde")]
impl<ID, P: Prefix, D> GErr<ID, P, D>
where
    ID: serde::Serialize,
    D: serde::Serialize,
    Self: Error + 'static,
{
    #[inline]
    pub fn display_json_data(&self) -> json_data::DisplayJsonReport<'_, ID, D> {
        json_data::DisplayJsonReport {
            id: &self.id,
            prefix: self.prefix(),
            message: self.message(),
            tags: self.tags(),
            data: &self.data,
        }
    }

    pub fn display_json(&self) -> serde_json::Result<String>
    where
        ID: serde::Serialize,
        D: serde::Serialize,
    {
        serde_json::to_string_pretty(&self.display_json_data())
    }

    pub fn json_data(&self) -> json_data::JsonReport<'_, ID, D> {
        let chain = self.chain().skip(1).map(ToString::to_string).collect();

        let report = json_data::JsonReport {
            id: &self.id,
            prefix: self.prefix(),
            message: self.message(),
            tags: self.tags(),
            data: &self.data,
            location: json_data::LocationReport {
                file: self.location.file(),
                line: self.location.line(),
                column: self.location.column(),
            },
            chain,

            #[cfg(feature = "backtrace")]
            backtrace: format!("{:?}", self.backtrace),
        };

        report
    }

    pub fn json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.json_data())
    }
}
