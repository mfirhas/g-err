#[macro_export]
macro_rules! gerr {
    // message only
    ($message:expr $(,)?) => {
        $crate::GErr::new($message)
    };

    // message + optional fields
    (
        $message:expr
        $(, id = $id:expr)?
        $(, data = $data:expr)?
        $(, source = $source:expr)?
        $(, prefix = $prefix:expr)?
        $(, tag = $tag:expr)*
        $(, tags = $tags:expr)?
        $(,)?
    ) => {{
        let mut err = $crate::GErr::new($message);

        $(
            let err = err.set_id($id);
        )?

        $(
            let err = err.with_data($data);
        )?

        $(
            let err = err.set_source($source);
        )?

        $(
            let err = err.set_prefix($prefix);
        )?

        $(
            let err = err.set_tag($tag);
        )*

        $(
            let err = err.set_tags($tags);
        )?

        err
    }};
}
