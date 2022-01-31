use nekoslife::UnitResult;

#[tokio::main]
async fn main() -> UnitResult {
    // get owoified version of "hello, world"
    let owo =
        nekoslife::get(nekoslife::OwOify("hello, world"))
            .await?;

    // this will be converted version of our text.
    assert_eq!(owo, "hewwo, wowwd");

    Ok(())
}
