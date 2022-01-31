// we need async context to use 'get' method.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 'get' method will return 'Future'.
    // so you have to use '.await' to get the result.
    // in this case, the return type
    // of the method is 'Result<String, Error>',
    // so we can use '?' operator here.
    let url: String =
        nekoslife::get(nekoslife::Category::Waifu).await?;

    // print out the recieved url
    println!("{url}");

    Ok(())
}
