use nekoslife::{Category, UnitResult};

#[tokio::main]
async fn main() -> UnitResult {
    // get the image url from 'Neko' category
    let url = nekoslife::get("neko").await?;
    // note that, the "neko" will be converted to Category::Neko.

    // then do something with the url.
    println!("{url}");

    Ok(())
}
