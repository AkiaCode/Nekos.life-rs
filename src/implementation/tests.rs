use super::*;
use {std::error, strum::IntoEnumIterator};

#[tokio::test]
#[ignore]
async fn all_endpoints_work() {
    let client = Client::new();

    for variant in Category::iter() {
        #[rustfmt::skip]
            get_with_client(&client, variant)
                .await
                .unwrap_or_else(|_| panic!("{variant} does not work"));
        println!("{variant}: works");
    }
}

#[tokio::test]
async fn no_new_endpoints(
) -> Result<(), Box<dyn error::Error>> {
    use {
        crate::types::UnitTestError, regex::Regex,
        std::collections::HashSet, strum::IntoEnumIterator,
    };

    let regex_img = Regex::new(
        r"^GET,HEAD,OPTIONS\s+/api/v2/img/<(?P<eps>.*)>$",
    )?;

    Ok(assert_eq!(
        Category::iter()
            .map(Into::<&'static str>::into)
            .chain(["v3", "nekoapi_v3.1"])
            .collect::<HashSet<_>>(),
        Regex::new(r"'(?P<ct>[\w\.]+)'")
            .expect("failed to init regex")
            .captures_iter(
                Client::new()
                    .get(BASEURL.join("endpoints")?)
                    .send()
                    .await?
                    .json::<Vec<String>>()
                    .await?
                    .iter()
                    .find_map(
                        |line| regex_img.captures(line)
                    )
                    .ok_or(UnitTestError::new(
                        "couldn't find endpoints line"
                    ))?
                    .name("eps")
                    .ok_or(UnitTestError::new(
                        "couldn't find eps from capture"
                    ))?
                    .as_str()
            )
            .map(|cap| cap
                .name("ct")
                .expect("couldn't find capture named ct")
                .as_str())
            .collect::<HashSet<_>>()
    ))
}
