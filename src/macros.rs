/// GErr macro.
///
/// Creates GErr easily with formatting support and rich data.
///
/// Error message and its metadata are separated by `;`.
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
///     prefix = "HTTP", // set prefix
///     tag = "server", // set a tag
///     tags = ["api", "v1"], // set tags
///     data = "payload", // set error data
///     prefix = "[USER]", // update prefix
///     pprefix = "@", // prepend to existing prefix, or become one
///     aprefix = "[CREATE]", // append to existing prefix, or become one
///     source = external_error, // set general error as source
///     gerr = inner, // set `Into<GErrSource>` error as source
///     help = "Try parsing valid signed integer 32", // set help hint
/// );
///
/// assert_eq!(err.message(), "failed 500");
/// assert_eq!(*err.id(), 999);
/// assert_eq!(err.prefix(), Some("@[USER][CREATE]"));
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
            $crate::NoID,
            $crate::NoPrefix,
            $crate::NoData,
        >::new(format!($fmt $(, $arg)*))
    };

    // arbitrary string expression
    ($message:expr $(,)?) => {
        $crate::GErr::<
            $crate::NoID,
            $crate::NoPrefix,
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
            $crate::NoID,
            $crate::NoPrefix,
            $crate::NoData,
        >::new(format!($fmt $(, $arg)*));

        $crate::gerr!(@build err, $($rest)*)
    }};

    (
        $message:expr ;
        $($rest:tt)*
    ) => {{
        let err = $crate::GErr::<
            $crate::NoID,
            $crate::NoPrefix,
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
    // id = ...
    // ==================================================

    (
        @build $err:ident,
        id = $id:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.with_id($id);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // prefix = ...
    // ==================================================

    (
        @build $err:ident,
        prefix = $prefix:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.set_prefix($prefix);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // pprefix = ...
    // ==================================================

    (
        @build $err:ident,
        pprefix = $pprefix:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.prepend_prefix($pprefix);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // ==================================================
    // aprefix = ...
    // ==================================================

    (
        @build $err:ident,
        aprefix = $aprefix:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.append_prefix($aprefix);
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
