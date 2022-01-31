use nekoslife::UnitResult;

#[tokio::main]
async fn main() -> UnitResult {
    // get some interesting fact.
    let fact = nekoslife::get(nekoslife::Fact).await?;

    // then do something with the result.
    println!("{fact}");
    // this shows some fact, such like "There are over 500 different types of bananas"

    Ok(())
}
