use {crate::r#static::BASEURL, serde::Deserialize};

struct Cat;

#[derive(Deserialize)]
pub struct CatModel {
    cat: crate::UrlString,
}

impl super::types::IntoUrl for Cat {
    type Response = crate::types::UrlString;

    type Fut = into_url_fut! {};

    fn into_url(self) -> crate::types::Result<url::Url> {
        Ok(BASEURL.join("cat")?)
    }

    parse_json! {
        CatModel,
        cat,
    }
}

struct OwOify<'a>(pub &'a str);

#[derive(Deserialize)]
pub struct OwOifyModel {
    owo: crate::UrlString,
}

impl<'a> super::types::IntoUrl for OwOify<'a> {
    type Response = crate::types::UrlString;

    type Fut = into_url_fut! {};

    fn into_url(self) -> crate::types::Result<url::Url> {
        Ok({
            let mut url = BASEURL.join("owoify")?;

            url.query_pairs_mut()
                .append_pair(
                    "text", 
                    if matches! {
                        self.0.len(),
                        1..=200
                    } { self.0 } else {
                        panic!("OwOify text must be between 1 and 200 characters")
                    }
                );
            url
        })
    }

    parse_json! {
        OwOifyModel,
        owo
    }
}

#[cfg(test)]
mod tests;
