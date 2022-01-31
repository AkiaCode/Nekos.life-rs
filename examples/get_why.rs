use nekoslife::UnitResult;

#[tokio::main]
async fn main() -> UnitResult {
    // get some interesting question.
    let why = nekoslife::get(nekoslife::Why).await?;

    // then do something with the result.
    println!("{why}");
    // this shows some interesting question,
    // such like "why are there seashells on mt everest?"

    Ok(())
}
