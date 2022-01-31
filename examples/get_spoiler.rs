use nekoslife::UnitResult;

#[tokio::main]
async fn main() -> UnitResult {
    // get the discord-like spoiler text of "Akiacode"
    let spoiler =
        nekoslife::get(nekoslife::Spoiler("Akiacode"))
            .await?;

    // this will be converted version of our text.
    assert_eq!(
        spoiler,
        "||A||||k||||i||||a||||c||||o||||d||||e||"
    );

    Ok(())
}
