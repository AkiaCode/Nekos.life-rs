use super::*;

/// Gets the url of an image / gif from the api, from the given category,
/// and using the given client.
/// # Examples
/// ```rust,no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = reqwest::blocking::Client::new();
///     let url: String = nekoslife::get_with_client(&client, nekoslife::SfwCategory::Waifu)?;
/// #   Ok(())
/// # }
pub fn get_with_client(
    client: &reqwest::blocking::Client,
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
        .send()?
        .json::<Response>()?;

    Ok(resp.url)
}

/// Gets the url of an image / gif from the api, from the given category,
/// and using the default client.
/// # Examples
/// ```rust,no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let url: String = nekoslife::get(nekoslife::SfwCategory::Waifu)?;
/// #   Ok(())
/// # }
pub fn get(category: impl Into<Category>) -> Response {
    let client = reqwest::blocking::Client::new();

    get_with_client(&client, category)
}
