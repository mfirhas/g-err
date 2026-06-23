use alloc::vec::Vec;
use core::fmt::{Debug, Display};

use crate::{DataSource, GErr, GErrSource, IdSource, Prefix};

impl<ID, P, D> GErr<ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    #[must_use]
    pub fn iter(&self) -> GErrTree<'_, ID, P, D> {
        GErrTree {
            nodes: vec![GErrNode::Root(self)],
        }
    }
}

pub enum GErrNode<'a, ID, P, D> {
    Root(&'a GErr<ID, P, D>),
    Leaf(&'a GErrSource),
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
            Self::Leaf(gerr) => write!(f, "{}", gerr),
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
            Self::Leaf(gerr) => write!(f, "gerr: {:#?}", gerr),
        }
    }
}

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
                        self.nodes.push(GErrNode::Leaf(source));
                    }
                }
            }

            GErrNode::Leaf(gerr) => {
                if let Some(sources) = gerr.sources.as_deref() {
                    for source in sources.iter().rev() {
                        self.nodes.push(GErrNode::Leaf(source));
                    }
                }
            }
        }

        Some(current)
    }
}
