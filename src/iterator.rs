//! Iterator for [`GErr`].
//!
//! Iterate over GErr's tree of root and leaf nodes.
//!
//! Produce iterator using [`GErr::iter`] method producing [`GErrTree`].
//!
//! [`GErrTree`] is traversed by DFS method.
use alloc::vec;
use alloc::vec::Vec;
use core::error::Error;
use core::fmt::{Debug, Display};

use crate::gerr::Source;
use crate::{Config, DataSource, GErr, GErrBox, GErrSource, IdSource};

impl<'a, C: Config, D> IntoIterator for &'a GErr<C, D>
where
    C::Id: IdSource + 'static,
    D: DataSource + 'static,
{
    type Item = GErrNode<'a, C, D>;
    type IntoIter = GErrTree<'a, C, D>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, C: Config, D> IntoIterator for &'a GErrBox<C, D>
where
    C::Id: IdSource + 'static,
    D: DataSource + 'static,
{
    type Item = GErrNode<'a, C, D>;
    type IntoIter = GErrTree<'a, C, D>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (*self).iter()
    }
}

impl<C: Config, D> GErr<C, D>
where
    C::Id: IdSource + 'static,
    D: DataSource + 'static,
{
    /// Produces iterator of GErr's nodes(including self).
    #[inline]
    pub fn iter(&self) -> GErrTree<'_, C, D> {
        GErrTree {
            nodes: vec![GErrNode::Root(self)],
        }
    }
}

/// A node in GErrTree.
///
/// Contained by [`GErrTree`].
pub enum GErrNode<'a, C: Config, D> {
    /// Root of GErr.
    Root(&'a GErr<C, D>),
    /// non-gerr source node.
    LeafErr(&'a (dyn Error + Send + Sync + 'static)),
    /// gerr or any error convertible to [`GErrSource`] node.
    LeafGErr(&'a GErrSource),
}

impl<'a, C: Config, D> Display for GErrNode<'a, C, D>
where
    C::Id: IdSource + 'static,
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

impl<'a, C: Config, D> Debug for GErrNode<'a, C, D>
where
    C::Id: IdSource + 'static,
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
pub struct GErrTree<'a, C: Config, D> {
    nodes: Vec<GErrNode<'a, C, D>>,
}

impl<'a, C: Config, D> Iterator for GErrTree<'a, C, D>
where
    C::Id: IdSource + 'static,
    D: DataSource + 'static,
{
    type Item = GErrNode<'a, C, D>;

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
