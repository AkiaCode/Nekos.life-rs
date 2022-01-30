#![allow(missing_docs)]

use serde::Deserialize;

use crate::IntoUrl;

pub struct EightBall;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum EightBallMessage {
    #[serde(rename(deserialize = "Very Likely"))]
    VeryLikely,
    #[serde(rename(deserialize = "Wait For It"))]
    WaitForIt,
    Yes,
    Absolutely,
    #[serde(rename(deserialize = "It will pass"))]
    ItWillPass,
    #[serde(rename(deserialize = "count on it"))]
    CountOnIt,
    #[serde(rename(deserialize = "cannot tell now"))]
    CannotTellNow,
    Maybe,
    #[serde(rename(deserialize = "Not Now"))]
    NotNow,
    #[serde(rename(deserialize = "It is OK"))]
    ItIsOk,
    #[serde(rename(deserialize = "You're hot"))]
    YouAreHot,
    #[serde(rename(deserialize = "Ask Again"))]
    AskAgain,
    No,
    #[serde(rename(deserialize = "No doubt"))]
    NoDoubt,
    #[serde(rename(deserialize = "Go For It"))]
    GoForIt,
}

#[derive(Deserialize, Debug)]
pub struct EightBallResponse {
    pub response: EightBallMessage,
    pub url: crate::UrlString,
}

impl IntoUrl for EightBall {
    type Response = EightBallResponse;

    type Fut = into_url_fut! {};

    fn into_url(self) -> crate::Result<url::Url> {
        Ok(crate::BASEURL.join("8ball")?)
    }

    fn parse(res: reqwest::Response) -> Self::Fut {
        println!("{:#?}", res);

        Box::pin(async move {
            Ok(res.json::<EightBallResponse>().await?)
        })
    }
}

#[cfg(test)]
mod tests;
