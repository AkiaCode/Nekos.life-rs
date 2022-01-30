#![allow(missing_docs)]

use serde::Deserialize;

#[derive(Debug, strum::EnumString, Deserialize)]
pub enum EightBallMessage {
    VeryLikely,
    WaitForIt,
    Yes,
    Absolutely,
    #[strum(serialize = "Itwillpass")]
    ItWillPass,
    #[strum(serialize = "countonit")]
    CountOnIt,
    #[strum(serialize = "cannottellnow")]
    CannotTellNow,
    Maybe,
    NotNow,
    #[strum(serialize = "ItisOK")]
    ItIsOk,
    #[strum(serialize = "You'rehot")]
    YouAreHot,
    AskAgain,
    No,
    #[strum(serialize = "Nodoubt")]
    NoDoubt,
    GoForIt,
}

#[derive(Deserialize)]
pub struct EightBallResponse {
    pub response: EightBallMessage,
    pub url: crate::UrlString,
}

#[cfg(test)]
mod tests;
