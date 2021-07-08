/// Any error that can occur while accessing the api.
#[derive(thiserror::Error, Debug)]
pub enum NekosLifeError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
}

/// The base api url.
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
    /// # use nekoslife_rs::{Category, SfwCategory};
    /// assert_eq!(Category::from(SfwCategory::Waifu).to_url_path(), "waifu");
    /// ```
    pub const fn to_url_path(self) -> &'static str {
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

#[cfg(feature = "blocking")]
mod implementation {
    use super::*;

    /// Gets the url of an image / gif from the api, from the given category,
    /// and using the given client.
    /// # Examples
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = reqwest::blocking::Client::new();
    ///     let url: String = nekoslife_rs::get_with_client(&client, nekoslife_rs::SfwCategory::Waifu)?;
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
            .get(format!("{}/img/{}", BASEURL, category.to_url_path()))
            .send()?
            .json::<Response>()?;

        Ok(resp.url)
    }

    /// Gets the url of an image / gif from the api, from the given category,
    /// and using the default client.
    /// # Examples
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url: String = nekoslife_rs::get(nekoslife_rs::SfwCategory::Waifu)?;
    /// #   Ok(())
    /// # }
    pub fn get(category: impl Into<Category>) -> Result<String, NekosLifeError> {
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
    ///     let url: String = nekoslife_rs::get_with_client(&client, nekoslife_rs::SfwCategory::Waifu).await?;
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
            .get(format!("{}/img/{}", BASEURL, category.to_url_path()))
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
    ///     let url: String = nekoslife_rs::get(nekoslife_rs::SfwCategory::Waifu).await?;
    /// #   Ok(())
    /// # }
    pub async fn get(category: impl Into<Category>) -> Result<String, NekosLifeError> {
        let client = reqwest::Client::new();

        get_with_client(&client, category).await
    }
}

pub use implementation::{get, get_with_client};

#[cfg(test)]
mod test {
    use super::*;

    async fn try_endpoint(
        client: &reqwest::Client,
        category: impl Into<Category>,
    ) -> Result<(), (NekosLifeError, Category)> {
        let category = category.into();
        match get_with_client(client, category).await {
            Ok(_) => Ok(()),
            Err(e) => Err((e, category)),
        }
    }

