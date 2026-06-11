extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::String;
use core::error::Error;
use core::fmt::Debug;
use core::fmt::Write;

use crate::{Err, Prefix};

impl<ID: Debug, P: Prefix, D: Debug> Err<ID, P, D>
where
    Self: Error + 'static,
{
    pub fn report(&self) -> String {
        let mut out = String::new();

        let _ = writeln!(out, "Error Report");
        let _ = writeln!(out, "============");

        let _ = writeln!(out, "Message: {}", self.message);

        if let Some(prefix) = self.prefix() {
            let _ = writeln!(out, "Prefix: {prefix}");
        }

        let _ = writeln!(out, "ID: {:?}", self.id);

        if !self.tags.is_empty() {
            let _ = writeln!(out, "Tags:");
            for tag in &self.tags {
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
                    let _ = writeln!(out, "  {i}: {err}");
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
        let _ = writeln!(out, "## Message\n");
        let _ = writeln!(out, "{}\n", self.message());

        let _ = writeln!(
            out,
            "## Location\n\n{}:{}:{}\n",
            self.location().file(),
            self.location().line(),
            self.location().column()
        );

        if !self.tags().is_empty() {
            let _ = writeln!(out, "## Tags\n");

            for tag in self.tags() {
                let _ = writeln!(out, "- {tag}");
            }

            let _ = writeln!(out);
        }

        let causes: Vec<_> = self.chain().skip(1).collect();

        if !causes.is_empty() {
            let _ = writeln!(out, "## Causes\n");

            for (i, cause) in causes.iter().enumerate() {
                let _ = writeln!(out, "{}. {}", i + 1, cause);
            }
        }

        out
    }

    pub fn trace(&self) -> String
    where
        Self: Error + 'static,
    {
        use alloc::string::String;
        use core::fmt::Write;

        let mut out = String::new();

        for (depth, err) in self.chain().enumerate() {
            if depth == 0 {
                let _ = writeln!(out, "{err}");
            } else {
                let indent = "   ".repeat(depth - 1);
                let _ = writeln!(out, "{indent}└─ {err}");
            }
        }

        out
    }
}

#[cfg(feature = "json")]
impl<ID, P: Prefix, D> Err<ID, P, D>
where
    ID: serde::Serialize,
    D: serde::Serialize,
    Self: Error + 'static,
{
    pub fn json(&self) -> serde_json::Result<String> {
        #[derive(serde::Serialize)]
        struct LocationReport<'a> {
            file: &'a str,
            line: u32,
            column: u32,
        }

        #[derive(serde::Serialize)]
        struct ErrorReport<'a, ID, D> {
            id: &'a ID,
            prefix: Option<&'static str>,
            message: &'a str,
            tags: &'a [Cow<'static, str>],
            data: &'a Option<D>,
            location: LocationReport<'a>,
            chain: Vec<String>,

            #[cfg(feature = "backtrace")]
            backtrace: String,
        }

        let chain = self.chain().skip(1).map(ToString::to_string).collect();

        let report = ErrorReport {
            id: &self.id,
            prefix: self.prefix(),
            message: self.message(),
            tags: &self.tags,
            data: &self.data,
            location: LocationReport {
                file: self.location.file(),
                line: self.location.line(),
                column: self.location.column(),
            },
            chain,

            #[cfg(feature = "backtrace")]
            backtrace: format!("{:?}", self.backtrace),
        };

        serde_json::to_string_pretty(&report)
    }
}
