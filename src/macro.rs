macro_rules! string_to_endpoint {
    ($ category :expr) => {
        crate::r#static::BASEURL
            .join("img/")?
            .join($category)?
    };
}

#[rustfmt::skip]
macro_rules! parse_json {
    (
        $ model :ty,
        $ field :ident
        $ (
            ,
        ) ?
    ) => {
        fn parse(
            res: reqwest::Response
        ) -> Self::Fut {
            Box::pin(async move {
                Ok(
                    res
                        .json::<$
                            model
                        >()
                        .await?.$
                        field
                )
            })
        }
    };
}

macro_rules! into_url_fut {
    () => {
        std
        ::pin
        ::Pin<
            Box<
                dyn std
                ::future
                ::Future<
                    Output = crate
                    ::types
                    ::Result<
                        Self::Response
                    >,
                >,
            >
        >
    };
}

macro_rules! make_text_endpoints {
    (
        $ endpoint :ident
    ) => {
        use paste::paste;

        pub struct $endpoint;

        paste! {
            #[derive(Deserialize)]
            pub struct [<
                $endpoint
                Model
            >] {
                [<
                    $endpoint :lower
                >]: crate::UrlString,
            }
        }

        impl super::types::IntoUrl for $endpoint {
            type Response = crate::types::UrlString;

            type Fut = into_url_fut! {};

            fn into_url(
                self,
            ) -> crate::types::Result<url::Url> {
                paste! {
                    Ok(
                        crate::r#static::BASEURL
                            .join(
                                stringify!(
                                    [<
                                        $endpoint :lower
                                    >]
                                )
                            )?
                    )
                }
            }

            paste! {
                parse_json! {
                    [<
                        $endpoint
                        Model
                    >],
                    [<
                        $endpoint :lower
                    >],
                }
            }
        }
    };
}
