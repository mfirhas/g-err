use core::fmt::Debug;
use core::fmt::Display;
use g_err::GErr;
use g_err::GErrDefault;
use g_err::GErrSource;
use g_err::GResultExt;
use g_err::Id;
use g_err::NoData;
use g_err::NoID;
use g_err::NoPrefix;
use g_err::Prefix;
use g_err::Result;
use g_err::ResultExt;
use g_err::SetField;
extern crate alloc;
use alloc::borrow::Cow;
use core::panic::Location;
use uuid::Uuid;

#[path = "macro_test.rs"]
mod macro_test;

#[path = "constructor_test.rs"]
mod constructor_test;

#[path = "conversion_test.rs"]
mod conversion_test;

#[path = "prefix_test.rs"]
mod prefix_test;

#[path = "query_test.rs"]
mod query_test;

#[path = "report_test.rs"]
mod report_test;

#[path = "sources_test.rs"]
mod sources_test;

#[path = "tags_test.rs"]
mod tags_test;

fn parse_age(str_age: &str) -> Result<i32, NoID, PrefixB> {
    str_age.parse().into_gerr()
}

fn get_age(str_age: &str) -> Result<i32, UuidV4, NoPrefix> {
    let age = parse_age(str_age).with_gerr(|| format!("get age with input {}", str_age))?;
    Ok(age)
}

fn get_age_usecase(req_id: u32, input: &str) -> Result<u16, UuidV4, PrefixA, (&'static str, u32)> {
    let ret = get_age(input).map_err(|e| {
        GErr::new("get age usecase")
            .set_data(("req_id", req_id))
            .add_tag("usecase")
            .add_source_gerr(e)
    })?;
    let ret = u16::try_from(ret)
        .context::<NoID, PrefixB, NoData>("converting i32 to u16")
        .map_err(|gerr| GErr::new("into u16").add_source_gerr(gerr))?;
    Ok(ret)
}

fn handler(req_id: u32, input: &str) -> Result<u16, &'static str> {
    let ret = get_age_usecase(req_id, input).map_err(|e| {
        GErr::new_with_id("handler-123", "handler get age error").add_source_gerr(e)
    })?;
    Ok(ret)
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Debug)]
pub struct UuidV4(Uuid);

impl Display for UuidV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Id for UuidV4 {
    fn id() -> Self {
        UuidV4(Uuid::new_v4())
    }
}

// #[derive(Debug)]
struct PrefixA;
impl Prefix for PrefixA {
    const PREFIX: Option<&'static str> = Some("[Prefix_A]");
}
// #[derive(Debug)]
struct PrefixB;
impl Prefix for PrefixB {
    const PREFIX: Option<&'static str> = Some("[Prefix_B]");
}

#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[derive(Default)]
pub struct ErrData(serde_json::Map<String, serde_json::Value>);

impl core::fmt::Display for ErrData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self.0).unwrap_or("FORMAT_ERR".into())
        )
    }
}

impl core::fmt::Debug for ErrData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self.0).unwrap_or("FORMAT_ERR".into())
        )
    }
}

impl SetField<String, serde_json::Value> for ErrData {
    fn set_field(&mut self, key: String, value: serde_json::Value) {
        self.0.insert(key, value);
    }
}

#[test]
fn test() {
    let ret: GErr<NoID, PrefixA> = handler(123456, "anu")
        .gerr("handler error")
        .unwrap_err()
        .set_help("figure it out!");
    dbg!(&ret);
    println!("***************************************************************************");
    println!("Display:\n{}", &ret);
    println!("***************************************************************************");
    println!("Pretty:\n{}", &ret.report());
    println!("***************************************************************************");
    #[cfg(feature = "serde")]
    println!("JSON:\n{}", &ret.report_as::<g_err::json::JsonReport>());
    println!("***************************************************************************");
    #[cfg(feature = "serde")]
    println!(
        "Display JSON:\n{}",
        &ret.report_as::<g_err::json::DisplayJsonReport>()
    );
    println!("***************************************************************************");
    println!("Markdown:\n{}", &ret.report_as::<g_err::MarkdownReport>());
    println!("***************************************************************************");
    println!("Trace:\n{}", &ret.report_as::<g_err::TraceReport>());
    println!("***************************************************************************");
    // println!("Root cause:\n{}", &ret.root_cause());
    println!("***************************************************************************");

    let err = GErr::<NoID>::new("test").set_prefix("[HTTP]");
    let err2 = GErr::<NoID>::new("test2");
    let err3 = GErr::<NoID>::new("test3");
    let err4 = GErr::<NoID>::new("test4").add_tag("nganu");
    let err4_2 = GErr::<u16>::new_with_id(123, "test4_2").with_id("XYZ");
    let err5 = GErr::<NoID>::new("test5").set_prefix("[ASD]");
    let err4 = err4.add_source(err5);
    let err2 = err2.add_source_gerr(err4);
    let err2 = err2.add_source_gerr(err4_2);

    let err = err.add_source_gerr(err2);
    let err = err.add_source_gerr(err3);
    println!("{}", err.report_as::<g_err::TraceReport>());

    let ret = err.iter_by_tag("nganu").next().unwrap();
    println!("{}", ret);

    let ret = err.find_id::<&str>().unwrap();
    println!("--> {ret:?}");

    let ret = err.iter_by_prefix("[HTTP]").next().unwrap();
    println!("==> {ret}");

    let ret = err.iter_by_id(&"XYZ").next().unwrap();
    println!("++> {ret}");

    println!("{}", err.iter().count());
    for e in err.iter() {
        println!("{e}");
    }

    println!("GErrDefault = {}", size_of::<GErrDefault>());
    println!(
        "message                = {}",
        size_of::<Cow<'static, str>>()
    );
    println!(
        "prefix                 = {}",
        size_of::<Option<Cow<'static, str>>>()
    );
    println!(
        "sources                = {}",
        size_of::<Option<Vec<GErrSource>>>()
    );
    println!(
        "tags                   = {}",
        size_of::<Option<Vec<Cow<'static, str>>>>()
    );
    println!(
        "help                   = {}",
        size_of::<Option<Cow<'static, str>>>()
    );
    println!(
        "location               = {}",
        size_of::<&'static Location<'static>>()
    );
}
