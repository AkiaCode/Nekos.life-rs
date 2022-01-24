use crate::{
    category::Category, error::NekosLifeError,
    implementation::get_with_client,
};

/// Gets the url of an image / gif from the api, from the given category,
/// and using the given client.
/// # Examples
/// ```rust,no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = reqwest::blocking::Client::new();
///     let url: String = nekoslife::get_with_client(&client, nekoslife::SfwCategory::Waifu)?;
/// #   Ok(())
/// # }
pub fn blocking_get_with_client(
    client: &reqwest::Client,
    category: impl Into<Category>,
) -> Result<String, NekosLifeError> {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()?
        .block_on(get_with_client(&client, category.into()))
}

/// Gets the url of an image / gif from the api, from the given category,
/// and using the default client.
/// # Examples
/// ```rust,no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let url: String = nekoslife::get(nekoslife::SfwCategory::Waifu)?;
/// #   Ok(())
/// # }
pub fn blocking_get(
    category: impl Into<Category>,
) -> Result<String, NekosLifeError> {
    let client = reqwest::Client::new();

    blocking_get_with_client(&client, category)
}
