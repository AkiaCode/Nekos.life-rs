fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String =
        nekoslife::get(nekoslife::SfwCategory::Waifu)?;

    println!("{url}");

    Ok(())
}
