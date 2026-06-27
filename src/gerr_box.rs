use crate::{GErr, NoData, NoID, NoPrefix};

pub type GErrBox<ID = NoID, P = NoPrefix, D = NoData> = Box<GErr<ID, P, D>>;
