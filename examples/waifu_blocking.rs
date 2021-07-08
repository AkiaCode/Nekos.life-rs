fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String = nekoslife_rs::get(nekoslife_rs::SfwCategory::Waifu)?;

    println!("{}", url);

    Ok(())
}
