use {crate::r#static::BASEURL, serde::Deserialize};

#[derive(Debug, strum::IntoStaticStr, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Text {
    Cat,
}

#[derive(Deserialize)]
pub struct Cat {
    cat: crate::UrlString,
}

impl super::types::IntoUrl for Text {
    type Response = crate::types::UrlString;

    type Fut = into_url_fut! {};

    fn into_url(self) -> crate::types::Result<url::Url> {
        Ok(BASEURL
            .join(Into::<&'static str>::into(&self))?)
    }

    parse_json! {
        Cat,
        cat,
    }
}

#[cfg(test)]
mod tests;
