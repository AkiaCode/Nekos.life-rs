/// A nsfw category of images.
// On new variants, update the all_nsfw_endpoints_work and no_new_images tests
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum NsfwCategory {
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
    #[deprecated(
        note = "always returns https://cdn.nekos.life/smallboobs/404.png; this is only here for completeness"
    )]
    SmallBoobs,
}

impl NsfwCategory {
    /// converts [NsfwCategory] variants to `&'static str`
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
