use super::*;

#[test]
fn can_get_with_client_in_blocking_context(
) -> Result<(), NekosLifeError> {
    let _ =
        get_with_client(&Client::new(), Category::Neko)?;
    Ok(())
}
