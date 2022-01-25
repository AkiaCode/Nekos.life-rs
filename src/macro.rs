macro_rules! string_to_endpoint {
    ($category :expr) => {
        crate::r#static::BASEURL
            .join("img/")?
            .join($category)?
    };
}
