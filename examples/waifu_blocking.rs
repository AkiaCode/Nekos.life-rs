fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String = nekoslife::blocking_get(
        nekoslife::Category::Waifu,
    )?;

    println!("{url}");

    Ok(())
}
