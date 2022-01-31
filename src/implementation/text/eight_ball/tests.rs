use {
    super::*,
    crate::UnitResult,
    pretty_assertions::{assert_eq, assert_ne},
    serde::Deserialize,
    serde_json::{self, from_str},
};

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

#[test]
fn string_to_eight_ball_message() {
    let variant = r#""Wait For It""#;

    assert_eq!(
        from_str::<EightBallMessage>(variant).unwrap(),
        EightBallMessage::WaitForIt
    );
}

#[test]
fn all_known_strings_can_be_deserialized_to_eight_ball_message(
) {
    for message_string in KNOWN_VARIANTS.iter() {
        let f = from_str::<EightBallMessage>(&format!(
            r#""{}""#,
            message_string
        ));

        println!("{} -> {:?}", message_string, f);
        assert!(f.is_ok());
    }
}

#[derive(Deserialize)]
struct Model {
    response: String,
}

#[tokio::test]
#[ignore]
async fn get_all_answer() -> UnitResult {
    let client = reqwest::Client::new();

    let mut col = std::collections::HashSet::new();

    for _ in 1..=3 {
        let res = client
            .get(crate::BASEURL.join("8ball")?)
            .send()
            .await?
            .json::<Model>()
            .await?
            .response;

        println!("rr: {}", res);

        col.insert(res);
    }

    let original =
        &std::collections::HashSet::from(KNOWN_VARIANTS)
            .into_iter()
            .map(ToString::to_string)
            .collect::<std::collections::HashSet<_>>();

    let diff = col.difference(original);

    println!("diff: {:#?}", diff);

    Ok(assert!(diff.count() == 0))
}

#[test]
fn deserialize_from_json() {
    let json = r#"{
    "response": "Not Now",
    "url": "https://example.com"
}"#;

    println!("{}", json);

    let parsed =
        from_str::<EightBallResponse>(json).unwrap();

    println!("{:#?}", parsed);

    assert_ne!(parsed.url.len(), 0);
}

#[tokio::test]
async fn get_8ball_test() -> UnitResult {
    let eight_ball_res: EightBallResponse =
        crate::get(EightBall).await?;

    Ok(assert_ne!(eight_ball_res.url.len(), 0))
}

#[test]
fn to_string() {
    assert_eq!(
        EightBallMessage::WaitForIt.to_string(),
        "Wait For It"
    )
}

#[test]
fn into_static_str() {
    assert_eq!(
        Into::<&'static str>::into(
            EightBallMessage::WaitForIt
        ),
        "Wait For It"
    )
}
