use crate::{GErr, GErrSource};
use core::error::Error;
use core::fmt::{Debug, Display};

pub type Result<T, ID = NoID, P = NoPrefix, D = NoData> = core::result::Result<T, GErr<ID, P, D>>;

pub trait Id {
    fn id() -> Self;
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Debug)]
pub struct NoID;

impl Display for NoID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoID")
    }
}

impl Id for NoID {
    #[inline]
    fn id() -> NoID {
        NoID
    }
}

pub trait Prefix {
    const PREFIX: Option<&'static str> = None;
}

#[derive(Debug)]
pub struct NoPrefix;

impl Prefix for NoPrefix {}

#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Debug)]
pub struct NoData;

pub trait SetField<K, V> {
    fn set_field(&mut self, key: K, value: V);
}

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum Source {
    Err(BoxError),
    GErr(Box<GErrSource>),
}
