use core::mem::size_of;

use g_err::{
    Config, DataSource, DefaultConfig, ErrorLocation, GErr, GErrBox, GErrDefault, GErrSource,
    IdSource, NoData, Source,
};

#[test]
fn print_type_sizes() {
    let size_of_default_gerr = size_of::<GErrDefault>();
    println!("{}", size_of_default_gerr);
}

use std::borrow::Cow;

#[test]
fn print_gerr_layout() {
    use core::mem::{align_of, size_of};

    println!("=== GErr<DefaultConfig, NoData> Layout ===");

    println!(
        "id         : {:>3} bytes (align {})",
        size_of::<Option<<DefaultConfig as Config>::Id>>(),
        align_of::<Option<<DefaultConfig as Config>::Id>>(),
    );
    assert_eq!(size_of::<Option<<DefaultConfig as Config>::Id>>(), 1);
    assert_eq!(align_of::<Option<<DefaultConfig as Config>::Id>>(), 1);

    println!(
        "code       : {:>3} bytes (align {})",
        size_of::<Option<Cow<'static, str>>>(),
        align_of::<Option<Cow<'static, str>>>(),
    );
    assert_eq!(size_of::<Option<Cow<'static, str>>>(), 24);
    assert_eq!(align_of::<Option<Cow<'static, str>>>(), 8);

    println!(
        "message    : {:>3} bytes (align {})",
        size_of::<Cow<'static, str>>(),
        align_of::<Cow<'static, str>>(),
    );
    assert_eq!(size_of::<Cow<'static, str>>(), 24);
    assert_eq!(align_of::<Cow<'static, str>>(), 8);

    println!(
        "sources    : {:>3} bytes (align {})",
        size_of::<Option<Vec<Source>>>(),
        align_of::<Option<Vec<Source>>>(),
    );
    assert_eq!(size_of::<Option<Vec<Source>>>(), 24);
    assert_eq!(align_of::<Option<Vec<Source>>>(), 8);

    println!(
        "tags       : {:>3} bytes (align {})",
        size_of::<Option<Vec<Cow<'static, str>>>>(),
        align_of::<Option<Vec<Cow<'static, str>>>>(),
    );
    assert_eq!(size_of::<Option<Vec<Cow<'static, str>>>>(), 24);
    assert_eq!(align_of::<Option<Vec<Cow<'static, str>>>>(), 8);

    println!(
        "data       : {:>3} bytes (align {})",
        size_of::<Option<NoData>>(),
        align_of::<Option<NoData>>(),
    );
    assert_eq!(size_of::<Option<NoData>>(), 1);
    assert_eq!(align_of::<Option<NoData>>(), 1);

    println!(
        "help       : {:>3} bytes (align {})",
        size_of::<Option<Cow<'static, str>>>(),
        align_of::<Option<Cow<'static, str>>>(),
    );
    assert_eq!(size_of::<Option<Cow<'static, str>>>(), 24);
    assert_eq!(align_of::<Option<Cow<'static, str>>>(), 8);

    println!(
        "location   : {:>3} bytes (align {})",
        size_of::<ErrorLocation>(),
        align_of::<ErrorLocation>(),
    );
    assert_eq!(size_of::<ErrorLocation>(), 32);
    assert_eq!(align_of::<ErrorLocation>(), 8);

    #[cfg(feature = "backtrace")]
    {
        println!(
            "backtrace  : {:>3} bytes (align {})",
            size_of::<std::backtrace::Backtrace>(),
            align_of::<std::backtrace::Backtrace>(),
        );
        assert_eq!(size_of::<std::backtrace::Backtrace>(), 48);
        assert_eq!(align_of::<std::backtrace::Backtrace>(), 8);
    }

    println!("---------------------------------");

    #[cfg(not(feature = "backtrace"))]
    let expected_total = 160;
    #[cfg(feature = "backtrace")]
    let expected_total = 208;

    println!(
        "GErr       : {:>3} bytes (align {})",
        size_of::<GErr<DefaultConfig, NoData>>(),
        align_of::<GErr<DefaultConfig, NoData>>(),
    );
    assert_eq!(size_of::<GErr<DefaultConfig, NoData>>(), expected_total);
    assert_eq!(align_of::<GErr<DefaultConfig, NoData>>(), 8);

    println!("---------------------------------");

    println!(
        "GErrBox    : {:>3} bytes (align {})",
        size_of::<GErrBox<DefaultConfig, NoData>>(),
        align_of::<GErrBox<DefaultConfig, NoData>>(),
    );
    assert_eq!(size_of::<GErrBox<DefaultConfig, NoData>>(), 8);
    assert_eq!(align_of::<GErrBox<DefaultConfig, NoData>>(), 8);
}

#[test]
fn print_gerr_source_layout() {
    use core::mem::{align_of, size_of};
    use std::borrow::Cow;

    println!("=== GErrSource Layout ===");

    println!(
        "id         : {:>3} bytes (align {})",
        size_of::<Option<Box<dyn IdSource>>>(),
        align_of::<Option<Box<dyn IdSource>>>(),
    );
    assert_eq!(size_of::<Option<Box<dyn IdSource>>>(), 16);
    assert_eq!(align_of::<Option<Box<dyn IdSource>>>(), 8);

    #[cfg(feature = "serde")]
    {
        println!(
            "id_json    : {:>3} bytes (align {})",
            size_of::<Option<serde_json::Value>>(),
            align_of::<Option<serde_json::Value>>(),
        );
    }

    println!(
        "code       : {:>3} bytes (align {})",
        size_of::<Option<Cow<'static, str>>>(),
        align_of::<Option<Cow<'static, str>>>(),
    );
    assert_eq!(size_of::<Option<Cow<'static, str>>>(), 24);
    assert_eq!(align_of::<Option<Cow<'static, str>>>(), 8);

    println!(
        "message    : {:>3} bytes (align {})",
        size_of::<Cow<'static, str>>(),
        align_of::<Cow<'static, str>>(),
    );
    assert_eq!(size_of::<Cow<'static, str>>(), 24);
    assert_eq!(align_of::<Cow<'static, str>>(), 8);

    println!(
        "sources    : {:>3} bytes (align {})",
        size_of::<Option<Vec<Source>>>(),
        align_of::<Option<Vec<Source>>>(),
    );
    assert_eq!(size_of::<Option<Vec<Source>>>(), 24);
    assert_eq!(align_of::<Option<Vec<Source>>>(), 8);

    println!(
        "tags       : {:>3} bytes (align {})",
        size_of::<Option<Vec<Cow<'static, str>>>>(),
        align_of::<Option<Vec<Cow<'static, str>>>>(),
    );
    assert_eq!(size_of::<Option<Vec<Cow<'static, str>>>>(), 24);
    assert_eq!(align_of::<Option<Vec<Cow<'static, str>>>>(), 8);

    println!(
        "data       : {:>3} bytes (align {})",
        size_of::<Option<Box<dyn DataSource>>>(),
        align_of::<Option<Box<dyn DataSource>>>(),
    );
    assert_eq!(size_of::<Option<Box<dyn DataSource>>>(), 16);
    assert_eq!(align_of::<Option<Box<dyn DataSource>>>(), 8);

    println!(
        "help       : {:>3} bytes (align {})",
        size_of::<Option<Cow<'static, str>>>(),
        align_of::<Option<Cow<'static, str>>>(),
    );
    assert_eq!(size_of::<Option<Cow<'static, str>>>(), 24);
    assert_eq!(align_of::<Option<Cow<'static, str>>>(), 8);

    #[cfg(feature = "serde")]
    {
        println!(
            "data_json  : {:>3} bytes (align {})",
            size_of::<Option<serde_json::Value>>(),
            align_of::<Option<serde_json::Value>>(),
        );
    }

    println!(
        "location   : {:>3} bytes (align {})",
        size_of::<Option<ErrorLocation>>(),
        align_of::<Option<ErrorLocation>>(),
    );
    assert_eq!(align_of::<Option<ErrorLocation>>(), 8);

    println!("---------------------------------");

    println!(
        "GErrSource : {:>3} bytes (align {})",
        size_of::<GErrSource>(),
        align_of::<GErrSource>(),
    );
}
