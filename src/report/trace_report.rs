use super::Report;
use crate::gerr::{Config, Source};
use crate::gerr_view::GErrView;
use core::fmt::{Debug, Display, Write};

/// Report type for error reporting as tracing format.
pub struct TraceReport;

impl Report for TraceReport {
    fn report<E, C: Config, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, C, D>>,
        C::Id: Display,
        D: Debug,
    {
        let err = &err.into();
        let mut out: String = String::new();
        Self::header(err, &mut out);
        Self::sources(err, &mut out);

        out
    }
}

impl TraceReport {
    fn header<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String)
    where
        C::Id: Display,
    {
        match (err.id, err.code) {
            (Some(id), Some(code)) => {
                let _ = writeln!(out, "[{id}] {code} {}", err.message);
            }
            (Some(id), None) => {
                let _ = writeln!(out, "[{id}] {}", err.message);
            }
            (None, Some(code)) => {
                let _ = writeln!(out, "[-] {code} {}", err.message);
            }
            (None, None) => {
                let _ = writeln!(out, "[-] {}", err.message);
            }
        }
    }

    fn sources<C: Config, D: Debug>(err: &GErrView<C, D>, out: &mut String) {
        if let Some(sources) = err.sources {
            Self::write_sources(sources, out, &mut Vec::new());
        }
    }

    fn write_sources(
        sources: &[Source],
        out: &mut String,
        ancestors: &mut Vec<bool>, // true = ancestor was last child
    ) {
        for (idx, src) in sources.iter().enumerate() {
            let is_last = idx + 1 == sources.len();

            // Draw ancestor guide lines.
            for &ancestor_is_last in ancestors.iter() {
                let _ = write!(out, "{}", if ancestor_is_last { "   " } else { "│  " });
            }

            let branch = if is_last { "└─ " } else { "├─ " };

            match src {
                Source::Err(e) => {
                    let _ = writeln!(out, "{branch}{e}");
                }

                Source::GErr(ge) => {
                    let msg = match (ge.id.as_ref(), ge.code.as_ref()) {
                        (Some(id), Some(code)) => {
                            format!("[{id}] {code} {}", ge.message)
                        }
                        (Some(id), None) => {
                            format!("[{id}] {}", ge.message)
                        }
                        (None, Some(code)) => {
                            format!("[-] {code} {}", ge.message)
                        }
                        (None, None) => {
                            format!("[-] {}", ge.message)
                        }
                    };

                    let _ = writeln!(out, "{branch}{msg}");

                    if let Some(nested) = &ge.sources {
                        ancestors.push(is_last);
                        Self::write_sources(nested, out, ancestors);
                        ancestors.pop();
                    }
                }
            }
        }
    }
}
