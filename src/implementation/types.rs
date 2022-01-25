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
    /// consumes itself and returns a [`Result`](crate::types::Result) of [`UrlString`].
    fn into_url(self) -> types::Result<url::Url>;
}

impl IntoUrl for &'static str {
    fn into_url(self) -> types::Result<url::Url> {
        Ok(string_to_endpoint!(Into::<&'static str>::into(
            &<Category as std::str::FromStr>::from_str(
                self
            )
            .map_err(|error| {
                NekosLifeError::UnknownEndpoint {
                    error,
                    url: self.to_owned(),
                }
            })?
        )))
    }
}

#[cfg(test)]
mod tests {
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
}
