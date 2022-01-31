use nekoslife::{
    blocking::get_with_client, Category, CategoryIter,
    UnitResult,
};

fn main() -> UnitResult {
    // declare the client we use
    let client = reqwest::Client::new();

    // and repeat the request 5 times.
    for category in Category::iter().take(5) {
        // get the url with blocking context. (no 'await')
        // note that we pass the reference of the client
        // as the first argument at here.
        let url = get_with_client(&client, category)?;

        // then do something with the each result.
        println!("{url}");
    }

    Ok(())
}
