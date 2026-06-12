#[macro_export]
macro_rules! gerr {
    // plain message
    ($message:expr $(,)?) => {
        $crate::GErr::new($message)
    };

    // formatted message, no builder args
    ($fmt:expr, $($arg:expr),+ $(,)?) => {
        $crate::GErr::new(format!($fmt, $($arg),+))
    };

    // plain message + builder args
    (
        $message:expr ;
        $($rest:tt)*
    ) => {{
        let err = $crate::GErr::new($message);
        $crate::gerr!(@build err, $($rest)*)
    }};

    // formatted message + builder args
    (
        $fmt:expr,
        $($arg:expr),+ ;
        $($rest:tt)*
    ) => {{
        let err = $crate::GErr::new(format!($fmt, $($arg),+));
        $crate::gerr!(@build err, $($rest)*)
    }};

    (@build $err:ident,) => { $err };
    (@build $err:ident) => { $err };

    (@build $err:ident,
        id = $id:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.with_id($id);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    (@build $err:ident,
        data = $data:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.with_data($data);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    (@build $err:ident,
        source = $source:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.set_source($source);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    (@build $err:ident,
        prefix = $prefix:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.set_prefix($prefix);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    (@build $err:ident,
        tag = $tag:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.set_tag($tag);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};

    (@build $err:ident,
        tags = $tags:expr
        $(, $($rest:tt)*)?
    ) => {{
        let err = $err.set_tags($tags);
        $crate::gerr!(@build err $(, $($rest)*)?)
    }};
}
