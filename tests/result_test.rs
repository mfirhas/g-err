use g_err::{iterator::GErrNode, *};
#[path = "setup_test.rs"]
mod setup_test;
use setup_test::*;

#[path = "result_test_data.rs"]
mod result_test_data;

fn parse_age(str_age: &str) -> Result<i32, ErrIDi32, Data> {
    str_age
        .parse()
        .context(4235, "failed parsing age")
        .map_err(|gerr| {
            gerr.set_code("[REPO]").set_data(Data {
                user_id: 123,
                user_name: "ajo".into(),
            })
        })
}

fn get_age(str_age: &str) -> Result<i32, ErrAutoIDCode, (String, String)> {
    let age = parse_age(str_age)
        .gerr_auto("failed getting age")
        .map_err(|gerr| gerr.set_data(("age".into(), str_age.into())))?;
    Ok(age)
}

fn parse_email(email: &str) -> Result<String, ErrAutoID, NoData> {
    if !email.contains("@") {
        return gerr!("invalid email: {}", email; config=ErrAutoID, code="[REPO]")
            .result()
            .context_auto("failed parsing email");
    }
    Ok(email.into())
}

fn get_email(email: &str) -> Result<String, ErrAutoCode, Data> {
    parse_email(email).wrap_err(gerr!("failed getting email: {}", email; config, data=Data { user_id: 123, user_name: "ajo".into() }))
}

fn use_case(
    email: &str,
    age: &str,
) -> core::result::Result<(String, i32), GErr<ErrIDStrAutoCode, NoData>> {
    let age = get_age(age);

    let email = get_email(email);

    let ret = match (age, email) {
        (Ok(a), Ok(e)) => Ok((e, a)),
        (Ok(_), Err(ee)) => Err(ee),
        (Err(ae), Ok(_)) => Err(ae.with_config::<ErrAutoCode>().with_data(Data {
            user_id: 34,
            user_name: "asd".into(),
        })),
        (Err(ae), Err(ee)) => Err(
            GErr::<ErrAutoCode, Data>::new("failed both get age and email")
                .add_source_gerr(ae)
                .add_source_gerr(ee),
        ),
    };

    ret.gerr("E-001", "failed getting age or email")
}

#[rustfmt::skip]
fn handler(email: &str, age: &str) -> core::result::Result<String, GErrBox<ErrAutoIDCode, NoData>> {
    let ret = use_case(email, age).wrap_gerr(gerr!("handler get email and age error";
        config,
        tags=["http", "handler", "email", "age"],
        help="please pass valid email and age",
    )).boxed()?;

    Ok(format!("email: {} and age: {}", ret.0, ret.1))
}

#[test]
fn test_result_ext() {
    use result_test_data::test_result_ext_data;
    let email = "dono.the_dono_gmail.com";
    let age = "52!";
    let ret = handler(email, age).unwrap_err();

    assert_eq!(ret.id().unwrap(), &AutoID);
    assert_eq!(ret.code().unwrap(), "AutoCode");
    let debug = format!("{:#?}", ret);
    assert_eq!(debug, test_result_ext_data::EXPECTED_DEBUG);
}

fn func1() -> core::result::Result<i32, GErr<ErrAutoIDCode>> {
    let ret = func2().to_gerr()?;
    Ok(ret)
}

use std::num::ParseIntError;

fn func2() -> core::result::Result<i32, ParseIntError> {
    "qwe".parse::<i32>()
}

#[test]
fn test_result_to_gerr() {
    let expected_line = 91;
    let expected_column = 23;
    let gerr = func1().unwrap_err();
    assert_eq!(gerr.id().unwrap(), &AutoID);
    assert_eq!(gerr.code().unwrap(), "AutoCode");
    assert_eq!(gerr.message(), "invalid digit found in string");
    assert_eq!(gerr.location().file, file!());
    assert_eq!(gerr.location().line, expected_line);
    assert_eq!(gerr.location().column, expected_column);
}

struct ErrFuncA;
impl Config for ErrFuncA {
    type Id = i32;
    const CODE: Option<&'static str> = Some("CODE_A");
}

struct ErrFuncB;
impl Config for ErrFuncB {
    type Id = u64;
    fn id() -> Option<Self::Id> {
        Some(4000)
    }

    const TAGS: Option<&'static [&'static str]> = Some(&["tag1", "tag2"]);
}

fn func_a(input: &str) -> core::result::Result<i32, GErr<ErrFuncA, (&'static str, String)>> {
    let ret = input.parse::<i32>();

    match ret {
        Ok(num) => Ok(num),
        Err(err) => GErr::new_with_id(123, "func_a error")
            .add_source(err)
            .set_data(("anu", input.into()))
            .result(),
    }
}

fn func_b(input: &str) -> core::result::Result<i64, GErr<ErrFuncB>> {
    let ret = func_a(input).to()?;
    Ok(ret as i64)
}

#[test]
fn test_result_to() {
    let input = "qwe";
    let func_a_ret = func_a(input).unwrap_err();
    assert_eq!(func_a_ret.id().unwrap(), &123);
    if let GErrNode::LeafErr(e) = func_a_ret.iter_source::<ParseIntError>().next().unwrap() {
        assert_eq!(e.to_string(), "invalid digit found in string");
    }
    assert_eq!(func_a_ret.data().unwrap(), &("anu", String::from("qwe")));
    assert!(func_a_ret.tags().is_none());
    assert_eq!(func_a_ret.location().file, file!());

    let func_b_ret = func_b(input).unwrap_err();
    assert_eq!(func_b_ret.id().unwrap(), &4000);
    if let GErrNode::LeafErr(e) = func_b_ret.iter_source::<ParseIntError>().next().unwrap() {
        assert_eq!(e.to_string(), "invalid digit found in string");
    }
    assert!(func_b_ret.data().is_none());
    assert!(func_b_ret.iter_tags().eq(["tag1", "tag2"]));
    assert_eq!(func_b_ret.location().file, file!());
}
