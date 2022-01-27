macro_rules! string_to_endpoint {
    ($category :expr) => {
        crate::r#static::BASEURL
            .join("img/")?
            .join($category)?
    };
}

macro_rules! parse_json {
    ($model :ty, $field :ident) => {
        fn parse(res: reqwest::Response) -> Self::Fut {
            Box::pin(async move {
                Ok(res.json::<$model>().await?.$field)
            })
        }
    };
}
