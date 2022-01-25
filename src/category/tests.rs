use {super::*, strum::IntoEnumIterator};

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
