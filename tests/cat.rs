use nekoslife::UnitResult;

use {nekoslife, pretty_assertions::assert_ne};

#[tokio::test]
async fn cat() -> UnitResult {
    Ok(assert_ne!(
        nekoslife::get(nekoslife::Cat)
            .await?
            .len(),
        0
    ))
}
