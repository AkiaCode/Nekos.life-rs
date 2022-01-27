//! important types to interact with the API.

use crate::{
    types::{self, UrlString},
    Category, NekosLifeError,
};

// represents the response body
// for serde deserialization
#[derive(serde::Deserialize, Debug)]
pub(crate) struct ApiResponseBody {
    pub(crate) url: UrlString,
}

/// a trait that must be implemented to be passed by the [`get`] function.
///
/// this is slightly different from the [`TryInto`] trait in `std`,
/// since Rust does not allow to derive foreign traits to foreign types,\
/// such as implementing [`TryInto<url::Url>`] for [`&str`].
pub trait IntoUrl {
    /// Response type
    type Response;

    /// Future type for async method
    type Fut: std::future::Future<
        Output = types::Result<Self::Response>,
    >;

    /// consumes itself and returns a [`Result`](crate::types::Result) of [`UrlString`].
    fn into_url(self) -> types::Result<url::Url>;

    /// parse the body of the response
    fn parse(res: reqwest::Response) -> Self::Fut;
}

impl IntoUrl for &'static str {
    type Response = crate::types::UrlString;
    type Fut = std::pin::Pin<
        Box<
            dyn std::future::Future<
                Output = types::Result<Self::Response>,
            >,
        >,
    >;

    fn into_url(self) -> types::Result<url::Url> {
        Ok(string_to_endpoint!(
            Into::<&'static str>::into(
                &<Category as std::str::FromStr>::from_str(
                    self
                )
                .map_err(|error| {
                    NekosLifeError::UnknownEndpoint {
                        error,
                        url: self.to_owned(),
                    }
                })?
            )
        ))
    }

    fn parse(res: reqwest::Response) -> Self::Fut {
        Box::pin(async move {
            Ok(res.json::<ApiResponseBody>().await?.url)
        })
    }
}

#[cfg(test)]
mod tests {
    // use pretty_assertions::assert_eq;

    use {
        crate::{NekosLifeError, BASEURL},
        pretty_assertions::assert_eq,
    };

    #[test]
    fn string_to_url() -> Result<(), NekosLifeError> {
        use super::IntoUrl;

        Ok(assert_eq!(
            "Waifu".into_url()?.as_str(),
            format!(
                "{baseurl}img/waifu",
                baseurl = BASEURL.as_str()
            )
        ))
    }

    #[tokio::test]
    async fn parse_test() -> Result<(), NekosLifeError> {
        Ok(assert!(lazy_regex::regex_is_match!(
            r"^https://cdn\.nekos\.life/neko/[\w_.]+$",
            &<&str as super::IntoUrl>::parse(
                reqwest::get(
                    BASEURL.join("img/")?.join("neko")?
                )
                .await?
            )
            .await?
        )))
    }
}
