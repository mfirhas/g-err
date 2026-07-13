use crate::{Config, Source, gerr_view::GErrView};

use super::Report;
use core::fmt::{Debug, Display, Write};

/// Report type for error reporting as markdown.
pub struct MarkdownReport;

impl Report for MarkdownReport {
    fn report<E, C: Config, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, C, D>>,
        C::Id: Display,
        D: Debug,
    {
        let err = &err.into();
        let mut out: String = String::new();
        Self::header(&mut out);
        Self::preamble::<C, D>(err, &mut out);
        Self::data::<C, D>(err, &mut out);
        Self::help::<C, D>(err, &mut out);
        Self::tags::<C, D>(err, &mut out);
        Self::location::<C, D>(err, &mut out);
        Self::sources::<C, D>(err, &mut out);
        #[cfg(feature = "backtrace")]
        Self::backtrace::<C, D>(err, &mut out);

        out
    }
}

impl MarkdownReport {
    fn header(out: &mut String) {
        let _ = writeln!(out, "# Error Report\n");
    }
    fn preamble<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String)
    where
        C::Id: Display,
    {
        if let Some(id) = err.id {
            let _ = writeln!(out, "## ID: {}\n", id);
        } else {
            let _ = writeln!(out, "## ID: -\n");
        }

        let _ = writeln!(out, "## Code: {}\n", err.code.unwrap_or("-"));

        let _ = writeln!(out, "## Message\n");
        let _ = writeln!(out, "> {}\n", err.message);
    }
    fn data<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        if let Some(data) = err.data {
            let _ = writeln!(out, "## Data\n");

            let _ = writeln!(out, "```");
            let _ = writeln!(out, "{data:#?}");
            let _ = writeln!(out, "```");
            let _ = writeln!(out);
        }
    }
    fn tags<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
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
    fn location<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        let _ = writeln!(
            out,
            "## Location\n\n{}:{}:{}\n",
            err.location.file, err.location.line, err.location.column
        );
    }
    fn sources<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        if let Some(sources) = err.sources {
            let _ = writeln!(out, "## Causes\n");
            for (i, gerr) in sources.iter().enumerate() {
                let i = i + 1;

                match gerr {
                    crate::gerr::Source::Err(err) => {
                        let _ = writeln!(out, "### {}. {}\n", i, err);
                    }

                    crate::gerr::Source::GErr(gerr) => {
                        let msg = match gerr.code.as_deref() {
                            Some(code) => format!("{code} {}", gerr.message),
                            None => gerr.message.to_string(),
                        };

                        let _ = writeln!(out, "### {}. {}\n", i, msg);

                        if let Some(id) = gerr.id.as_ref() {
                            let _ = writeln!(out, "- **ID:** `{}`\n", id);
                        } else {
                            let _ = writeln!(out, "- **ID:** `-`\n");
                        }

                        if let Some(ref loc) = gerr.location {
                            let _ = writeln!(
                                out,
                                "- **Location:** `{}:{}:{}`\n",
                                loc.file, loc.line, loc.column
                            );
                        }

                        if let Some(tags) = &gerr.tags
                            && !tags.is_empty()
                        {
                            let _ = writeln!(out, "- **Tags:** *{}*\n", tags.join(", "));
                        }

                        if let Some(help) = &gerr.help {
                            let _ = writeln!(out, "- **Help:** *{}*\n", help);
                        }

                        if let Some(data) = &gerr.data {
                            let _ = writeln!(out, "- **Data:**\n");
                            let _ = writeln!(out, "```");
                            let _ = writeln!(out, "{data:#?}");
                            let _ = writeln!(out, "```");
                        }

                        if let Some(sources) = &gerr.sources {
                            let _ = writeln!(out, "- **Causes:**");
                            let _ = writeln!(out);

                            Self::write_sources(out, sources, 1);
                        }
                    }
                }
            }
        }
    }
    fn write_sources(out: &mut String, sources: &[Source], depth: usize) {
        let indent = "    ".repeat(depth);
        let item_indent = format!("{indent}    ");

        for (i, src) in sources.iter().enumerate() {
            let _ = write!(out, "{indent}{}. ", i + 1);

            match src {
                Source::Err(err) => {
                    let _ = writeln!(out, "{err}");
                    let _ = writeln!(out);
                }

                Source::GErr(gerr) => {
                    match gerr.code.as_deref() {
                        Some(code) => {
                            let _ = writeln!(out, "{code} {}", gerr.message);
                        }
                        None => {
                            let _ = writeln!(out, "{}", gerr.message);
                        }
                    }

                    let _ = writeln!(out);
                    if let Some(id) = gerr.id.as_ref() {
                        let _ = writeln!(out, "{item_indent}- **ID:** `{}`", id);
                    } else {
                        let _ = writeln!(out, "{item_indent}- **ID:** `-`");
                    }

                    if let Some(ref loc) = gerr.location {
                        let _ = writeln!(
                            out,
                            "{item_indent}- **Location:** `{}:{}:{}`",
                            loc.file, loc.line, loc.column
                        );
                    }

                    if let Some(tags) = &gerr.tags
                        && !tags.is_empty()
                    {
                        let _ = writeln!(out, "{item_indent}- **Tags:** *{}*", tags.join(", "));
                    }

                    if let Some(help) = &gerr.help {
                        let _ = writeln!(out, "{item_indent}- **Help:** *{help}*");
                    }

                    if let Some(data) = &gerr.data {
                        let _ = writeln!(out);
                        let _ = writeln!(out, "{item_indent}- **Data:**");
                        let _ = writeln!(out);
                        let _ = writeln!(out, "{item_indent}```text");

                        let pretty = format!("{data:#?}");
                        for line in pretty.lines() {
                            let _ = writeln!(out, "{item_indent}{line}");
                        }

                        let _ = writeln!(out, "{item_indent}```");
                    }

                    if let Some(sources) = &gerr.sources {
                        let _ = writeln!(out, "{item_indent}- **Causes:**");

                        Self::write_sources(out, sources, depth + 2);
                    }

                    let _ = writeln!(out);
                }
            }
        }
    }
    fn help<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        if let Some(help) = err.help {
            let _ = writeln!(out, "## Help\n");
            let _ = writeln!(out, "> {}\n", help);
        }
    }
    #[cfg(feature = "backtrace")]
    fn backtrace<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        let _ = writeln!(out, "## Backtrace\n");
        let _ = writeln!(out, "```");
        let _ = writeln!(out, "{:#?}", err.backtrace);
        let _ = writeln!(out, "```");
    }
}
