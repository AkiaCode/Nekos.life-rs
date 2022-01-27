use {
    super::{super::types::IntoUrl, Category},
    crate::types,
};

impl IntoUrl for Category {
    type Response = types::UrlString;

    type Fut = std::pin::Pin<
        Box<
            dyn std::future::Future<
                Output = types::Result<Self::Response>,
            >,
        >,
    >;

    fn into_url(self) -> crate::types::Result<url::Url> {
        Ok(string_to_endpoint!(self.into()))
    }

    fn parse(res: reqwest::Response) -> Self::Fut {
        Box::pin(async move {
            Ok(res.json::<ApiResponseBody>().await?.url)
        })
    }
}

// represents the response body
// for serde deserialization
#[derive(serde::Deserialize, Debug)]
pub(crate) struct ApiResponseBody {
    pub(crate) url: types::UrlString,
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
                    crate::NekosLifeError::UnknownEndpoint {
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
mod tests;
