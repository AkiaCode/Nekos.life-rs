use nekoslife::UnitResult;

#[tokio::main]
async fn main() -> UnitResult {
    // get some funny name.
    let name = nekoslife::get(nekoslife::Name).await?;

    // then do something with the result.
    println!("{name}");
    // this shows the name, such like "Fence Monster"

    Ok(())
}
