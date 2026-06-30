use core::any::Any;
use core::error::Error;

use crate::{DataSource, GErr, IdSource, Prefix, iterator::GErrNode};

impl<ID, P, D> GErr<ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    /// Iterate over GErr's prefixes.
    #[inline]
    pub fn iter_by_prefix<'a>(
        &'a self,
        prefix: &'a str,
    ) -> impl Iterator<Item = GErrNode<'a, ID, P, D>> + 'a {
        self.iter().filter(move |item| match item {
            GErrNode::Root(gerr) => gerr.prefix().is_some_and(|p| p == prefix),

            GErrNode::LeafGErr(gerr) => gerr.prefix.as_ref().is_some_and(|p| p == prefix),

            _ => false,
        })
    }

    /// Iterate over GErr's tags.
    #[inline]
    pub fn iter_by_tag<'a>(
        &'a self,
        tag: &'a str,
    ) -> impl Iterator<Item = GErrNode<'a, ID, P, D>> + 'a {
        self.iter().filter(move |item| match item {
            GErrNode::Root(gerr) => gerr
                .tags()
                .is_some_and(|tags| tags.iter().any(|t| t == tag)),

            GErrNode::LeafGErr(gerr) => gerr
                .tags
                .as_ref()
                .is_some_and(|tags| tags.iter().any(|t| t == tag)),

            _ => false,
        })
    }

    /// Iterate over GErr's id type.
    #[inline]
    pub fn iter_id<T>(&self) -> impl Iterator<Item = GErrNode<'_, ID, P, D>>
    where
        T: Any,
    {
        self.iter().filter(|item| match item {
            GErrNode::Root(gerr) => (gerr.id() as &dyn Any).is::<T>(),

            GErrNode::LeafGErr(gerr) => (&*gerr.id as &dyn Any).is::<T>(),

            _ => false,
        })
    }

    /// Iterate over GErr's id.
    #[inline]
    pub fn iter_by_id<T>(&self, value: &T) -> impl Iterator<Item = GErrNode<'_, ID, P, D>>
    where
        T: Any + PartialEq,
    {
        self.iter().filter(move |item| match item {
            GErrNode::Root(gerr) => (gerr.id() as &dyn Any)
                .downcast_ref::<T>()
                .is_some_and(|id| id == value),

            GErrNode::LeafGErr(gerr) => (&*gerr.id as &dyn Any)
                .downcast_ref::<T>()
                .is_some_and(|id| id == value),

            _ => false,
        })
    }

    /// Iterate over GErr's data's type.
    #[inline]
    pub fn iter_data<T>(&self) -> impl Iterator<Item = GErrNode<'_, ID, P, D>>
    where
        T: Any,
    {
        self.iter().filter(|item| match item {
            GErrNode::Root(gerr) => gerr.data().is_some_and(|data| (data as &dyn Any).is::<T>()),

            GErrNode::LeafGErr(gerr) => gerr
                .data
                .as_ref()
                .is_some_and(|data| (&**data as &dyn Any).is::<T>()),

            _ => false,
        })
    }

    /// Iterate over GErr's data.
    #[inline]
    pub fn iter_by_data<T>(&self, value: &T) -> impl Iterator<Item = GErrNode<'_, ID, P, D>>
    where
        T: Any + PartialEq,
    {
        self.iter().filter(move |item| match item {
            GErrNode::Root(gerr) => gerr
                .data()
                .and_then(|data| (data as &dyn Any).downcast_ref::<T>())
                .is_some_and(|data| data == value),

            GErrNode::LeafGErr(gerr) => gerr
                .data
                .as_ref()
                .and_then(|data| (&**data as &dyn Any).downcast_ref::<T>())
                .is_some_and(|data| data == value),

            _ => false,
        })
    }

    /// Iterate over GErr's sources by source's type.
    #[inline]
    pub fn iter_source<E>(&self) -> impl Iterator<Item = GErrNode<'_, ID, P, D>>
    where
        E: Error + 'static,
    {
        self.iter().filter_map(|item| match item {
            GErrNode::LeafErr(err) if (err as &dyn Error).is::<E>() => Some(item),

            GErrNode::LeafGErr(gerr) if (gerr as &dyn Error).is::<E>() => Some(item),

            _ => None,
        })
    }

    /// Find GErr's id by type.
    #[inline]
    pub fn find_id<T>(&self) -> Option<GErrNode<'_, ID, P, D>>
    where
        T: Any,
    {
        self.iter_id::<T>().next()
    }

    /// Find GErr's id by value.
    #[inline]
    pub fn find_by_id<T>(&self, value: &T) -> Option<GErrNode<'_, ID, P, D>>
    where
        T: Any + PartialEq,
    {
        self.iter_by_id(value).next()
    }

    /// Find GErr's prefix by value.
    #[inline]
    pub fn find_by_prefix<'a>(&'a self, value: &'a str) -> Option<GErrNode<'a, ID, P, D>> {
        self.iter_by_prefix(value).next()
    }

    /// Find GErr's data by type.
    #[inline]
    pub fn find_data<T>(&self) -> Option<GErrNode<'_, ID, P, D>>
    where
        T: Any,
    {
        self.iter_data::<T>().next()
    }

    /// Find GErr's data by value.
    #[inline]
    pub fn find_by_data<T>(&self, value: &T) -> Option<GErrNode<'_, ID, P, D>>
    where
        T: Any + PartialEq,
    {
        self.iter_by_data(value).next()
    }

    /// Find GErr's tags by value.
    #[inline]
    pub fn find_by_tag<'a>(&'a self, value: &'a str) -> Option<GErrNode<'a, ID, P, D>> {
        self.iter_by_tag(value).next()
    }

    /// Find GErr's sources by source's type.
    #[inline]
    pub fn find_source<E>(&self) -> Option<GErrNode<'_, ID, P, D>>
    where
        E: Error + 'static,
    {
        self.iter_source::<E>().next()
    }
}
