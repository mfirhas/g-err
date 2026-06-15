use super::Report;
use core::fmt::{Debug, Display, Write};

pub struct TraceReport;

impl crate::sealed::Sealed for TraceReport {}

impl Report for TraceReport {
    fn report<ID, D>(err: &super::GErrView<ID, D>) -> String
    where
        ID: Display,
        D: Debug,
    {
        let mut out: String = String::new();
        Self::header(err, &mut out);
        Self::sources(err, &mut out);

        out
    }
}

impl TraceReport {
    fn header<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        if let Some(prefix) = err.prefix {
            let _ = writeln!(out, "{prefix} {}", err.message);
        } else {
            let _ = writeln!(out, "{}", err.message);
        }
    }

    fn sources<ID: Display, D: Debug>(err: &super::GErrView<ID, D>, out: &mut String) {
        if let Some(sources) = err.sources {
            Self::write_sources(sources, out, 0);
        }
    }

    fn write_sources(sources: &[crate::Source], out: &mut String, depth: usize) {
        let indent = "   ".repeat(depth);

        for src in sources {
            match src {
                crate::Source::Err(e) => {
                    let _ = writeln!(out, "{indent}└─ {e}");
                }

                crate::Source::GErr(ge) => {
                    let msg = match ge.prefix {
                        Some(prefix) => format!("{prefix} {} ({})", ge.message, ge.id),
                        None => format!("{} ({})", ge.message, ge.id),
                    };

                    let _ = writeln!(out, "{indent}└─ {msg}");

                    if let Some(nested) = &ge.source {
                        Self::write_sources(nested, out, depth + 1);
                    }
                }
            }
        }
    }
}
