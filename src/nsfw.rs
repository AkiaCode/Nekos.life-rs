#![allow(deprecated)]

/// A nsfw category of images.
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
)]
#[cfg_attr(test, derive(strum::EnumIter))]
#[strum(serialize_all = "lowercase")]
pub enum NsfwCategory {
    #[strum(serialize = "Random_hentai_gif")]
    RandomHentaiGif,
    Pussy,
    #[strum(serialize = "nsfw_neko_gif")]
    NekoGif,
    #[strum(serialize = "lewd")]
    Neko,
    #[strum(serialize = "les")]
    Lesbian,
    Kuni,
    #[strum(serialize = "cum")]
    Cumsluts,
    Classic,
    Boobs,
    Bj,
    Anal,
    #[strum(serialize = "nsfw_avatar")]
    Avatar,
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
    Kemonomimi,
    #[strum(serialize = "lewdk")]
    Kitsune,
    Keta,
    #[strum(serialize = "hololewd")]
    Holo,
    HoloEro,
    Hentai,
    Futanari,
    Femdom,
    #[strum(serialize = "feetg")]
    FeetGif,
    EroFeet,
    Feet,
    Ero,
    #[strum(serialize = "erok")]
    EroKitsune,
    #[strum(serialize = "erokemo")]
    EroKemonomimi,
    #[strum(serialize = "eron")]
    EroNeko,
    EroYuri,
    #[strum(serialize = "cum_jpg")]
    CumArts,
    BlowJob,
    Spank,
    Gasm,
    #[deprecated(
        note = "always returns https://cdn.nekos.life/smallboobs/404.png; this is only here for completeness"
    )]
    SmallBoobs,
}

impl NsfwCategory {
    pub const fn to_url_path(self) -> &'static str {
        use NsfwCategory::*;
        match self {
            RandomHentaiGif => "Random_hentai_gif",
            Pussy => "pussy",
            NekoGif => "nsfw_neko_gif",
            Neko => "lewd",
            Lesbian => "les",
            Kuni => "kuni",
            Cumsluts => "cum",
            Classic => "classic",
            Boobs => "boobs",
            Bj => "bj",
            Anal => "anal",
            Avatar => "nsfw_avatar",
            Yuri => "yuri",
            Trap => "trap",
            Tits => "tits",
            GirlSoloGif => "solog",
            GirlSolo => "solo",
            PussyWankGif => "pwankg",
            PussyArt => "pussy_jpg",
            Kemonomimi => "lewdkemo",
            Kitsune => "lewdk",
            Keta => "keta",
            Holo => "hololewd",
            HoloEro => "holoero",
            Hentai => "hentai",
            Futanari => "futanari",
            Femdom => "femdom",
            FeetGif => "feetg",
            EroFeet => "erofeet",
            Feet => "feet",
            Ero => "ero",
            EroKitsune => "erok",
            EroKemonomimi => "erokemo",
            EroNeko => "eron",
            EroYuri => "eroyuri",
            CumArts => "cum_jpg",
            BlowJob => "blowjob",
            Spank => "spank",
            Gasm => "gasm",
            #[allow(deprecated)]
            SmallBoobs => "smallboobs",
        }
    }
}

#[test]
fn can_be_displayed_as_expected() {
    use strum::IntoEnumIterator;

    NsfwCategory::iter().for_each(|variant| {
        assert_eq!(Into::<&'static str>::into(&variant), variant.to_url_path())
    })
}
