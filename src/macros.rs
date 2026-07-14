/// GErr macro.
///
/// Creates GErr easily with formatting support and rich data.
///
/// Without metadatas, the default config is [`DefaultConfig`] and default data is [`NoData`].
///
/// Error message and its metadata are separated by `;`.
///
/// # Supported metadata:
/// - `config`: infer auto-generating config type for id and code from return type, and auto-generate both.
/// - `config=$type`: set auto-generating config type for id and code, and auto-generate both.
/// - `id=$expr`: set id manually with id type as set by `config=$type`.
/// - `code=$expr`: set code string.
/// - `data_type`: infer error data type from return type.
/// - `data_type=$type`: define error data type.
/// - `data=$expr`: set data, along with its type.
/// - `source=$expr`: set non-GErr error source.
/// - `gerr=$expr`: set GErr error source, or any error convertible to GErrSource.
/// - `tag=$expr`: add tag.
/// - `tags=$expr`: add multiple tags, e.g: `["tag1", "tag2",...]`.
/// - `help=$expr`: set help message.
///
/// You can override the metadatas, and latest ones will be used.
///
/// # Example
/// ```rust
/// use g_err::gerr;
///
/// let inner = gerr!("parsing integer");
/// let external_error = "anu".parse::<i32>().unwrap_err();
/// let err = gerr!(
///     "failed {}",
///     500;
///     id = 999u32, // set id
///     code = "HTTP", // set code
///     tag = "server", // set a tag
///     tags = ["api", "v1"], // set tags
///     data = "payload", // set error data
///     code = "E-HTTP", // update code
///     source = external_error, // set general error as source
///     gerr = inner, // set `Into<GErrSource>` error as source
///     help = "Try parsing valid signed integer 32", // set help hint
/// );
///
/// assert_eq!(err.message(), "failed 500");
/// assert_eq!(err.id().unwrap(), &999);
/// assert_eq!(err.code(), Some("E-HTTP"));
/// assert_eq!(err.data(), Some(&"payload"));
///
/// let tags = err.tags().unwrap();
/// assert_eq!(tags.len(), 3);
///
/// let sources = err.sources().unwrap();
/// assert_eq!(sources.len(), 2);
/// ```
#[macro_export]
macro_rules! gerr {
    // ==================================================
    // Message only
    // ==================================================

    // format!-style
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::GErr::<
            $crate::DefaultConfig,
            $crate::NoData,
        >::new(format!($fmt $(, $arg)*))
    };

    // arbitrary string expression
    ($message:expr $(,)?) => {
        $crate::GErr::<
            $crate::DefaultConfig,
            $crate::NoData,
        >::new($message)
    };

    // ==================================================
    // Message + builder args
    // ==================================================

    (
        $fmt:literal $(, $arg:expr)* ;
        $($rest:tt)*
    ) => {{
        let err = $crate::GErr::<
            $crate::DefaultConfig,
            $crate::NoData,
        >::new(format!($fmt $(, $arg)*));

        $crate::gerr!(@build err, $($rest)*)
    }};

    (
        $message:expr ;
        $($rest:tt)*
    ) => {{
        let err = $crate::GErr::<
            $crate::DefaultConfig,
            $crate::NoData,
        >::new($message);

        $crate::gerr!(@build err, $($rest)*)
    }};

    // ==================================================
    // End recursion
    // ==================================================

    (@build $err:ident) => { $err };
    (@build $err:ident,) => { $err };

    // ==================================================
    // config
    // ==================================================

    (
        @build $err:ident,
        config
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.with_config();
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // config = ...
    // ==================================================

    (
        @build $err:ident,
        config = $config:ty
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.with_config::<$config>();
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // id = ...
    // ==================================================

    (
        @build $err:ident,
        id = $id:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.set_id($id);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // code = ...
    // ==================================================

    (
        @build $err:ident,
        code = $code:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.set_code($code);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // data_type
    // ==================================================

    (
        @build $err:ident,
        data_type
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.with_data_type();
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // data_type = ...
    // ==================================================

    (
        @build $err:ident,
        data_type = $data_type:ty
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.with_data_type::<$data_type>();
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // data = ...
    // ==================================================

    (
        @build $err:ident,
        data = $data:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.with_data($data);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // source = ...
    // ==================================================

    (
        @build $err:ident,
        source = $source:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.add_source($source);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // gerr = ...
    // ==================================================

    (
        @build $err:ident,
        gerr = $source:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.add_source_gerr($source);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // tag = ...
    // ==================================================

    (
        @build $err:ident,
        tag = $tag:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.add_tag($tag);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // tags = ...
    // ==================================================

    (
        @build $err:ident,
        tags = $tags:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.add_tags($tags);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // help = ...
    // ==================================================

    (
        @build $err:ident,
        help = $help:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.set_help($help);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};
}
