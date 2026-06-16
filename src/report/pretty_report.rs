use super::Report;
use core::fmt::Write;
use core::fmt::{Debug, Display};

pub struct PrettyReport;

impl Report for PrettyReport {
    fn report<E, ID, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<super::GErrView<'a, ID, D>>,
        ID: Display,
        D: Debug,
    {
        let err = &err.into();
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

impl PrettyReport {
    fn header<ID: Display, D: Debug>(out: &mut String) {
        let _ = writeln!(out, "Error Report");
        let _ = writeln!(out, "============");
    }
    fn preamble<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(out, "ID: {}", err.id);
        if let Some(prefix) = err.prefix {
            let _ = writeln!(out, "Prefix: {prefix}");
        }
        let _ = writeln!(out, "Message: {}", err.message);
    }
    fn data<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        if let Some(data) = err.data {
            let _ = writeln!(out, "Data:\n {data:#?}");
        }
    }
    fn tags<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        if let Some(tags) = err.tags
            && !tags.is_empty()
        {
            let _ = writeln!(out, "Tags:");
            for tag in tags {
                let _ = writeln!(out, "  - {tag}");
            }
        }
    }
    fn location<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(
            out,
            "Location: {}:{}:{}",
            err.location.file(),
            err.location.line(),
            err.location.column()
        );
    }
    fn sources<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        if let Some(sources) = err.sources {
            let _ = writeln!(out, "Caused by:");
            for (i, src) in sources.iter().enumerate() {
                let i = i + 1;
                match src {
                    crate::Source::Err(e) => {
                        let _ = writeln!(out, "  {i}: {e}");
                    }
                    crate::Source::GErr(gerr) => {
                        match gerr.prefix {
                            Some(prefix) => {
                                let _ = writeln!(out, "  {i}: {prefix} {}", gerr.message);
                            }
                            None => {
                                let _ = writeln!(out, "  {i}: {}", gerr.message);
                            }
                        }

                        let _ = writeln!(out, "     id: {}", gerr.id);

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
                            let _ = writeln!(out, "     data: {data:#?}");
                        }
                    }
                }
            }
        }
    }
    #[cfg(feature = "backtrace")]
    fn backtrace<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        let _ = writeln!(out, "Backtrace:");
        let _ = writeln!(out, "{:#?}", err.backtrace);
    }
}