    macro_rules! try_endpoints {
        ($client:expr, $try_endpoint_fn:ident, $sfw_nsfw:ident, [$($(#[$at:meta])* $category:ident),* $(,)?]) => {
            $(try_endpoints!($client, $try_endpoint_fn, $sfw_nsfw, $(#[$at])* $category);)*
        };

        ($client:expr, $try_endpoint_fn:ident, $sfw_nsfw:ident, $(#[$at:meta])* $category:ident) => {
            $try_endpoint_fn($client, $(#[$at])* {$sfw_nsfw::$category}).await.unwrap(); // test will fail if any of them error
        }
    }

    #[tokio::test]
    async fn all_sfw_endpoints_work() {
        let client = reqwest::Client::new();
        try_endpoints!(
            &client,
            try_endpoint,
            SfwCategory,
            [
                Tickle, Slap, Poke, Pat, Neko, Meow, Lizard, Kiss, Hug, FoxGirl, Feed, Cuddle,
                NekoGif, Kemonomimi, Holo, Smug, Baka, Woof, Wallpaper, Goose, Gecg, Avatar, Waifu,
                EightBall,
            ]
        );
    }

    #[tokio::test]
    async fn all_nsfw_endpoints_work() {
        let client = reqwest::Client::new();
        try_endpoints!(
            &client,
            try_endpoint,
            NsfwCategory,
            [
                RandomHentaiGif,
                Pussy,
                NekoGif,
                Neko,
                Lesbian,
                Kuni,
                Cumsluts,
                Classic,
                Boobs,
                Bj,
                Anal,
                Avatar,
                Yuri,
                Trap,
                Tits,
                GirlSoloGif,
                GirlSolo,
                PussyWankGif,
                PussyArt,
                Kemonomimi,
                Kitsune,
                Keta,
                Holo,
                HoloEro,
                Hentai,
                Futanari,
                Femdom,
                FeetGif,
                EroFeet,
                Feet,
                Ero,
                EroKitsune,
                EroKemonomimi,
                EroNeko,
                EroYuri,
                CumArts,
                BlowJob,
                Spank,
                Gasm,
                #[allow(deprecated)]
                SmallBoobs,
            ]
        );
    }

    #[tokio::test]
    async fn no_new_endpoints() {
        let client = reqwest::Client::new();

        macro_rules! known_image_endpoints {
            ([$($sfw_nsfw:ident : [$($(#[$at:meta])* $category:ident),* $(,)?]),* $(,)?]) => {
                [
                    $(
                        $($(#[$at])* {known_image_endpoints!($sfw_nsfw, $category)},)*
                    )*

                    // ignore those endpoints
                    "v3",
                    "nekoapi_v3.1"
                ]
            };

            ($sfw_nsfw:ident, $category:ident $(,)?) => {
                $sfw_nsfw::$category.to_url_path()
            };
        }

        const KNOWN_ENDPOINTS: &[&str] = &known_image_endpoints!(
            [
                SfwCategory: [
                    Tickle,
                    Slap,
                    Poke,
                    Pat,
                    Neko,
                    Meow,
                    Lizard,
                    Kiss,
                    Hug,
                    FoxGirl,
                    Feed,
                    Cuddle,
                    NekoGif,
                    Kemonomimi,
                    Holo,
                    Smug,
                    Baka,
                    Woof,
                    Wallpaper,
                    Goose,
                    Gecg,
                    Avatar,
                    Waifu,
                    EightBall,
                ],
                NsfwCategory: [
                    RandomHentaiGif,
                    Pussy,
                    NekoGif,
                    Neko,
                    Lesbian,
                    Kuni,
                    Cumsluts,
                    Classic,
                    Boobs,
                    Bj,
                    Anal,
                    Avatar,
                    Yuri,
                    Trap,
                    Tits,
                    GirlSoloGif,
                    GirlSolo,
                    PussyWankGif,
                    PussyArt,
                    Kemonomimi,
                    Kitsune,
                    Keta,
                    Holo,
                    HoloEro,
                    Hentai,
                    Futanari,
                    Femdom,
                    FeetGif,
                    EroFeet,
                    Feet,
                    Ero,
                    EroKitsune,
                    EroKemonomimi,
                    EroNeko,
                    EroYuri,
                    CumArts,
                    BlowJob,
                    Spank,
                    Gasm,
                    #[allow(deprecated)]
                    SmallBoobs,
                ],
            ]
        );

        async fn get_endpoints(client: &reqwest::Client) -> Vec<String> {
            client
                .get(format!("{}/endpoints", BASEURL))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap()
        }

        let endpoints = get_endpoints(&client).await;
        let image_endpoints = endpoints
            .iter()
            .find(|it| it.starts_with("GET,HEAD,OPTIONS     /api/v2/img/"))
            .unwrap()
            .as_str();
        let comma_list = image_endpoints
            .trim_start_matches("GET,HEAD,OPTIONS     /api/v2/img/<")
            .trim_end_matches(">");
        let list = comma_list
            .split(',')
            .map(|it| it.trim().trim_start_matches('\'').trim_end_matches('\''))
            .collect::<Vec<_>>();

        let mut unknown_endpoints = vec![];
        for item in list.iter() {
            if !KNOWN_ENDPOINTS.contains(item) {
                unknown_endpoints.push(format!("{}/img/{}", BASEURL, item.to_string()));
            }
        }

        if !unknown_endpoints.is_empty() {
            panic!(
                "Looks like there are new endpoints, please add them: {:?}",
                unknown_endpoints
            );
        }
    }
}
