use nekoslife::UnitResult;

#[tokio::main]
async fn main() -> UnitResult {
    // get some cute cat asciiart
    let cat = nekoslife::get(nekoslife::Cat).await?;

    // then do something with the result.
    println!("{cat}");
    // this shows cat text, such like "(｡･ω･｡)"

    Ok(())
}
