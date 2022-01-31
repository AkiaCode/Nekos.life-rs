//! items to interact with [`8ball`](self) endpoint.
//!
//! * [`EightBall`] is the struct that implements the [`IntoUrl`] trait.\
//! you can use this struct as an argument to [`get`](crate::get) function
//! to get the [`EightBallResponse`].
//!
//! * [`EightBallResponse`] represents the response of this endpoint.\
//! you may receive the answer for your question with the message and image url.\
//! and the message will be parsed to [`EightBallMessage`].
//!
//! * [`EightBallMessage`] represents all kind of messages from the API.

use {crate::IntoUrl, serde::Deserialize};

/// Represents a `8ball` endpoint.
///
/// this struct can be used to get the response
/// of the `8ball` endpoint from API.
///
/// see [module level documentation](self) for more details.
///
/// # Examples
///
/// ```rust
/// # #[tokio::main]
/// # async fn main() -> nekoslife::UnitResult {
/// match nekoslife::get(nekoslife::EightBall)
///     .await? {
///     nekoslife::EightBallResponse {
///         response, // response enum
///         url, // url string of image
///     } => {
///         // do something
///         println!("{:?}", response);
///         assert_ne!(
///             url.len(),
///             0usize,
///         );
///     },
///     _ => unreachable!(),
/// };
/// # Ok(())
/// # }
/// ```
pub struct EightBall;

pair! {
    /// Represents all kind of response messages from `8ball` endpoint.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// match nekoslife::get(nekoslife::EightBall)
    ///     .await? {
    ///     nekoslife::EightBallResponse {
    ///         response, // response enum
    ///         url: _url, // url string of image
    ///     } => {
    ///         use nekoslife::EightBallMessage::*;
    ///
    ///         // do something
    ///         match response {
    ///             VeryLikely => println!("you are very likely"),
    ///             Yes => println!("yes!"),
    ///             CannotTellNow => println!("I can't tell you now"),
    ///             Maybe => println!("I'm not sure now"),
    ///             AskAgain => println!("please ask again :("),
    ///             No => println!("don't do it!"),
    ///             GoForIt => println!("it looks good. go ahead!"),
    ///             _ => println!("It is too complicated for me to answer"),
    ///         }
    ///     },
    ///     _ => unreachable!(),
    /// };
    /// # Ok(())
    /// # }
    /// ```
    #[derive(
        Debug,
        Deserialize,
        PartialEq,
        Eq,
    )]
    #[non_exhaustive]
    pub enum EightBallMessage {
        VeryLikely <"Very Likely">,
        WaitForIt <"Wait For It">,
        Yes<"Yes">,
        Absolutely <"Absolutely">,
        ItWillPass <"It will pass"> ,
        CountOnIt <"count on it">,
        CannotTellNow <"cannot tell now">,
        Maybe <"Maybe">,
        NotNow <"Not Now">,
        ItIsOk <"It is OK">,
        YouAreHot <"You're hot">,
        AskAgain <"Ask Again">,
        No <"No">,
        NoDoubt <"No doubt">,
        GoForIt <"Go For It">,
    }
}

/// Represents a response from `8ball` endpoint.
///
/// this type is consisted of `response` and `url` fields:
///
/// * `response` is a [`EightBallMessage`] enum.
/// it can represent various kind messages,\
/// such like `Cannot tell now`, `Wait for it`, etc.
///
/// * `url` is a [`UrlString`](crate::UrlString)
/// type which represents a answer image url.\
/// each image is paired with a response message.
///
/// # Examples
///
/// ```rust
/// # #[tokio::main]
/// # async fn main() -> nekoslife::UnitResult {
/// let eight_ball_res: nekoslife::EightBallResponse = nekoslife::get(
///     nekoslife::EightBall
/// ).await?;
///
/// // such like "Yes"
/// println!("my answer: {}", eight_ball_res.response);
///
/// // such like "https://cdn.nekos.life/8ball/Yes.png"
/// println!("image url: {}", eight_ball_res.url);
/// # Ok(())
/// # }
/// ```
#[derive(Deserialize, Debug)]
pub struct EightBallResponse {
    /// response message
    ///
    /// this is [`EightBallMessage`] enum.
    pub response: EightBallMessage,
    /// The url of the answer image.
    ///
    /// the image is paired with the response message.
    pub url: crate::UrlString,
}

impl IntoUrl for EightBall {
    type Response = EightBallResponse;

    type Fut = into_url_fut! {};

    fn into_url(self) -> crate::Result<url::Url> {
        Ok(crate::BASEURL.join("8ball")?)
    }

    fn parse(res: reqwest::Response) -> Self::Fut {
        Box::pin(async move {
            // parse response body with EightyBallResponse
            // response field will be parsed as EightBallMessage enum.
            Ok(res.json::<EightBallResponse>().await?)
        })
    }
}

#[cfg(test)]
mod tests;
