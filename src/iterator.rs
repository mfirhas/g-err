use alloc::vec::Vec;
use core::error::Error;
use core::fmt::{Debug, Display};

use crate::{DataSource, GErr, GErrSource, IdSource, Prefix, Source};

impl<ID, P, D> GErr<ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    #[must_use]
    pub fn iter(&self) -> Iter<'_, ID, P, D> {
        Iter {
            stack: vec![IterItem::Root(self)],
        }
    }
}

pub enum IterItem<'a, ID, P, D> {
    Root(&'a GErr<ID, P, D>),
    Err(&'a (dyn Error + Send + Sync + 'static)),
    GErr(&'a GErrSource),
}

impl<'a, ID, P, D> Display for IterItem<'a, ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root(root) => write!(f, "{}", root),
            Self::Err(err) => write!(f, "{}", err),
            Self::GErr(gerr) => write!(f, "{}", gerr),
        }
    }
}

impl<'a, ID, P, D> Debug for IterItem<'a, ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root(root) => write!(f, "root: {:#?}", root),
            Self::Err(err) => write!(f, "err: {:#?}", err),
            Self::GErr(gerr) => write!(f, "gerr: {:#?}", gerr),
        }
    }
}

pub struct Iter<'a, ID, P, D> {
    stack: Vec<IterItem<'a, ID, P, D>>,
}

impl<'a, ID, P, D> Iterator for Iter<'a, ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    type Item = IterItem<'a, ID, P, D>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;

        match &current {
            IterItem::Err(_) => {}

            IterItem::Root(gerr) => {
                if let Some(sources) = gerr.sources() {
                    for source in sources.iter().rev() {
                        match source {
                            Source::Err(err) => {
                                self.stack.push(IterItem::Err(&**err));
                            }

                            Source::GErr(gerr) => {
                                self.stack.push(IterItem::GErr(gerr.as_ref()));
                            }
                        }
                    }
                }
            }

            IterItem::GErr(gerr) => {
                if let Some(sources) = gerr.sources.as_deref() {
                    for source in sources.iter().rev() {
                        match source {
                            Source::Err(err) => {
                                self.stack.push(IterItem::Err(&**err));
                            }

                            Source::GErr(gerr) => {
                                self.stack.push(IterItem::GErr(gerr.as_ref()));
                            }
                        }
                    }
                }
            }
        }

        Some(current)
    }
}
