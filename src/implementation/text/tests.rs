use const_format::{concatcp, str_repeat};

use {
    super::*,
    crate::{get, get_with_client, NekosLifeError},
    pretty_assertions::assert_eq,
    // lazy_regex::{lazy_regex, Lazy, Regex},
    reqwest::Client,
};

// static RESULT_CAT: Lazy<Regex> = lazy_regex!("qwer");

const EXACTLY_200_CHARS: &str = str_repeat! {"a", 200};

const EXACTLY_1000_CHARS: &str = str_repeat! {"a", 1000};

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
#[ignore]
async fn owoify_with_exactly_200_chars(
) -> Result<(), NekosLifeError> {
    assert_eq!(EXACTLY_200_CHARS.len(), 200);

    assert_eq!(
        get(OwOify(&EXACTLY_200_CHARS)).await?,
        EXACTLY_200_CHARS
    );

    Ok(())
}

#[tokio::test]
async fn owoify_with_more_than_200_chars() {
    assert!(get(OwOify(concatcp! {
       EXACTLY_200_CHARS,
       "a"
    }))
    .await
    .is_err());
}

#[tokio::test]
async fn owoify_with_0_chars() {
    assert!(get(OwOify("")).await.is_err());
}

#[tokio::test]
async fn why_test() -> Result<(), NekosLifeError> {
    get(Why).await?;

    Ok(())
}

#[tokio::test]
async fn fact_test() -> Result<(), NekosLifeError> {
    get(Fact).await?;

    Ok(())
}

#[tokio::test]
async fn spoiler_test() -> Result<(), NekosLifeError> {
    Ok(assert_eq!(
        get(Spoiler("Abiria is cute and kawaii :)"))
            .await?,
        "||A||||b||||i||||r||||i||||a|||| ||||i||||s|||| ||||c||||u||||t||||e|||| ||||a||||n||||d|||| ||||k||||a||||w||||a||||i||||i|||| ||||:||||)||"
    ))
}

#[tokio::test]
#[ignore]
async fn spoiler_with_exactly_200_chars(
) -> Result<(), NekosLifeError> {
    assert_eq!(
        get(Spoiler(&EXACTLY_1000_CHARS)).await?,
        str_repeat! {"||a||", 1000}
    );

    Ok(())
}

#[tokio::test]
async fn spoiler_with_more_than_200_chars() {
    assert!(get(Spoiler(concatcp! {
        EXACTLY_1000_CHARS,
       "a"
    }))
    .await
    .is_err());
}

#[tokio::test]
async fn spoiler_with_0_chars() {
    assert!(get(Spoiler("")).await.is_err());
}

#[tokio::test]
async fn name_test() -> Result<(), NekosLifeError> {
    get(Name).await?;

    Ok(())
}
