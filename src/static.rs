//! private module that contains the [`struct@BASEURL`] for the nekos.life API.
//!
//! # Important:
//!
//! this is private module.\
//! so this doc will only appears to developers of this library.\
//! rendered version at docs.rs may not contain this document.
//!
//! # How to update API url
//!
//! the endpoint of the API has hardcoded as v2.\
//! but if you want to use v3 or more recent version of API,\
//! you can override the url by setting
//! the environment variable `NEKOS_LIFE_API_URL`.
//!
//! ```sh
//! $ set NEKOS_LIFE_API_URL="https://example.com/api/v3/" && cargo run --example waifu
//! ```

use {lazy_static::lazy_static, url::Url};

/// enviroment variable name to override default API url.
///
/// this library will try to get `custom baseurl`
/// before initialize the [struct@BASEURL]
/// from enviroment variable with this name
const CUSTOM_BASEURL_ENV_VAR: &str = "NEKOS_LIFE_API_URL";

/// the default nekos.life API base url.
///
/// it is hardcoded to use version 2 API.\
/// but can be overrided by enviroment variable
/// named `NEKOS_LIFE_API_URL`.
const DEFAULT_BASEURL: &str = "https://nekos.life/api/v2/";

lazy_static! {
    /// the base endpoint of the nekos.life API.
    ///
    /// this is [`url::Url`] object.\
    /// it can't be initialized in `const` or `static`,
    /// so we need to use `lazy_static` to do it.\
    /// and it will be initialized only once.\
    /// also this is private,
    /// and user must not access to this directly.\
    /// the only way to change the `BASEURL`
    /// is to set enviroment variable named `NEKOS_LIFE_API_URL`.
    pub(crate) static ref BASEURL: Url =
        Url::parse(
            &std::env::var(CUSTOM_BASEURL_ENV_VAR)
                .unwrap_or(DEFAULT_BASEURL.to_owned())
        )
            .expect("Invalid base url");
}

#[cfg(test)]
mod test {
    use {super::*, const_format::formatcp, std::env};

    // for test, it will force to use default base url
    fn set_to_default_baseurl() {
        env::set_var(
            CUSTOM_BASEURL_ENV_VAR,
            DEFAULT_BASEURL,
        );
    }

    #[test]
    fn url_to_string() {
        set_to_default_baseurl();

        assert_eq!(BASEURL.as_str(), DEFAULT_BASEURL);
    }

    #[test]
    #[ignore]
    // this will change the enviroment variable,
    // but the [BASEURL] will already be initialized.
    // (lazy_static! macro will initialize variable only once.)
    // so runninng this test along with others may fail.
    fn can_set_custom_baseurl() {
        env::set_var(
            CUSTOM_BASEURL_ENV_VAR,
            "https://example.com/some/path/",
        );

        assert_eq!(
            BASEURL.host(),
            Some(url::Host::Domain("example.com"))
        );

        assert_eq!(BASEURL.path(), "/some/path/");
    }

    #[test]
    #[ignore]
    #[should_panic(
        expected = "Invalid base url: RelativeUrlWithoutBase"
    )]
    // ignored because of the same reason as `can_set_custom_baseurl` test.
    fn will_crash_when_malformed_url_has_been_given() {
        env::set_var(
            CUSTOM_BASEURL_ENV_VAR,
            "c3mpuo#@%123",
        );

        // must panic at here.
        let _ = BASEURL.host();
    }

    #[test]
    fn url_join() -> Result<(), url::ParseError> {
        set_to_default_baseurl();

        assert_eq!(
            BASEURL.join("endpoints")?.as_str(),
            formatcp!("{DEFAULT_BASEURL}endpoints") // "https://nekos.life/api/v2/endpoints"
        );

        assert_eq!(
            BASEURL
                .join("img/")?
                .join("category")?
                .as_str(),
            formatcp!("{DEFAULT_BASEURL}img/category") // "https://nekos.life/api/v2/img/category"
        );

        Ok(())
    }
}
