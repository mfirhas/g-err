use core::any::Any;
use core::error::Error;

use crate::{Config, DataSource, GErr, IdSource, iterator::GErrNode};

impl<C: Config, D> GErr<C, D>
where
    C::Id: IdSource + 'static,
    D: DataSource + 'static,
{
    /// Iterate over GErr's codes.
    #[inline]
    pub fn iter_by_code<'a, 'b>(
        &'a self,
        code: &'b str,
    ) -> impl Iterator<Item = GErrNode<'a, C, D>> + 'a
    where
        'b: 'a,
    {
        self.iter().filter(move |item| match item {
            GErrNode::Root(gerr) => gerr.code().is_some_and(|p| p == code),

            GErrNode::LeafGErr(gerr) => gerr.code.as_ref().is_some_and(|p| p == code),

            _ => false,
        })
    }

    /// Iterate over GErr's tags.
    #[inline]
    pub fn iter_by_tag<'a, 'b>(
        &'a self,
        tag: &'b str,
    ) -> impl Iterator<Item = GErrNode<'a, C, D>> + 'a
    where
        'b: 'a,
    {
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
    pub fn iter_id<T>(&self) -> impl Iterator<Item = GErrNode<'_, C, D>>
    where
        T: Any,
    {
        self.iter().filter(|item| match item {
            GErrNode::Root(gerr) => gerr.id().is_some_and(|id| (&*id as &dyn Any).is::<T>()),

            GErrNode::LeafGErr(gerr) => gerr
                .id
                .as_ref()
                .is_some_and(|id| (&**id as &dyn Any).is::<T>()),

            _ => false,
        })
    }

    /// Iterate over GErr's id.
    #[inline]
    pub fn iter_by_id<'a, 'b, T>(&'a self, value: &'b T) -> impl Iterator<Item = GErrNode<'a, C, D>>
    where
        T: Any + PartialEq,
        'b: 'a,
    {
        self.iter().filter(move |item| match item {
            GErrNode::Root(gerr) => gerr.id().is_some_and(|id| {
                (&*id as &dyn Any)
                    .downcast_ref::<T>()
                    .is_some_and(|id| id == value)
            }),
            GErrNode::LeafGErr(gerr) => gerr.id.as_ref().is_some_and(|id| {
                (&**id as &dyn Any)
                    .downcast_ref::<T>()
                    .is_some_and(|id| id == value)
            }),

            _ => false,
        })
    }

    /// Iterate over GErr's data's type.
    #[inline]
    pub fn iter_data<T>(&self) -> impl Iterator<Item = GErrNode<'_, C, D>>
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
    pub fn iter_by_data<'a, 'b, T>(
        &'a self,
        value: &'b T,
    ) -> impl Iterator<Item = GErrNode<'a, C, D>>
    where
        T: Any + PartialEq,
        'b: 'a,
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
    pub fn iter_source<E>(&self) -> impl Iterator<Item = GErrNode<'_, C, D>>
    where
        E: Error + 'static,
    {
        self.iter().filter_map(|item| match item {
            GErrNode::LeafErr(err) if err.is::<E>() => Some(item),

            GErrNode::LeafGErr(gerr) if (gerr as &dyn Error).is::<E>() => Some(item),

            _ => None,
        })
    }
}
