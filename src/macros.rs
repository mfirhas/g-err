#[macro_export]
macro_rules! gerr {
    // --------------------------------------------------
    // format-style message only
    // --------------------------------------------------

    ($($msg:tt)+) => {
        $crate::GErr::<
            $crate::NoID,
            $crate::NoPrefix,
            $crate::NoData,
        >::new(format!($($msg)+))
    };

    // --------------------------------------------------
    // format-style message + builder args
    // --------------------------------------------------

    (
        $($msg:tt)+ ;
        $($rest:tt)*
    ) => {{
        let err = $crate::GErr::<
            $crate::NoID,
            $crate::NoPrefix,
            $crate::NoData,
        >::new(format!($($msg)+));

        $crate::gerr!(@build err, $($rest)*)
    }};

    // --------------------------------------------------
    // end recursion
    // --------------------------------------------------

    (@build $err:ident) => { $err };
    (@build $err:ident,) => { $err };

    // --------------------------------------------------
    // id = ...
    // --------------------------------------------------

    (
        @build $err:ident,
        id = $id:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.with_id($id);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // --------------------------------------------------
    // prefix = ...
    // --------------------------------------------------

    (
        @build $err:ident,
        prefix = $prefix:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.set_prefix($prefix);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // --------------------------------------------------
    // data = ...
    // --------------------------------------------------

    (
        @build $err:ident,
        data = $data:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.with_data($data);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // --------------------------------------------------
    // source = ...
    // --------------------------------------------------

    (
        @build $err:ident,
        source = $source:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.add_source($source);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // --------------------------------------------------
    // tag = ...
    // --------------------------------------------------

    (
        @build $err:ident,
        tag = $tag:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.add_tag($tag);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    // --------------------------------------------------
    // tags = ...
    // --------------------------------------------------

    (
        @build $err:ident,
        tags = $tags:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.set_tags($tags);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};
}
