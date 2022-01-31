//! module that contains the [`Category`] enum.
#![allow(deprecated)]

/// Image [categories](self::Category).
///
/// # About
///
/// this is most general type that you may be going to use frequently.
///
/// you can use this to get images from certain categories,\
/// for example, assuming you want to get some image in the `neko` category,
/// you can accomplish it by:
///
/// ```rust
/// nekoslife::get(nekoslife::Category::Neko);
/// ```
///
/// this will give you a [`UrlString`](crate::UrlString)
/// which tells you the url of the image.
///
/// there are many kinds of categories,
/// if you are finding some image to use as avatar,\
/// try it:
///
/// ```rust
/// nekoslife::get(nekoslife::Category::Avatar);
/// ```
///
/// this will give you some corner-truncated image
/// that fits the shape of profile images.
///
/// if you want more information about each categories,
/// please visit the [nekos.life](https://nekos.life/) website
/// and thier [github](https://github.com/Nekos-life).
///
/// ## Compatibility
///
/// also this implementation were trying to reflect the API of
/// [original js implementation](https://github.com/Nekos-life/nekos-dot-life) as much as possible,\
/// certain names and rules, however, was localized to fits for rust.
///
/// for example, the text endpoints were split into another [`module`](crate::text).\
/// and some conflicting names were renamed.
///
/// ## Types
///
/// you can pass strings instead of [Category], such like
///
/// ```rust
/// nekoslife::get("neko");
/// ```
///
/// instead of
///
/// ```compile_fail
/// nekoslife::get(nekoslife::Category::Neko); // Err(nekoslife::Error::UnkownEndpoint)
/// ```
///
/// this is useful for testing and development purposes.
///
/// but keep that in mind this does not garuntee
/// that the endpoint is correct,\
/// so tring below code will be failed.
///
/// ```rust
/// nekoslife::get("not existing endpoint"); // Err(nekoslife::Error::UnkownEndpoint)
/// ```
///
/// also string interpretation is only available in [Category].\
/// if you want to use another endpoints, such like [`Cat`](crate::Cat),
/// you must pass the correct struct:
///
/// ```rust
/// nekoslife::get(nekoslife::Cat);
/// ```
///
/// rather than
///
/// ```rust
/// nekoslife::get("cat"); // Err(nekoslife::Error::UnkownEndpoint)
/// ```
///
/// ## Iteration
///
/// [`Category`] is [non-exhaustive][non-exhaustive] enum,
/// which means this is not a complete list of all categories.\
/// some categories are deprecared or added later,
/// so you do not matching this enum directly.
///
/// instead, you can use [`iter`][iter]
/// to get an iterator of all categories.
///
/// ```rust,ignore
/// # #[tokio::main]
/// # async fn main() -> nekoslife::UnitResult {
/// // you can use for loop for iterator
/// for category in nekoslife::Category::iter() {
///     // get the result one by one.
///     let res = nekoslife::get(category).await?;
///
///     // do something with the result.
///     println!(
///         "image url from {category} categoty: {}",
///         res.url(),
///     );
/// }
/// # Ok(())
/// # }
/// ```
///
/// below is another example that uses [`choose`][choose] method.
///
/// ```rust
#[doc = include_str!("../../examples/pick_random.rs")]
/// ```
///
/// [non-exhaustive]: https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
/// [iter]: https://docs.rs/strum/latest/strum/trait.IntoEnumIterator.html#tymethod.iter
/// [choose]: https://rust-random.github.io/rand/rand/seq/trait.SliceRandom.html#tymethod.choose
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
#[non_exhaustive]
#[deny(deprecated)]
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

mod into_url;

#[cfg(test)]
mod tests;
