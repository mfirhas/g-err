use super::Report;
use core::fmt::{Debug, Display, Write};

pub struct MarkdownReport;

impl crate::sealed::Sealed for MarkdownReport {}

impl Report for MarkdownReport {
    fn report<ID, D>(err: &super::GErrView<ID, D>) -> String
    where
        ID: Display,
        D: Debug,
    {
        let mut out: String = String::new();
        Self::header::<ID, D>(&mut out);
        Self::preamble::<ID, D>(err, &mut out);
        Self::data::<ID, D>(err, &mut out);
        Self::tags::<ID, D>(err, &mut out);
        Self::location::<ID, D>(err, &mut out);
        Self::sources::<ID, D>(err, &mut out);
        #[cfg(feature = "backtrace")]
        Self::backtrace::<ID, D>(err, &mut out);
        out
    }
}

impl MarkdownReport {
    fn header<ID: Display, D: Debug>(out: &mut String) {
        let _ = writeln!(out, "# Error Report\n");
    }
    fn preamble<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(out, "## ID: {}\n", err.id);

        let _ = writeln!(out, "## Prefix: {}\n", err.prefix.unwrap_or("-"));

        let _ = writeln!(out, "## Message\n");
        let _ = writeln!(out, "> {}\n", err.message);
    }
    fn data<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        if let Some(data) = err.data {
            let _ = writeln!(out, "## Data\n");

            let _ = writeln!(out, "```");
            let _ = writeln!(out, "{data:#?}");
            let _ = writeln!(out, "```");
            let _ = writeln!(out);
        }
    }
    fn tags<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
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
    fn location<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(
            out,
            "## Location\n\n{}:{}:{}\n",
            err.location.file(),
            err.location.line(),
            err.location.column()
        );
    }
    fn sources<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        if let Some(sources) = err.sources {
            let _ = writeln!(out, "## Causes\n");
            for (i, src) in sources.iter().enumerate() {
                let i = i + 1;
                match src {
                    crate::Source::Err(e) => {
                        let _ = writeln!(out, "### {}. {}\n\n", i, e);
                    }
                    crate::Source::GErr(gerr) => {
                        let msg = match gerr.prefix {
                            Some(prefix) => format!("{prefix} {}", gerr.message),
                            None => gerr.message.to_string(),
                        };

                        let _ = writeln!(out, "### {}. {}\n", i, msg);

                        let _ = writeln!(out, "- **ID:** `{}`\n", gerr.id);

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
                    }
                }
            }
        }
    }
    #[cfg(feature = "backtrace")]
    fn backtrace<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(out, "## Backtrace\n");
        let _ = writeln!(out, "```");
        let _ = writeln!(out, "{:#?}", err.backtrace);
        let _ = writeln!(out, "```");
    }
}
