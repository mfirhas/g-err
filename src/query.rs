use core::any::Any;
use core::error::Error;

use crate::{DataSource, GErr, IdSource, IterItem, Prefix};

impl<ID, P, D> GErr<ID, P, D>
where
    ID: IdSource + 'static,
    P: Prefix,
    D: DataSource + 'static,
{
    #[inline]
    pub fn iter_by_prefix<'a>(
        &'a self,
        prefix: &'a str,
    ) -> impl Iterator<Item = IterItem<'a, ID, P, D>> + 'a {
        self.iter().filter(move |item| match item {
            IterItem::Root(gerr) => gerr.prefix().is_some_and(|p| p == prefix),

            IterItem::GErr(gerr) => gerr.prefix.as_ref().is_some_and(|p| p == prefix),

            IterItem::Err(_) => false,
        })
    }

    #[inline]
    pub fn iter_by_tag<'a>(
        &'a self,
        tag: &'a str,
    ) -> impl Iterator<Item = IterItem<'a, ID, P, D>> + 'a {
        self.iter().filter(move |item| match item {
            IterItem::Root(gerr) => gerr
                .tags()
                .is_some_and(|tags| tags.iter().any(|t| t == tag)),

            IterItem::GErr(gerr) => gerr
                .tags
                .as_ref()
                .is_some_and(|tags| tags.iter().any(|t| t == tag)),

            IterItem::Err(_) => false,
        })
    }

    #[inline]
    pub fn iter_id<T>(&self) -> impl Iterator<Item = IterItem<'_, ID, P, D>>
    where
        T: Any,
    {
        self.iter().filter(|item| match item {
            IterItem::Root(gerr) => (gerr.id() as &dyn Any).is::<T>(),

            IterItem::GErr(gerr) => (&*gerr.id as &dyn Any).is::<T>(),

            IterItem::Err(_) => false,
        })
    }

    #[inline]
    pub fn iter_by_id<T>(&self, value: &T) -> impl Iterator<Item = IterItem<'_, ID, P, D>>
    where
        T: Any + PartialEq,
    {
        self.iter().filter(move |item| match item {
            IterItem::Root(gerr) => (gerr.id() as &dyn Any)
                .downcast_ref::<T>()
                .is_some_and(|id| id == value),

            IterItem::GErr(gerr) => (&*gerr.id as &dyn Any)
                .downcast_ref::<T>()
                .is_some_and(|id| id == value),

            IterItem::Err(_) => false,
        })
    }

    #[inline]
    pub fn iter_data<T>(&self) -> impl Iterator<Item = IterItem<'_, ID, P, D>>
    where
        T: Any,
    {
        self.iter().filter(|item| match item {
            IterItem::Root(gerr) => gerr.data().is_some_and(|data| (data as &dyn Any).is::<T>()),

            IterItem::GErr(gerr) => gerr
                .data
                .as_ref()
                .is_some_and(|data| (&**data as &dyn Any).is::<T>()),

            IterItem::Err(_) => false,
        })
    }

    #[inline]
    pub fn iter_by_data<T>(&self, value: &T) -> impl Iterator<Item = IterItem<'_, ID, P, D>>
    where
        T: Any + PartialEq,
    {
        self.iter().filter(move |item| match item {
            IterItem::Root(gerr) => gerr
                .data()
                .and_then(|data| (data as &dyn Any).downcast_ref::<T>())
                .is_some_and(|data| data == value),

            IterItem::GErr(gerr) => gerr
                .data
                .as_ref()
                .and_then(|data| (&**data as &dyn Any).downcast_ref::<T>())
                .is_some_and(|data| data == value),

            IterItem::Err(_) => false,
        })
    }

    #[inline]
    pub fn iter_source<E>(&self) -> impl Iterator<Item = IterItem<'_, ID, P, D>>
    where
        E: Error + 'static,
    {
        self.iter().filter_map(|item| match item {
            IterItem::Err(err) if err.is::<E>() => Some(item),

            IterItem::GErr(gerr) if (gerr as &dyn Error).is::<E>() => Some(item),

            _ => None,
        })
    }

    #[inline]
    pub fn find_id<T>(&self) -> Option<IterItem<'_, ID, P, D>>
    where
        T: Any,
    {
        self.iter_id::<T>().next()
    }

    #[inline]
    pub fn find_data<T>(&self) -> Option<IterItem<'_, ID, P, D>>
    where
        T: Any,
    {
        self.iter_data::<T>().next()
    }

    #[inline]
    pub fn find_source<E>(&self) -> Option<IterItem<'_, ID, P, D>>
    where
        E: Error + 'static,
    {
        self.iter_source::<E>().next()
    }
}
