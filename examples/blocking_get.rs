use nekoslife::{blocking::get, Category, UnitResult};

fn main() -> UnitResult {
    // get the image url from 'Neko' category
    let url = get(Category::Neko)?;
    // in this case, the return type will be 'String'

    // then do something with the url.
    println!("{url}");

    Ok(())
}
