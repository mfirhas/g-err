use std::num::ParseIntError;

#[inline(never)]
fn func(input: &str) -> Result<i32, ParseIntError> {
    let ret = input.parse::<i32>()?;
    Ok(ret)
}

// ########### g_err ###########
pub mod g_err_bench {
    use super::*;

    use g_err::*;

    pub struct ErrI32;
    impl Config for ErrI32 {
        type Id = i32;
        const CODE: Option<&'static str> = Some("CODE");
    }

    pub struct ErrU64Auto;
    impl Config for ErrU64Auto {
        type Id = u64;
        #[inline]
        fn id() -> Option<Self::Id> {
            Some(3434)
        }
        const TAGS: Option<&'static [&'static str]> = Some(&["tag1"]);
    }

    #[inline(never)]
    fn func_gerr(input: &str) -> core::result::Result<i32, GErr<ErrI32>> {
        let ret = input
            .parse::<i32>()
            .to_gerr()
            .map_err(|err| err.set_id(234))?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn gerr_to_gerr(input: &str) -> core::result::Result<i32, GErrDefault> {
        let ret = func(input).to_gerr()?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn gerr_context_auto(input: &str) -> core::result::Result<i32, GErrDefault> {
        let ret = func(input).context_auto("gerr error")?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn gerr_context(input: &str) -> core::result::Result<i32, GErrDefault> {
        let ret = func(input).context(NoID, "gerr error")?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn gerr_wrap_err(input: &str) -> core::result::Result<i32, GErrDefault> {
        let ret = func(input).wrap_err(gerr!("gerr error: {}", input))?;
        Ok(ret)
    }

    // gerr

    #[inline(never)]
    pub fn gerr_to(input: &str) -> core::result::Result<i32, GErr<ErrU64Auto>> {
        let ret = func_gerr(input).to()?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn gerr_gerr_auto(input: &str) -> core::result::Result<i32, GErrDefault> {
        let ret = func_gerr(input).gerr_auto("gerr error")?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn gerr_gerr(input: &str) -> core::result::Result<i32, GErrDefault> {
        let ret = func_gerr(input).gerr(NoID, "gerr error")?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn gerr_wrap_gerr(input: &str) -> core::result::Result<i32, GErrDefault> {
        let ret = func_gerr(input).wrap_gerr(gerr!("gerr error: {}", input))?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn gerr_boxed(input: &str) -> core::result::Result<i32, GErrBox> {
        let ret = func_gerr(input)
            .wrap_gerr(gerr!("gerr error: {}", input))
            .boxed()?;
        Ok(ret)
    }
}

// ########### anyhow ###########
pub mod anyhow_bench {
    use super::*;

    use anyhow::{Context, bail};

    #[inline(never)]
    pub fn anyhow(input: &str) -> Result<i32, anyhow::Error> {
        let ret = func(input)?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn anyhow_bail(input: &str) -> Result<i32, anyhow::Error> {
        let ret = func(input);
        match ret {
            Ok(ret) => Ok(ret),
            Err(err) => bail!("anyhow bail error: {}", err),
        }
    }

    #[inline(never)]
    pub fn anyhow_context(input: &str) -> Result<i32, anyhow::Error> {
        let ret = func(input).context("anyhow error")?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn anyhow_context_with(input: &str) -> Result<i32, anyhow::Error> {
        let ret = func(input).with_context(|| format!("anyhow error: {}", 123))?;
        Ok(ret)
    }
}

// ########### snafu ###########
pub mod snafu_bench {
    use super::*;

    use snafu::FromString;
    use snafu::Whatever;
    use snafu::prelude::*;

    #[derive(Debug, Snafu)]
    #[snafu(display("Error with invalid id: {id}"))]
    pub struct InvalidIdError {
        id: String,
        source: ParseIntError,
    }

    #[inline(never)]
    pub fn snafu_whatever(input: &str) -> Result<i32, Whatever> {
        let ret = func(input);
        match ret {
            Ok(ret) => Ok(ret),
            Err(_) => Err(snafu::Whatever::without_source("snafu error".into())),
        }
    }

    #[inline(never)]
    pub fn snafu_whatever_source(input: &str) -> Result<i32, Whatever> {
        let ret = func(input);
        match ret {
            Ok(ret) => Ok(ret),
            Err(err) => Err(snafu::Whatever::with_source(
                err.into(),
                "snafu error".into(),
            )),
        }
    }

    #[inline(never)]
    pub fn snafu_whatever_macro(input: &str) -> Result<i32, snafu::Whatever> {
        let ret = func(input);
        match ret {
            Ok(ret) => Ok(ret),
            Err(_) => snafu::whatever!("snafu macro"),
        }
    }

    #[inline(never)]
    pub fn snafu_whatever_source_macro(input: &str) -> Result<i32, snafu::Whatever> {
        let ret = func(input);
        let ret = snafu::whatever!(ret, "snafu macro");
        Ok(ret)
    }

    #[inline(never)]
    pub fn snafu_whatever_macro_fmt(input: &str) -> Result<i32, snafu::Whatever> {
        let ret = func(input);
        match ret {
            Ok(ret) => Ok(ret),
            Err(_) => snafu::whatever!("snafu error with id: {}", input),
        }
    }

    #[inline(never)]
    pub fn snafu_whatever_source_macro_fmt(input: &str) -> Result<i32, snafu::Whatever> {
        let ret = func(input);
        let ret = snafu::whatever!(ret, "snafu error with id: {}", input);
        Ok(ret)
    }

    #[inline(never)]
    pub fn snafu_whatever_context(input: &str) -> Result<i32, snafu::Whatever> {
        let ret = func(input).whatever_context("whatever context error")?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn snafu_whatever_context_with(input: &str) -> Result<i32, snafu::Whatever> {
        let ret =
            func(input).with_whatever_context(|err| format!("whatever context error: {}", err))?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn snafu_context(input: &str) -> Result<i32, InvalidIdError> {
        let ret = func(input).context(InvalidIdSnafu { id: input })?;
        Ok(ret)
    }

    #[inline(never)]
    pub fn snafu_context_with(input: &str) -> Result<i32, InvalidIdError> {
        let ret = func(input).with_context(|_| InvalidIdSnafu { id: input })?;
        Ok(ret)
    }
}
