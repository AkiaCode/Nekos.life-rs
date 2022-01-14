use strum;

/// A sfw category of images.
// On new variants, update the all_sfw_endpoints_work and no_new_images tests
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum::Display,
)]
#[cfg_attr(test, derive(strum::EnumIter))]
#[strum(serialize_all = "snake_case")]
pub enum SfwCategory {
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
}

impl SfwCategory {
    pub const fn to_url_path(self) -> &'static str {
        use SfwCategory::*;
        match self {
            Tickle => "tickle",
            Slap => "slap",
            Poke => "poke",
            Pat => "pat",
            Neko => "neko",
            Meow => "meow",
            Lizard => "lizard",
            Kiss => "kiss",
            Hug => "hug",
            FoxGirl => "fox_girl",
            Feed => "feed",
            Cuddle => "cuddle",
            NekoGif => "ngif",
            Kemonomimi => "kemonomimi",
            Holo => "holo",
            Smug => "smug",
            Baka => "baka",
            Woof => "woof",
            Wallpaper => "wallpaper",
            Goose => "goose",
            Gecg => "gecg",
            Avatar => "avatar",
            Waifu => "waifu",
            EightBall => "8ball",
        }
    }
}

#[test]
fn can_be_displayed_as_expected() {
    use strum::IntoEnumIterator;

    SfwCategory::iter().for_each(|variant| {
        assert_eq!(variant.to_string(), variant.to_url_path())
    })
}
