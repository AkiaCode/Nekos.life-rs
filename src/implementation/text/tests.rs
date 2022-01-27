use {
    super::*,
    crate::{get_with_client, NekosLifeError},
    // lazy_regex::{lazy_regex, Lazy, Regex},
    reqwest::Client,
};

// static RESULT_CAT: Lazy<Regex> = lazy_regex!("qwer");

#[tokio::test]
async fn get_with_client_and_cat_test(
) -> Result<(), NekosLifeError> {
    dbg!(get_with_client(&Client::new(), Text::Cat).await?);

    Ok(())
}
