use {
    super::*,
    crate::{Category, UnitResult},
};

#[test]
fn can_get_with_client_in_blocking_context() -> UnitResult {
    let _ =
        get_with_client(&Client::new(), Category::Neko)?;
    Ok(())
}
