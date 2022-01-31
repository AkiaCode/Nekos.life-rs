use {
    super::*, const_format::formatcp,
    pretty_assertions::assert_eq, std::env,
};

// for test, it will force to use default base url
fn set_to_default_baseurl() {
    env::set_var(CUSTOM_BASEURL_ENV_VAR, DEFAULT_BASEURL);
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
    env::set_var(CUSTOM_BASEURL_ENV_VAR, "c3mpuo#@%123");

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
