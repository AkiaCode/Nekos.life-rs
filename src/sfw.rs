/// A sfw category of images.
// On new variants, update the all_sfw_endpoints_work and no_new_images tests
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
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
}

impl SfwCategory {
    /// converts [SfwCategory] variants to `&'static str`
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
