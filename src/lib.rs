#[derive(thiserror::Error, Debug)]
pub enum NekosLifeError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
}

const BASEURL: &str = "https://nekos.life/api/v2";

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
    #[cfg(feature = "nsfw")]
    Nsfw(nsfw::NsfwCategory),
    #[cfg(feature = "sfw")]
    Sfw(sfw::SfwCategory),
}

impl Category {
    fn to_url_path(self) -> &'static str {
        match self {
            #[cfg(feature = "nsfw")]
            Self::Nsfw(c) => c.to_url_path(),
            #[cfg(feature = "sfw")]
            Self::Sfw(c) => c.to_url_path(),
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
        .get(format!("{}{}", BASEURL, category.to_url_path()))
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(resp.url)
}

pub async fn get(category: impl Into<Category>) -> Result<String, NekosLifeError> {
    let client = reqwest::Client::new();

    get_with_client(&client, category).await
}
