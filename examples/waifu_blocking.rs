fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String =
        nekoslife::get(nekoslife::Category::Waifu)?;

    println!("{}", url);

    Ok(())
}
