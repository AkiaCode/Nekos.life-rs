use {lazy_static::lazy_static, url::Url};

/// Any error that can occur while accessing the api.
#[derive(thiserror::Error, Debug)]
pub enum NekosLifeError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("invalid url was provided")]
    UrlParseError(#[from] url::ParseError),
}

lazy_static! {
    /// The base api url.
    static ref BASEURL: Url =
        Url::parse("https://nekos.life/api/v2/").expect("Invalid base url");
}

#[cfg(feature = "nsfw")]
mod nsfw;
#[cfg(feature = "sfw")]
mod sfw;

#[cfg(feature = "nsfw")]
pub use nsfw::NsfwCategory;
#[cfg(feature = "sfw")]
pub use sfw::SfwCategory;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    /// A nsfw category.
    #[cfg(feature = "nsfw")]
    Nsfw(NsfwCategory),
    /// A sfw category.
    #[cfg(feature = "sfw")]
    Sfw(SfwCategory),
}

impl Category {
    /// Gets the path to append after [`BASEURL`]+/img/ to make a request to get an image / gif url.
    /// # Examples
    /// ```rust
    /// # use nekoslife::{Category, SfwCategory};
    /// assert_eq!(Category::from(SfwCategory::Waifu).to_url_path(), "waifu");
    /// ```
    pub fn to_url_path(self) -> &'static str {
        match self {
            #[cfg(feature = "nsfw")]
            Self::Nsfw(c) => c.into(),
            #[cfg(feature = "sfw")]
            Self::Sfw(c) => c.into(),
        }
    }
}

#[cfg(feature = "nsfw")]
impl From<NsfwCategory> for Category {
    fn from(c: NsfwCategory) -> Self {
        Self::Nsfw(c)
    }
}

#[cfg(feature = "sfw")]
impl From<SfwCategory> for Category {
    fn from(c: SfwCategory) -> Self {
        Self::Sfw(c)
    }
}

#[cfg(feature = "blocking")]
mod implementation {
    use super::*;

    /// Gets the url of an image / gif from the api, from the given category,
    /// and using the given client.
    /// # Examples
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = reqwest::blocking::Client::new();
    ///     let url: String = nekoslife::get_with_client(&client, nekoslife::SfwCategory::Waifu)?;
    /// #   Ok(())
    /// # }
    pub fn get_with_client(
        client: &reqwest::blocking::Client,
        category: impl Into<Category>,
    ) -> Result<String, NekosLifeError> {
        let category = category.into();

        #[derive(serde::Deserialize)]
        struct Response {
            url: String,
        }

        let resp = client
            .get(BASEURL.join("img/")?.join(category.to_url_path())?)
            .send()?
            .json::<Response>()?;

        Ok(resp.url)
    }

    /// Gets the url of an image / gif from the api, from the given category,
    /// and using the default client.
    /// # Examples
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url: String = nekoslife::get(nekoslife::SfwCategory::Waifu)?;
    /// #   Ok(())
    /// # }
    pub fn get(
        category: impl Into<Category>,
    ) -> Result<String, NekosLifeError> {
        let client = reqwest::blocking::Client::new();

        get_with_client(&client, category)
    }
}

#[cfg(not(feature = "blocking"))]
mod implementation {
    use super::*;

    /// Gets the url of an image / gif from the api, from the given category,
    /// and using the given client.
    /// # Examples
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = reqwest::Client::new();
    ///     let url: String = nekoslife::get_with_client(&client, nekoslife::SfwCategory::Waifu).await?;
    /// #   Ok(())
    /// # }
    pub async fn get_with_client(
        client: &reqwest::Client,
        category: impl Into<Category>,
    ) -> Result<String, NekosLifeError> {
        let category = category.into();

        #[derive(serde::Deserialize)]
        struct Response {
            url: String,
        }

        let resp = client
            .get(BASEURL.join("img/")?.join(category.to_url_path())?)
            .send()
            .await?
            .json::<Response>()
            .await?;

        Ok(resp.url)
    }

    /// Gets the url of an image / gif from the api, from the given
    /// category, and using the default client.
    /// # Examples
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url: String = nekoslife::get(nekoslife::SfwCategory::Waifu).await?;
    /// #   Ok(())
    /// # }
    pub async fn get(
        category: impl Into<Category>,
    ) -> Result<String, NekosLifeError> {
        let client = reqwest::Client::new();

        get_with_client(&client, category).await
    }
}

pub use implementation::{get, get_with_client};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dose_url_parsing_work() {
        assert_eq!(BASEURL.to_string(), "https://nekos.life/api/v2/");
        assert_eq!(
            BASEURL
                .join("endpoints")
                .expect("url parsing failed")
                .to_string(),
            "https://nekos.life/api/v2/endpoints"
        );
        assert_eq!(
            BASEURL
                .join("img/")
                .expect("url parsing failed")
                .join("category")
                .expect("url parsing failed")
                .to_string(),
            "https://nekos.life/api/v2/img/category"
        );
    }

    #[tokio::test]
    async fn all_endpoints_work() {
        use strum::IntoEnumIterator;

        let client = reqwest::Client::new();

        for variant in SfwCategory::iter()
            .map(Category::from)
            .chain(NsfwCategory::iter().map(Category::from))
        {
            get_with_client(&client, variant).await.unwrap_or_else(|_| {
                panic!("{} does not work", variant.to_url_path())
            });
            println!("{}: works", variant.to_url_path());
        }
    }

    #[tokio::test]
    async fn no_new_endpoints() {
        use {
            regex::Regex, std::collections::HashSet, strum::IntoEnumIterator,
        };

        let regex_img =
            Regex::new(r"^GET,HEAD,OPTIONS\s+/api/v2/img/<(?P<eps>.*)>$")
                .expect("failed to init regex");

        assert_eq!(
            SfwCategory::iter()
                .map(Into::into)
                .chain(NsfwCategory::iter().map(Into::into))
                .chain(["v3", "nekoapi_v3.1"])
                .collect::<HashSet<_>>(),
            Regex::new(r"'(?P<ct>[\w\.]+)'")
                .expect("failed to init regex")
                .captures_iter(
                    reqwest::Client::new()
                        .get(
                            BASEURL.join("endpoints").expect(
                                "an error occurred while joining the url"
                            )
                        )
                        .send()
                        .await
                        .expect("failed to send request")
                        .json::<Vec<String>>()
                        .await
                        .expect("failed to parse response")
                        .iter()
                        .find_map(|line| regex_img.captures(line))
                        .expect("no match found")
                        .name("eps")
                        .expect("couldn't find capture named eps")
                        .as_str()
                )
                .map(|cap| cap
                    .name("ct")
                    .expect("couldn't find capture named ct")
                    .as_str())
                .collect::<HashSet<_>>()
        );
    }
}
