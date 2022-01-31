use {
    super::*,
    crate::{get, get_with_client, UnitResult},
    pretty_assertions::assert_eq,
    reqwest::Client,
};

#[tokio::test]
async fn get_with_client_and_cat_test() -> UnitResult {
    dbg!(get_with_client(&Client::new(), Cat).await?);

    Ok(())
}

#[tokio::test]
async fn get_with_client_and_owoify_test() -> UnitResult {
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
async fn why_test() -> UnitResult {
    get(Why).await?;

    Ok(())
}

#[tokio::test]
async fn fact_test() -> UnitResult {
    get(Fact).await?;

    Ok(())
}

#[tokio::test]
async fn spoiler_test() -> UnitResult {
    Ok(assert_eq!(
        get(Spoiler("Abiria is cute :)"))
            .await?,
        "||A||||b||||i||||r||||i||||a|||| ||||i||||s|||| ||||:||||)||"
    ))
}

#[tokio::test]
async fn name_test() -> UnitResult {
    get(Name).await?;

    Ok(())
}
