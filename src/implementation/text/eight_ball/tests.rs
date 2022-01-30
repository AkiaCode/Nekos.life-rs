use {super::*, crate::NekosLifeError, serde::Deserialize};

const KNOWN_VARIANTS: [&str; 15] = [
    "Very Likely",
    "Wait For It",
    "Yes",
    "Absolutely",
    "It will pass",
    "count on it",
    "cannot tell now",
    "Maybe",
    "Not Now",
    "It is OK",
    "You're hot",
    "Ask Again",
    "No",
    "No doubt",
    "Go For It",
];

#[tokio::test]
async fn test_enum_string() {
    use std::str::FromStr;

    for message_string in KNOWN_VARIANTS.iter() {
        let f = EightBallMessage::from_str(
            &message_string.replace(" ", ""),
        );
        println!("{} -> {:?}", message_string, f);
        assert!(f.is_ok());
    }
}

#[tokio::test]
#[ignore]
async fn get_all_answer() -> Result<(), NekosLifeError> {
    let arr =
        std::collections::HashSet::from(KNOWN_VARIANTS)
            .into_iter()
            .map(ToString::to_string)
            .collect::<std::collections::HashSet<_>>();

    let client = reqwest::Client::new();

    let mut col = std::collections::HashSet::new();

    #[derive(Deserialize)]
    struct Model {
        response: String,
    }

    for res in std::iter::repeat_with(|| async {
        client
            .get(crate::BASEURL.join("8ball").unwrap())
            .send()
            .await
            .unwrap()
            .json::<Model>()
            .await
            .unwrap()
    })
    .take(30)
    {
        let res = res.await.response;

        println!("rr: {}", res);

        col.insert(res);
    }

    let diff = col.difference(&arr);

    println!("diff: {:#?}", diff);

    Ok(assert!(diff.count() == 0))
}
