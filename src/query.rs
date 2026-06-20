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
    pub fn contains_tag(&self, tag: &str) -> bool {
        self.iter().any(|item| match item {
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
    pub fn ids<T>(&self) -> impl Iterator<Item = &T>
    where
        T: Any,
    {
        self.iter().filter_map(|item| match item {
            IterItem::Root(gerr) => (gerr.id() as &dyn Any).downcast_ref::<T>(),

            IterItem::GErr(gerr) => (&*gerr.id as &dyn Any).downcast_ref::<T>(),

            IterItem::Err(_) => None,
        })
    }

    #[inline]
    pub fn datas<T>(&self) -> impl Iterator<Item = &T>
    where
        T: Any,
    {
        self.iter().filter_map(|item| match item {
            IterItem::Root(gerr) => gerr
                .data()
                .and_then(|data| (data as &dyn Any).downcast_ref::<T>()),

            IterItem::GErr(gerr) => gerr
                .data
                .as_ref()
                .and_then(|data| (&**data as &dyn Any).downcast_ref::<T>()),

            IterItem::Err(_) => None,
        })
    }

    #[inline]
    pub fn sources_of<E>(&self) -> impl Iterator<Item = &E>
    where
        E: Error + 'static,
    {
        self.iter().filter_map(|item| match item {
            IterItem::Err(err) => err.downcast_ref::<E>(),

            IterItem::GErr(gerr) => (gerr as &dyn Error).downcast_ref::<E>(),

            IterItem::Root(_) => None,
        })
    }

    #[inline]
    pub fn find_id<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        self.ids::<T>().next()
    }

    #[inline]
    pub fn find_data<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        self.datas::<T>().next()
    }

    #[inline]
    pub fn find_source<E>(&self) -> Option<&E>
    where
        E: Error + 'static,
    {
        self.sources_of::<E>().next()
    }
}
