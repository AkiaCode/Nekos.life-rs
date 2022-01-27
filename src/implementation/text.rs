use crate::NekosLifeError;

use {crate::r#static::BASEURL, serde::Deserialize};

// struct Cat;

// #[derive(Deserialize)]
// pub struct CatModel {
//     cat: crate::UrlString,
// }

// impl super::types::IntoUrl for Cat {
//     type Response = crate::types::UrlString;

//     type Fut = into_url_fut! {};

//     fn into_url(self) -> crate::types::Result<url::Url> {
//         Ok(BASEURL.join("cat")?)
//     }

//     parse_json! {
//         CatModel,
//         cat,
//     }
// }

make_text_endpoints! {
    #[derive(Debug)]
    Cat
}

make_text_endpoints! {
    Dog @ 'a |> str := owo <| ~~> owoify
}
// ~~> ddddog

// #[meta]
// Cat
// OwOify @ 'a |> pub &'a str := owo <| ~~> cat #! 1..=200

struct OwOify<'a>(pub &'a str);

#[derive(Deserialize)]
pub struct OwOifyModel {
    owo: crate::UrlString,
}

impl<'a> super::types::IntoUrl for OwOify<'a> {
    type Response = crate::types::UrlString;

    type Fut = into_url_fut! {};

    fn into_url(self) -> crate::types::Result<url::Url> {
        Ok({
            let mut url = BASEURL.join("owoify")?;

            url.query_pairs_mut().append_pair(
                "text",
                if matches! {
                    self.0.len(),
                    1..=200
                } {
                    self.0
                } else {
                    Err(NekosLifeError::OutOfRangeError {
                        endpoint_name: "OwOify".to_owned(),
                        range: 1..=200,
                    })?
                },
            );
            url
        })
    }

    parse_json! {
        OwOifyModel,
        owo
    }
}

struct Why;

#[derive(Deserialize)]
pub struct WhyModel {
    why: crate::UrlString,
}

impl super::types::IntoUrl for Why {
    type Response = crate::types::UrlString;

    type Fut = into_url_fut! {};

    fn into_url(self) -> crate::types::Result<url::Url> {
        Ok(BASEURL.join("why")?)
    }

    parse_json! {
        WhyModel,
        why,
    }
}

struct Fact;

#[derive(Deserialize)]
pub struct FactModel {
    fact: crate::UrlString,
}

impl super::types::IntoUrl for Fact {
    type Response = crate::types::UrlString;

    type Fut = into_url_fut! {};

    fn into_url(self) -> crate::types::Result<url::Url> {
        Ok(BASEURL.join("fact")?)
    }

    parse_json! {
        FactModel,
        fact,
    }
}

struct Spoiler<'a>(pub &'a str);

#[derive(Deserialize)]
pub struct SpoilerModel {
    owo: crate::UrlString,
}

impl<'a> super::types::IntoUrl for Spoiler<'a> {
    type Response = crate::types::UrlString;

    type Fut = into_url_fut! {};

    fn into_url(self) -> crate::types::Result<url::Url> {
        Ok({
            let mut url = BASEURL.join("spoiler")?;

            url.query_pairs_mut().append_pair(
                "text",
                if matches! {
                    self.0.len(),
                    1..=1000
                } {
                    self.0
                } else {
                    Err(NekosLifeError::OutOfRangeError {
                        endpoint_name: "OwOify".to_owned(),
                        range: 1..=200,
                    })?
                },
            );
            url
        })
    }

    parse_json! {
        SpoilerModel,
        owo
    }
}

#[cfg(test)]
mod tests;
