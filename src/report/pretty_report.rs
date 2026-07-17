extern crate alloc;
use alloc::format;
use alloc::string::String;

use super::Report;
use crate::Config;
use crate::gerr::Source;
use crate::gerr_view::GErrView;
use core::fmt::Write;
use core::fmt::{Debug, Display};

/// Report type for error reporting as pretty format.
pub struct PrettyReport;

impl Report for PrettyReport {
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

impl PrettyReport {
    fn header(out: &mut String) {
        let _ = writeln!(out, "Error Report");
        let _ = writeln!(out, "============");
    }
    fn preamble<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String)
    where
        C::Id: Display,
    {
        if let Some(id) = err.id {
            let _ = writeln!(out, "ID: {}", id);
        } else {
            let _ = writeln!(out, "ID: -");
        }
        if let Some(code) = err.code {
            let _ = writeln!(out, "Code: {code}");
        } else {
            let _ = writeln!(out, "Code: -");
        }
        let _ = writeln!(out, "Message: {}", err.message);
    }
    fn data<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        if let Some(data) = err.data {
            let _ = writeln!(out, "Data:");
            let pretty = format!("{data:#?}");

            for line in pretty.lines() {
                let _ = writeln!(out, "   {line}");
            }
        }
    }
    fn tags<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        if let Some(tags) = err.tags
            && !tags.is_empty()
        {
            let _ = writeln!(out, "Tags:");
            for tag in tags {
                let _ = writeln!(out, "  - {tag}");
            }
        }
    }
    fn location<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        let _ = writeln!(
            out,
            "Location: {}:{}:{}",
            err.location.file, err.location.line, err.location.column
        );
    }
    fn sources<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        if let Some(sources) = err.sources {
            let _ = writeln!(out, "Caused by:");
            for (i, src) in sources.iter().enumerate() {
                let i = i + 1;

                match src {
                    Source::Err(err) => {
                        let _ = writeln!(out, "  {i}: {err}");
                    }

                    Source::GErr(gerr) => {
                        let _ = writeln!(out, "  {i}: {}", gerr.message);

                        if let Some(id) = gerr.id.as_ref() {
                            let _ = writeln!(out, "     id: {}", id);
                        } else {
                            let _ = writeln!(out, "     id: -");
                        }

                        if let Some(code) = gerr.code.as_deref() {
                            let _ = writeln!(out, "     code: {}", code);
                        } else {
                            let _ = writeln!(out, "     code: -");
                        }

                        if let Some(ref loc) = gerr.location {
                            let _ =
                                writeln!(out, "     at: {}:{}:{}", loc.file, loc.line, loc.column);
                        }

                        if let Some(tags) = &gerr.tags
                            && !tags.is_empty()
                        {
                            let _ = writeln!(out, "     tags: {}", tags.join(", "));
                        }

                        if let Some(help) = &gerr.help {
                            let _ = writeln!(out, "     help: {}", help);
                        }

                        if let Some(data) = &gerr.data {
                            let _ = writeln!(out, "     data:");
                            let pretty = format!("{data:#?}");

                            for line in pretty.lines() {
                                let _ = writeln!(out, "       {line}");
                            }
                        }

                        if let Some(sources) = &gerr.sources {
                            let _ = writeln!(out, "     caused by:");
                            Self::write_sources(out, sources, 6);
                        }
                    }
                }
            }
        }
    }
    fn write_sources(out: &mut String, sources: &[Source], indent: usize) {
        let pad = " ".repeat(indent);

        for src in sources {
            match src {
                Source::Err(err) => {
                    let _ = writeln!(out, "{pad}- {err}");
                }

                Source::GErr(gerr) => {
                    let _ = writeln!(out, "{pad}- {}", gerr.message);

                    if let Some(id) = gerr.id.as_ref() {
                        let _ = writeln!(out, "{pad}  id: {}", id);
                    } else {
                        let _ = writeln!(out, "{pad}  id: -");
                    }

                    if let Some(code) = gerr.code.as_deref() {
                        let _ = writeln!(out, "{pad}  code: {}", code);
                    } else {
                        let _ = writeln!(out, "{pad}  code: -");
                    }

                    if let Some(ref loc) = gerr.location {
                        let _ =
                            writeln!(out, "{pad}  at: {}:{}:{}", loc.file, loc.line, loc.column);
                    }

                    if let Some(tags) = &gerr.tags
                        && !tags.is_empty()
                    {
                        let _ = writeln!(out, "{pad}  tags: {}", tags.join(", "));
                    }

                    if let Some(help) = &gerr.help {
                        let _ = writeln!(out, "{pad}  help: {help}");
                    }

                    if let Some(data) = &gerr.data {
                        let _ = writeln!(out, "{pad}  data:");
                        let pretty = format!("{data:#?}");
                        for line in pretty.lines() {
                            let _ = writeln!(out, "{pad}    {line}");
                        }
                    }

                    if let Some(sources) = &gerr.sources {
                        let _ = writeln!(out, "{pad}  caused by:");
                        Self::write_sources(out, sources, indent + 3);
                    }
                }
            }
        }
    }
    fn help<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        if let Some(help) = err.help {
            let _ = writeln!(out, "Help: {}", help);
        }
    }
    #[cfg(feature = "backtrace")]
    fn backtrace<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        let _ = writeln!(out, "Backtrace:");
        let _ = writeln!(out, "{:#?}", err.backtrace);
    }
}
