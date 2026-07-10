use super::Report;
use crate::gerr::Source;
use crate::gerr_view::GErrView;
use core::fmt::Write;
use core::fmt::{Debug, Display};

/// Report type for error reporting as pretty format.
pub struct PrettyReport;

impl Report for PrettyReport {
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

impl PrettyReport {
    fn header(out: &mut String) {
        let _ = writeln!(out, "Error Report");
        let _ = writeln!(out, "============");
    }
    fn preamble<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(out, "ID: {}", err.id);
        if let Some(prefix) = err.prefix {
            let _ = writeln!(out, "Prefix: {prefix}");
        }
        let _ = writeln!(out, "Message: {}", err.message);
    }
    fn data<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(data) = err.data {
            let _ = writeln!(out, "Data:");
            let pretty = format!("{data:#?}");

            for line in pretty.lines() {
                let _ = writeln!(out, "   {line}");
            }
        }
    }
    fn tags<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(tags) = err.tags
            && !tags.is_empty()
        {
            let _ = writeln!(out, "Tags:");
            for tag in tags {
                let _ = writeln!(out, "  - {tag}");
            }
        }
    }
    fn location<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(
            out,
            "Location: {}:{}:{}",
            err.location.file, err.location.line, err.location.column
        );
    }
    fn sources<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(sources) = err.sources {
            let _ = writeln!(out, "Caused by:");
            for (i, src) in sources.iter().enumerate() {
                let i = i + 1;

                match src {
                    Source::Err(err) => {
                        let _ = writeln!(out, "  {i}: {err}");
                    }

                    Source::GErr(gerr) => {
                        match gerr.prefix.as_deref() {
                            Some(prefix) => {
                                let _ = writeln!(out, "  {i}: {prefix} {}", gerr.message);
                            }
                            None => {
                                let _ = writeln!(out, "  {i}: {}", gerr.message);
                            }
                        }

                        let _ = writeln!(out, "     id: {}", gerr.id);

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
                    match gerr.prefix.as_deref() {
                        Some(prefix) => {
                            let _ = writeln!(out, "{pad}- {prefix} {}", gerr.message);
                        }
                        None => {
                            let _ = writeln!(out, "{pad}- {}", gerr.message);
                        }
                    }

                    let _ = writeln!(out, "{pad}  id: {}", gerr.id);

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
    fn help<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(help) = err.help {
            let _ = writeln!(out, "Help: {}", help);
        }
    }
    #[cfg(feature = "backtrace")]
    fn backtrace<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(out, "Backtrace:");
        let _ = writeln!(out, "{:#?}", err.backtrace);
    }
}
