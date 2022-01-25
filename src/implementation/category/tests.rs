use {super::*, std::str, strum::IntoEnumIterator};

#[test]
fn can_be_displayed() {
    // ensures that the `strum::Display` was derived correctly.
    Category::iter().for_each(|variant| {
        format!("{variant}");
    });
}

#[test]
fn can_be_stringified() {
    // ensures that the `strum::IntoStaticStr` was derived correctly.
    Category::iter().for_each(|variant| {
        let _ = Into::<&'static str>::into(&variant);
    })
}

#[test]
fn two_methods_result_in_same_string() {
    // ensures that
    // the `<Category as std::string::ToString>::to_string()` method
    // and the `<Category as Into<&'static str>>::into()` method
    // result in the same string.
    assert!(Category::iter()
        .map(|variant| variant.to_string())
        .eq(Category::iter()
            .map(Into::<&'static str>::into)));
}

#[test]
fn can_be_parsed_from_string(
) -> Result<(), strum::ParseError> {
    use Category::*;

    // ensures that the `<Category as std::str::FromStr>::from_str()` method
    // can be used to parse a string into a `Category`.
    Ok(assert!(["neko", "NeKO", "wAiFu"]
        .into_iter()
        .map(<Category as str::FromStr>::from_str)
        .eq([Neko, Neko, Waifu]
            .into_iter()
            .map(Result::Ok))))
}

#[test]
fn returns_error_when_invalid_category_has_been_given(
) -> Result<(), strum::ParseError> {
    // ensures that non-existing category string
    // has been given, returns Err(strum::ParseError).

    Ok(assert!(["Abiria", "is", "cute"]
        .into_iter()
        .map(<Category as str::FromStr>::from_str)
        .all(|e| e
            .eq(&Err(strum::ParseError::VariantNotFound)))))
}
