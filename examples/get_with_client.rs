use nekoslife::{
    get_with_client, Category, CategoryIter, UnitResult,
};

#[tokio::main]
async fn main() -> UnitResult {
    // declare the client we use
    let client = reqwest::Client::new();

    // and repeat the request 5 times.
    for category in Category::iter().take(5) {
        // get the url with async context. ('await')
        // note that we pass the reference of the client
        // as the first argument at here.
        let url =
            get_with_client(&client, category).await?;

        // then do something with the each result.
        println!("{url}");
    }

    Ok(())
}
