use super::*;
use {std::error, strum::IntoEnumIterator};

#[tokio::test]
async fn all_endpoints_work() {
    let client = reqwest::Client::new();

    for variant in Category::iter() {
        get_with_client(&client, variant)
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "{} does not work",
                    variant.to_url_path()
                )
            });
        println!("{}: works", variant.to_url_path());
    }
}

#[tokio::test]
async fn no_new_endpoints(
) -> Result<(), Box<dyn error::Error>> {
    use {
        regex::Regex, std::collections::HashSet,
        strum::IntoEnumIterator,
    };

    let regex_img = Regex::new(
        r"^GET,HEAD,OPTIONS\s+/api/v2/img/<(?P<eps>.*)>$",
    )?;

    Ok(assert_eq!(
        Category::iter()
            .map(|c| c.to_url_path())
            .chain(["v3", "nekoapi_v3.1"])
            .collect::<HashSet<_>>(),
        Regex::new(r"'(?P<ct>[\w\.]+)'")
            .expect("failed to init regex")
            .captures_iter(
                reqwest::Client::new()
                    .get(BASEURL.join("endpoints")?)
                    .send()
                    .await?
                    .json::<Vec<String>>()
                    .await?
                    .iter()
                    .find_map(
                        |line| regex_img.captures(line)
                    )
                    .ok_or(NekosLifeError::new_unittest_error(
                        "couldn't find endpoints line"
                    ))?
                    .name("eps")
                    .ok_or(NekosLifeError::new_unittest_error(
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
