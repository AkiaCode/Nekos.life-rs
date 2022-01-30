use {super::*, crate::Category};

#[test]
fn can_get_with_client_in_blocking_context(
) -> crate::Result<()> {
    let _ =
        get_with_client(&Client::new(), Category::Neko)?;
    Ok(())
}
