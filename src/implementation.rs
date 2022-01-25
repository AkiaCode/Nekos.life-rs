use {
    crate::{r#static::BASEURL, Category, Response},
    reqwest::{self, Client},
};

// use super::*;

/// Gets the url of an image / gif from the api, from the given category,
/// and using the given client.
/// # Examples
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = reqwest::Client::new();
///     let url: String = nekoslife::get_with_client(&client, nekoslife::Category::Waifu).await?;
/// #   Ok(())
/// # }
pub async fn get_with_client(
    client: &reqwest::Client,
    category: impl Into<Category>,
) -> Response {
    let category = category.into();

    #[derive(serde::Deserialize)]
    struct Response {
        url: String,
    }

    let resp = client
        .get(
            BASEURL
                .join("img/")?
                .join(category.to_url_path())?,
        )
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(resp.url)
}

/// Gets the url of an image / gif from the api, from the given
/// category, and using the default client.
/// # Examples
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let url: String = nekoslife::get(nekoslife::Category::Waifu).await?;
/// #   Ok(())
/// # }
pub async fn get(
    category: impl Into<Category>,
) -> Response {
    let client = Client::new();

    get_with_client(&client, category).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[tokio::test]
    async fn all_endpoints_work() {
        let client = Client::new();

        for variant in Category::iter() {
            get_with_client(&client, variant)
                .await
                .unwrap_or_else(|_| {
                    panic!(
                        "{} does not work",
                        variant.to_url_path()
                    )
                });
            println!("{}: works", variant.to_url_path());
        }
    }

    #[tokio::test]
    async fn no_new_endpoints() {
        use {
            regex::Regex, std::collections::HashSet,
            strum::IntoEnumIterator,
        };

        let regex_img =
            Regex::new(r"^GET,HEAD,OPTIONS\s+/api/v2/img/<(?P<eps>.*)>$")
                .expect("failed to init regex");

        assert_eq!(
            Category::iter()
                .map(|c| c.to_url_path())
                .chain(["v3", "nekoapi_v3.1"])
                .collect::<HashSet<_>>(),
            Regex::new(r"'(?P<ct>[\w\.]+)'")
                .expect("failed to init regex")
                .captures_iter(
                    Client::new()
                        .get(
                            BASEURL.join("endpoints").expect(
                                "an error occurred while joining the url"
                            )
                        )
                        .send()
                        .await
                        .expect("failed to send request")
                        .json::<Vec<String>>()
                        .await
                        .expect("failed to parse response")
                        .iter()
                        .find_map(|line| regex_img.captures(line))
                        .expect("no match found")
                        .name("eps")
                        .expect("couldn't find capture named eps")
                        .as_str()
                )
                .map(|cap| cap
                    .name("ct")
                    .expect("couldn't find capture named ct")
                    .as_str())
                .collect::<HashSet<_>>()
        );
    }
}
