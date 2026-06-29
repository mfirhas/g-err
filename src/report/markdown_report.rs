use crate::{NoID, gerr_view::GErrView};

use super::Report;
use core::fmt::{Debug, Display, Write};

/// Report type for error reporting as markdown.
pub struct MarkdownReport;

impl Report for MarkdownReport {
    fn report<E, ID, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: Display,
        D: Debug,
    {
        let err = &err.into();
        let mut out: String = String::new();
        Self::header(&mut out);
        Self::preamble::<ID, D>(err, &mut out);
        Self::data::<ID, D>(err, &mut out);
        Self::help::<ID, D>(err, &mut out);
        Self::tags::<ID, D>(err, &mut out);
        Self::location::<ID, D>(err, &mut out);
        Self::sources::<ID, D>(err, &mut out);
        #[cfg(feature = "backtrace")]
        Self::backtrace::<ID, D>(err, &mut out);

        out
    }
}

impl MarkdownReport {
    fn header(out: &mut String) {
        let _ = writeln!(out, "# Error Report\n");
    }
    fn preamble<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(id) = err.id {
            let _ = writeln!(out, "## ID: {}\n", id);
        } else {
            let _ = writeln!(out, "## ID: {}\n", NoID);
        };

        let _ = writeln!(out, "## Prefix: {}\n", err.prefix.unwrap_or("-"));

        let _ = writeln!(out, "## Message\n");
        let _ = writeln!(out, "> {}\n", err.message);
    }
    fn data<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(data) = err.data {
            let _ = writeln!(out, "## Data\n");

            let _ = writeln!(out, "```");
            let _ = writeln!(out, "{data:#?}");
            let _ = writeln!(out, "```");
            let _ = writeln!(out);
        }
    }
    fn tags<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(tags) = err.tags
            && !tags.is_empty()
        {
            let _ = writeln!(out, "## Tags\n");

            for tag in tags {
                let _ = writeln!(out, "- {tag}");
            }

            let _ = writeln!(out);
        }
    }
    fn location<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(
            out,
            "## Location\n\n{}:{}:{}\n",
            err.location.file(),
            err.location.line(),
            err.location.column()
        );
    }
    fn sources<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(sources) = err.sources {
            let _ = writeln!(out, "## Causes\n");
            for (i, gerr) in sources.iter().enumerate() {
                let i = i + 1;

                let msg = match gerr.prefix.as_deref() {
                    Some(prefix) => format!("{prefix} {}", gerr.message),
                    None => gerr.message.to_string(),
                };

                let _ = writeln!(out, "### {}. {}\n", i, msg);

                if let Some(id) = err.id {
                    let _ = writeln!(out, "- **ID:** `{}`\n", id);
                } else {
                    let _ = writeln!(out, "- **ID:** `{}`\n", NoID);
                };

                if let Some(loc) = gerr.location {
                    let _ = writeln!(
                        out,
                        "- **Location:** `{}:{}:{}`\n",
                        loc.file(),
                        loc.line(),
                        loc.column()
                    );
                }

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
            }
        }
    }
    fn help<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(help) = err.help {
            let _ = writeln!(out, "## Help\n");
            let _ = writeln!(out, "> {}\n", help);
        }
    }
    #[cfg(feature = "backtrace")]
    fn backtrace<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(out, "## Backtrace\n");
        let _ = writeln!(out, "```");
        let _ = writeln!(out, "{:#?}", err.backtrace);
        let _ = writeln!(out, "```");
    }
}
