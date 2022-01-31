use nekoslife::{Category, UnitResult};

// you must need an async context
// to '.await' Future.
#[tokio::main]
async fn main() -> UnitResult {
    // get the image url from 'Neko' category
    let url = nekoslife::get(Category::Neko).await?;
    // in this case, the return type will be 'String'

    // then do something with the url.
    println!("{url}");

    Ok(())
}
