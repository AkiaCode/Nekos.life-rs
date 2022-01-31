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

/// Type autocomplete for [`IntoUrl`](crate::IntoUrl) implementation.
///
/// This is just utility macro for easy implementing.
///
/// # Examples
///
/// ```rust
/// struct MyEndpoint;
///
/// impl nekoslife::IntoUrl for MyEndpoint {
///     type Response = ();
///
///     // this will be Pin<Box<Future>>
///     type Fut = nekoslife::into_url_fut!();
///
///     fn into_url(self) -> nekoslife::Result<url::Url> {
///         unimplemented!()
///     }
///
///     fn parse(res: reqwest::Response) -> Self::Fut {
///         unimplemented!()
///     }
/// }
/// ```
#[macro_export]
#[allow(missing_docs)]
macro_rules! into_url_fut {
    () => {
        std
        ::pin
        ::Pin<
            Box<
                dyn std
                ::future
                ::Future<
                    Output = $
                    crate
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
        $ i: tt
        || $ _: tt
    ) => {
        $ i
    };
    (|| $ i: tt) => {
        $ i
    };
    (
        $ i: tt
        ||= $ _: tt >=>
        $ cb: ident |>
        $ (
            $ arg: tt
        ) *
    ) => {
        $ cb! {
            $ (
                $ arg
            ) *
            $ i
        }
    };
    (
        ||= $ i: tt >=>
        $ cb: ident |>
        $ (
            $ arg: tt
        ) *
    ) => {
        $ cb! {
            $ (
                $ arg
            ) *
            $ i
        }
    };
    (
        |-> $ endpoint :ident
        ->> $ field :ident
    ) => {
        paste::paste! {
            #[allow(
                missing_docs
            )] #[derive(
                serde::Deserialize
            )] pub(
                crate
            ) struct [<
                $ endpoint
                Response
            >] {
                $ field: String,
            }
        }
    };
    (
        $ (
            $ (
                #[
                    $ attr :meta
                ]
            ) *
            $ endpoint :ident
            $ (
                @ $ l :lifetime
            ) ?
            $ (
                |>
                    $ (
                        $ inner_type: ty
                    ) ?
                    :=
                    $ (
                        $ inner :ident
                    ) ?
                <|
            ) ?
            $ (
                ~~> $ ep_alias :ident
            ) ?
            $ (
                !# $range :expr
            ) ?
        ) => +
    ) => {
        $ (
            #[derive(
                Debug,
                PartialEq,
                Eq,
                Clone,
                Copy,
            )] $ (
                #[
                    $ attr
                ]
            ) *
            pub struct $ endpoint $ (
                <
                    $ l
                >
            ) ? $ (
                (
                    pub & $ l
                    $ (
                        $ inner_type
                    ) ?
                )
            ) ? ;

            paste::paste! {
                make_text_endpoints! {
                    $ (
                        $ (
                            $ inner
                        ) ?
                    ) ?
                        ||=
                    [<
                        $ endpoint :lower
                    >]
                        >=>
                    make_text_endpoints |>
                        |-> $ endpoint
                        ->>
                }
            }

            impl $ (
                <
                    $ l
                >
            ) ? $ crate
                ::implementation
                ::types
                ::IntoUrl
            for $ endpoint $ (
                <
                    $ l
                >
            ) ? {
                type Response = $ crate::types::UrlString;

                type Fut = into_url_fut! {};

                fn into_url(
                    self,
                ) -> $ crate::Result<
                    url::Url
                > {
                    paste::paste! {
                        Ok({
                            #[allow(
                                unused_mut
                            )] let mut url: url::Url =
                                $ crate
                                    ::r#static
                                    ::BASEURL
                                        .join(
                                            make_text_endpoints! {
                                                $ (
                                                    $ ep_alias
                                                ) ?
                                                    ||=
                                                [<
                                                    $ endpoint :lower
                                                >]
                                                    >=>
                                                stringify |>
                                            }
                                        ) ? ;
                            $ (
                                $ (
                                    url
                                        .query_pairs_mut()
                                        .append_pair(
                                            "text",
                                            if matches! {
                                                (
                                                    self.0 as
                                                    &$ l $ inner_type
                                                )
                                                    .len(),
                                                $ range
                                            } { self.0 } else {
                                                return Err($crate
                                                    ::Error
                                                    ::OutOfRangeError {
                                                    endpoint_name: stringify! {
                                                        $ endpoint
                                                    }.to_owned(),
                                                    range: $ range,
                                                })
                                            }
                                        ) ;
                                ) ?
                            ) ?

                            url
                        })
                    }
                }

                paste::paste! {
                    make_text_endpoints! {
                        $ (
                            $ (
                                $ inner
                            ) ?
                        ) ?
                            ||=
                        [<
                            $ endpoint :lower
                        >]
                            >=>
                        parse_json |>
                            [<
                                $ endpoint
                                Response
                            >]
                            ,
                    }
                }
            }
        ) +
    };
}

macro_rules! pair {
    (
        $ (
            #[
                $ item_attr :meta
            ]
        ) *
        $ vis :vis
        enum
        $ name :ident
        {
            $ (
                $ (
                    #[
                        $ attr :meta
                    ]
                ) *
                $ field :ident
                <
                    $ raw :literal
                >
            ) , *
            $ ( , ) ?
        }
    ) => {
        $ (
            #[
                $ item_attr
            ]
        ) *
        $ vis
        enum $ name {
            $ (
                #[
                    serde(
                        rename = $ raw
                    )
                ] #[allow(
                    missing_docs
                )] $ (
                    #[
                        $ attr
                    ]
                ) * $ field
            ) , *
        }

        impl std::fmt::Display for $ name {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                match self {
                    $ (
                        $ name::$ field => write!(
                            f,
                            $ raw
                        ),
                    ) *
                }
            }
        }

        impl From<
            $ name
        > for &'static str {
            fn from(en: $ name) -> &'static str {
                match en {
                    $ (
                        $ name::$ field => $ raw,
                    ) *
                }
            }
        }
    };
}
