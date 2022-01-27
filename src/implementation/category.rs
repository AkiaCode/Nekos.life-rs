//! represents the image categories

#![allow(deprecated)]

use crate::{types, IntoUrl};

/// A category of images.
// On new variants, update the all_nsfw_endpoints_work and no_new_images tests
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::IntoStaticStr,
    strum::EnumIter,
    strum::Display,
    strum::EnumString,
)]
#[strum(
    serialize_all = "snake_case",
    ascii_case_insensitive
)]
#[allow(missing_docs)]
pub enum Category {
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
    #[strum(serialize = "ngif")]
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
    #[strum(serialize = "8ball")]
    EightBall,
    #[strum(serialize = "Random_hentai_gif")]
    RandomHentaiGif,
    Pussy,
    #[strum(serialize = "nsfw_neko_gif")]
    NsfwNekoGif,
    Lewd,
    #[strum(serialize = "les")]
    Lesbian,
    Kuni,
    #[strum(serialize = "cum")]
    Cumsluts,
    Classic,
    Boobs,
    Bj,
    Anal,
    NsfwAvatar,
    Yuri,
    Trap,
    Tits,
    #[strum(serialize = "solog")]
    GirlSoloGif,
    #[strum(serialize = "solo")]
    GirlSolo,
    #[strum(serialize = "pwankg")]
    PussyWankGif,
    #[strum(serialize = "pussy_jpg")]
    PussyArt,
    #[strum(serialize = "lewdkemo")]
    LewdKemonomimi,
    #[strum(serialize = "lewdk")]
    Kitsune,
    Keta,
    #[strum(serialize = "hololewd")]
    HoloLewd,
    #[strum(serialize = "holoero")]
    HoloEro,
    Hentai,
    Futanari,
    Femdom,
    #[strum(serialize = "feetg")]
    FeetGif,
    #[strum(serialize = "erofeet")]
    EroFeet,
    Feet,
    Ero,
    #[strum(serialize = "erok")]
    EroKitsune,
    #[strum(serialize = "erokemo")]
    EroKemonomimi,
    #[strum(serialize = "eron")]
    EroNeko,
    #[strum(serialize = "eroyuri")]
    EroYuri,
    #[strum(serialize = "cum_jpg")]
    CumArts,
    #[strum(serialize = "blowjob")]
    BlowJob,
    Spank,
    Gasm,
    #[deprecated(
        note = "always returns https://cdn.nekos.life/smallboobs/404.png; this is only here for completeness"
    )]
    #[strum(serialize = "smallboobs")]
    SmallBoobs,
}

impl IntoUrl for Category {
    type Response = crate::types::UrlString;

    type Fut = std::pin::Pin<
        Box<
            dyn std::future::Future<
                Output = types::Result<Self::Response>,
            >,
        >,
    >;

    fn into_url(self) -> types::Result<url::Url> {
        Ok(string_to_endpoint!(self.into()))
    }

    fn parse(res: reqwest::Response) -> Self::Fut {
        Box::pin(async move {
            Ok(res.json::<super::types::ApiResponseBody>().await?.url)
        })
    }
}

#[cfg(test)]
mod tests;
