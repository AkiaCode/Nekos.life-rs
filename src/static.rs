//! module that contains the [`struct@BASEURL`] and other statics for the nekos.life API.
//!
//! # How to update API url
//!
//! the endpoint of the API has hardcoded as v2[^url].\
//! but if you want to use v3 or more recent version of API,\
//! you can override the url by setting
//! the environment variable [`NEKOS_LIFE_API_URL`](CUSTOM_BASEURL_ENV_VAR).
//!
//! ```sh
//! $ set NEKOS_LIFE_API_URL="https://example.com/api/v3/" && cargo run --example waifu
//! ```
//!
//! currently, however, the `v3` api is not working (just gives only 404),
//! and `v1` is deprecated.\
//!
//! [^url]: `
#![doc = include_str!("../docs/apiv2.url")]
//! `

use {lazy_static::lazy_static, url::Url};

/// name of enviroment variable which can override [`default API url`](struct@BASEURL).
///
/// this library will try to get `custom baseurl`
/// before initialize the [struct@BASEURL]\
/// from enviroment variable with this name.
pub const CUSTOM_BASEURL_ENV_VAR: &str =
    "NEKOS_LIFE_API_URL";

// string version of BASEURL.
// not yet parsed by `url::Url::parse()`.
const DEFAULT_BASEURL: &str =
    include_str!("../docs/apiv2.url");

lazy_static! {
    /// the default endpoint of the nekos.life API.
    ///
    /// # Important
    ///
    /// ## Version
    ///
    /// it is hardcoded to use version 2 API.\
    /// but can be overrided by enviroment variable
    /// named `NEKOS_LIFE_API_URL`.
    ///
    /// read [module level document](self) for more information.
    ///
    /// ## Type
    ///
    /// this is [`Url`](url::Url) object, not [String] or [&str]\
    /// therefor, it can't be initialized in `const` or `static`,
    /// so it had to use `lazy_static` under the hood to do it.\
    /// and it will be initialized only once.\
    ///
    /// also this is struct,
    /// you might need to call [`.join()`](url::Url::join) method,\
    /// rather than `+` operator or [`format!`] macro,\
    /// to get extended url which based on this.
    ///
    /// ### Examples
    ///
    /// ```rust
    /// # use nekoslife::BASEURL;
    /// # use pretty_assertions::assert_eq;
    /// # assert_eq!(
    /// BASEURL.as_str() // "https://nekos.life/api/v2/"
    /// # , "https://nekos.life/api/v2/");
    ///
    /// # assert_eq!(
    /// BASEURL.join("cat")?.as_str() // "https://nekos.life/api/v2/cat"
    /// # , "https://nekos.life/api/v2/cat");
    ///
    /// # assert_eq!(
    /// BASEURL.join("img/neko")?.as_str() // "https://nekos.life/api/v2/img/neko"
    /// # , "https://nekos.life/api/v2/img/neko");
    ///
    /// # assert_eq!(
    /// // note that the "img/" has slash at the end.
    /// BASEURL.join("img/")?.join("neko")?.as_str() // "https://nekos.life/api/v2/img/neko"
    /// # , "https://nekos.life/api/v2/img/neko");
    ///
    /// # assert_eq!(
    /// // and without that slash...
    /// BASEURL.join("img")?.join("neko")?.as_str() // "https://nekos.life/api/v2/neko"
    /// // it will replace the "img" with "neko"
    /// // since it assumes this is not to be a directory path.
    /// # , "https://nekos.life/api/v2/neko");
    ///
    /// // any path which starts with "/" will be considered absolute path.
    /// # assert_eq!(
    /// BASEURL.join("/")?.as_str() // "https://nekos.life/"
    /// # , "https://nekos.life/");
    /// # assert_eq!(
    /// BASEURL.join("/api/v3")?.as_str() // "https://nekos.life/api/v3"
    /// # , "https://nekos.life/api/v3");
    ///
    /// let mut my_url = BASEURL.join("/my/endpoint")?;
    ///
    /// // set querty parameters
    /// my_url
    ///     .query_pairs_mut()
    ///     .append_pair("hello", "world");
    ///
    /// assert_eq!(my_url.as_str(), "https://nekos.life/my/endpoint?hello=world");
    /// # Ok::<(), url::ParseError>(())
    /// ```
    pub static ref BASEURL: Url =
        Url::parse(
            &std::env::var(CUSTOM_BASEURL_ENV_VAR)
                .unwrap_or_else(|_| DEFAULT_BASEURL.to_owned())
        )
            .expect("Invalid base url");
}

#[cfg(test)]
mod tests;
