//! Iterator for [`GErr`].
//!
//! Iterate over GErr's tree of root and leaf nodes.
//!
//! Produce iterator using [`GErr::iter`] method producing [`GErrTree`].
//!
//! [`GErrTree`] is traversed by DFS method.
use alloc::vec::Vec;
use core::error::Error;
use core::fmt::{Debug, Display};

use crate::gerr::Source;
use crate::{DataSource, GErr, GErrSource, IdSource, Prefix};

impl<ID, P, D> GErr<ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    /// Produces iterator of GErr's nodes(including self).
    #[inline]
    pub fn iter(&self) -> GErrTree<'_, ID, P, D> {
        GErrTree {
            nodes: vec![GErrNode::Root(self)],
        }
    }
}

/// A node in GErrTree.
///
/// Contained by [`GErrTree`].
pub enum GErrNode<'a, ID, P, D> {
    Root(&'a GErr<ID, P, D>),
    LeafErr(&'a (dyn Error + Send + Sync + 'static)),
    LeafGErr(&'a GErrSource),
}

impl<'a, ID, P, D> Display for GErrNode<'a, ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Root(root) => write!(f, "{}", root),
            Self::LeafErr(err) => write!(f, "{}", err),
            Self::LeafGErr(gerr) => write!(f, "{}", gerr),
        }
    }
}

impl<'a, ID, P, D> Debug for GErrNode<'a, ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Root(root) => write!(f, "root: {:#?}", root),
            Self::LeafErr(err) => write!(f, "err: {:#?}", err),
            Self::LeafGErr(gerr) => write!(f, "gerr: {:#?}", gerr),
        }
    }
}

/// Iterator of GErr error nodes.
///
/// Produced by [`GErr::iter`].
pub struct GErrTree<'a, ID, P, D> {
    nodes: Vec<GErrNode<'a, ID, P, D>>,
}

impl<'a, ID, P, D> Iterator for GErrTree<'a, ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    type Item = GErrNode<'a, ID, P, D>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.nodes.pop()?;

        match &current {
            GErrNode::Root(gerr) => {
                if let Some(sources) = gerr.sources() {
                    for source in sources.iter().rev() {
                        match &source {
                            Source::Err(err) => {
                                self.nodes.push(GErrNode::LeafErr(err.as_ref()));
                            }

                            Source::GErr(gerr) => {
                                self.nodes.push(GErrNode::LeafGErr(gerr.as_ref()));
                            }
                        }
                    }
                }
            }

            // External errors have no children.
            GErrNode::LeafErr(_) => {}

            GErrNode::LeafGErr(gerr) => {
                if let Some(sources) = gerr.sources.as_deref() {
                    for source in sources.iter().rev() {
                        match &source {
                            Source::Err(err) => {
                                self.nodes.push(GErrNode::LeafErr(err.as_ref()));
                            }

                            Source::GErr(gerr) => {
                                self.nodes.push(GErrNode::LeafGErr(gerr.as_ref()));
                            }
                        }
                    }
                }
            }
        }

        Some(current)
    }
}
