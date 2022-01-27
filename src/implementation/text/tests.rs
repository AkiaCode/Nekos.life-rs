use {
    super::*,
    crate::{get, get_with_client, NekosLifeError},
    pretty_assertions::assert_eq,
    // lazy_regex::{lazy_regex, Lazy, Regex},
    reqwest::Client,
};

// static RESULT_CAT: Lazy<Regex> = lazy_regex!("qwer");

const EXACTLY_200_CHAR: &str =
    const_format::str_repeat! {"a", 200};

#[tokio::test]
async fn get_with_client_and_cat_test(
) -> Result<(), NekosLifeError> {
    dbg!(get_with_client(&Client::new(), Cat).await?);

    Ok(())
}

#[tokio::test]
async fn get_with_client_and_owoify_test(
) -> Result<(), NekosLifeError> {
    Ok(assert_eq!(
        get_with_client(
            &Client::new(),
            OwOify("Abiria is cute and kawaii :)")
        )
        .await?,
        "Abiwia is cute anyd kawaii :)"
    ))
}

#[tokio::test]
async fn owoify_with_exactly_200_chars(
) -> Result<(), NekosLifeError> {
    assert_eq!(EXACTLY_200_CHAR.len(), 200);

    assert_eq!(
        get(OwOify(&EXACTLY_200_CHAR)).await?,
        EXACTLY_200_CHAR
    );

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn owoify_with_more_than_200_chars() {
    assert!(get(OwOify(const_format::concatcp! {
       EXACTLY_200_CHAR,
       "a"
    }))
    .await
    .is_err());
}

#[tokio::test]
#[should_panic]
async fn owoify_with_0_chars() {
    assert!(
        get_with_client(&Client::new(), OwOify(""))
            .await
            .is_err()
    );
}
