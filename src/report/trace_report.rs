use super::Report;
use crate::GErrSource;
use crate::gerr_view::GErrView;
use core::fmt::{Debug, Display, Write};

/// Report type for error reporting as tracing format.
pub struct TraceReport;

impl Report for TraceReport {
    fn report<E, ID, D>(err: &E) -> String
    where
        for<'a> &'a E: Into<GErrView<'a, ID, D>>,
        ID: Display,
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
    fn header<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(id) = err.id {
            if let Some(prefix) = err.prefix {
                let _ = writeln!(out, "{prefix} {} ({})", err.message, id);
            } else {
                let _ = writeln!(out, "{} ({})", err.message, id);
            }
        } else {
            if let Some(prefix) = err.prefix {
                let _ = writeln!(out, "{prefix} {} (NoID)", err.message);
            } else {
                let _ = writeln!(out, "{} (NoID)", err.message);
            }
        };
    }

    fn sources<ID: Display, D: Debug>(err: &GErrView<ID, D>, out: &mut String) {
        if let Some(sources) = err.sources {
            Self::write_sources(sources, out, &mut Vec::new());
        }
    }

    fn write_sources(
        sources: &[GErrSource],
        out: &mut String,
        ancestors: &mut Vec<bool>, // true = ancestor was last child
    ) {
        for (idx, ge) in sources.iter().enumerate() {
            let is_last = idx + 1 == sources.len();

            // Draw ancestor guide lines.
            for &ancestor_is_last in ancestors.iter() {
                let _ = write!(out, "{}", if ancestor_is_last { "   " } else { "│  " });
            }

            let branch = if is_last { "└─ " } else { "├─ " };

            let msg = match (ge.prefix.as_deref(), ge.id.as_deref()) {
                (Some(prefix), Some(id)) => {
                    format!("{prefix} {} ({})", ge.message, &*id)
                }
                (Some(prefix), None) => format!("{prefix} {} (NoID)", ge.message),
                (None, Some(id)) => format!("{} ({})", ge.message, &*id),
                _ => format!("{} (NoID)", ge.message),
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
