use {lazy_static::lazy_static, url::Url};

lazy_static! {
    /// The base api url.
    pub(crate) static ref BASEURL: Url =
        Url::parse("https://nekos.life/api/v2/")
            .expect("Invalid base url");
}

#[cfg(test)]
mod test {
    #[test]
    fn dose_url_parsing_work() {
        assert_eq!(
            crate::r#static::BASEURL.to_string(),
            "https://nekos.life/api/v2/"
        );
        assert_eq!(
            crate::r#static::BASEURL
                .join("endpoints")
                .expect("url parsing failed")
                .to_string(),
            "https://nekos.life/api/v2/endpoints"
        );
        assert_eq!(
            crate::r#static::BASEURL
                .join("img/")
                .expect("url parsing failed")
                .join("category")
                .expect("url parsing failed")
                .to_string(),
            "https://nekos.life/api/v2/img/category"
        );
    }
}
