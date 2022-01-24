use {lazy_static::lazy_static, url::Url};

// private
const CUSTOM_BASEURL_ENV_VAR: &str = "NEKOS_LIFE_API_URL";
const DEFAULT_BASEURL: &str = "https://nekos.life/api/v2/";

lazy_static! {
    /// The base api url.
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
    fn will_crash_when_malformed_url_has_been_given() {
        env::set_var(
            CUSTOM_BASEURL_ENV_VAR,
            "c3mpuo#@%123",
        );

        // must panic
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
