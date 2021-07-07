#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String = nekoslife_rs::get(nekoslife_rs::SfwCategory::Waifu).await?;

    println!("{}", url);

    Ok(())
}
