use {
    super::*,
    crate::{get, UnitResult},
    const_format::{concatcp, str_repeat},
};

const EXACTLY_200_CHARS: &str = str_repeat! {"a", 200};

const EXACTLY_1000_CHARS: &str = str_repeat! {"a", 1000};

#[tokio::test]
#[ignore]
async fn owoify_with_exactly_200_chars() -> UnitResult {
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
#[ignore]
async fn spoiler_with_exactly_200_chars() -> UnitResult {
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
