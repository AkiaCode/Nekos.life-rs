//! contains the [`Error`](Error) type.
//!
//! see [`Error`](Error) documentation for more information.

/// The error type for all kind of errors which can occur while accessing the api.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Network errors from [`reqwest`]
    ///
    /// in general, this error will occur when the given url is not reachable,\
    /// or the response body is not valid json etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// // define the custom endpoint
    /// struct MyEndpoint;
    ///
    /// // implement the IntoUrl trait
    /// // to be able to pass as an argument to `get` function.
    /// impl nekoslife::IntoUrl for MyEndpoint {
    ///     type Response = String;
    ///
    ///     type Fut = nekoslife::into_url_fut! {};
    ///
    ///     fn into_url(self) -> nekoslife::Result<url::Url> {
    ///         // url which doesn't exist
    ///         Ok(url::Url::parse("http://this.is.not.exists")?)
    ///     }
    ///
    ///     fn parse(res: reqwest::Response) -> Self::Fut {
    ///         // in this case, the error occurs
    ///         // before the response will be reached here.
    ///         unimplemented!()
    ///     }
    /// }
    ///
    /// // build the reqwest client with timeout
    /// let client = reqwest::Client::builder()
    ///     .timeout(std::time::Duration::from_millis(2))
    ///     .build()?;
    ///
    /// match nekoslife::get_with_client(
    ///     &client,
    ///     MyEndpoint,
    /// ).await {
    ///     Err(
    ///         nekoslife::Error::ReqwestError(err)
    ///     ) => assert!(err.is_timeout()),
    ///     _ => unreachable!(),
    /// };
    /// # Ok(())
    /// # }
    /// ```
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    /// parsing errors from [`url::ParseError`]
    ///
    /// this error may occur if the provided url was invalid.\
    /// for example:
    ///
    /// * when malformed [`BASEURL`](struct@crate::BASEURL) was given.
    /// * when invlid path was given.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// // define the custom endpoint
    /// struct MyEndpoint;
    ///
    /// // implement the IntoUrl trait
    /// // to be able to pass as an argument to `get` function.
    /// impl nekoslife::IntoUrl for MyEndpoint {
    ///     type Response = String;
    ///
    ///     type Fut = nekoslife::into_url_fut! {};
    ///
    ///     fn into_url(self) -> nekoslife::Result<url::Url> {
    ///         // invalid url joining
    ///         Ok(
    ///             url::Url::parse("ssh:nekos.life/api/v2/")?
    ///                 .join("my/endpoint")?
    ///         )
    ///         // it may failed to parse
    ///         // then return immediately by '?' operator.
    ///     }
    ///
    ///     fn parse(res: reqwest::Response) -> Self::Fut {
    ///         // in this case, the error occurs
    ///         // before the response will be reached here.
    ///         unimplemented!()
    ///     }
    /// }
    ///
    /// assert!(
    ///     matches!(
    ///         nekoslife::get(MyEndpoint).await,
    ///         Err(
    ///             nekoslife::Error::UrlParseError(
    ///                 url::ParseError::RelativeUrlWithCannotBeABaseBase
    ///             )
    ///         ),
    ///     )
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[error("invalid url was provided")]
    UrlParseError(#[from] url::ParseError),

    /// Async runtime error from [`std::io::Error`]
    ///
    /// occurs when failed to create new `tokio runtime`.
    #[error("unable to create runtime")]
    RuntimeError(#[from] std::io::Error),

    /// Invalid category string.
    ///
    /// Occurs when given string does not exists in the [`Category`](crate::Category) enum.
    ///
    /// this error contains [`strum::ParseError`] at `error` field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use nekoslife::{Spoiler, Error};
    /// # use pretty_assertions::assert_eq;
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// match nekoslife::get(
    ///     // unknown endpoint
    ///     "abiria"
    /// ).await {
    ///     // it would be unknown endpoint error
    ///     Err(Error::UnknownEndpoint {
    ///         endpoint_name,
    ///         parse_error: strum::ParseError::VariantNotFound,
    ///     }) => {
    ///         // "abiria" did not exist category.
    ///         assert_eq!(endpoint_name, "abiria");
    ///     },
    ///     _ => unreachable!(),
    /// };
    /// # Ok(())
    /// # }
    /// ```
    #[error("{parse_error}: `{endpoint_name}` is not a valid category or endpoint")]
    UnknownEndpoint {
        /// the url which couldn't be parsed
        endpoint_name: String,
        /// inner field that contains strum parse error
        parse_error: strum::ParseError,
    },

    /// Invalid range of content length.
    ///
    /// Occurs when user try to request an endpoint
    /// with too much text or to less text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use nekoslife::{Spoiler, Error};
    /// # use pretty_assertions::assert_eq;
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// match nekoslife::get(
    ///     // too much request
    ///     Spoiler(&"A".repeat(4567))
    /// ).await {
    ///     // it would be range error
    ///     Err(Error::OutOfRangeError {
    ///         endpoint_name,
    ///         range,
    ///     }) => {
    ///         assert_eq!(
    ///             endpoint_name,
    ///             "Spoiler".to_owned(),
    ///         );
    ///         // "Spoiler" endpoint has a range of 1~1000 chars.
    ///         assert_eq!(range, 1..=1000);
    ///     },
    ///     _ => unreachable!(),
    /// };
    /// # Ok(())
    /// # }
    /// ```
    #[error(
        "{endpoint_name} text must be between {start} and {end} characters",
        start = range.start(),
        end = range.end(),
    )]
    OutOfRangeError {
        /// endpoint name that failed to fetch due to this error
        endpoint_name: String,
        /// the valid range of given endpoint.
        range: std::ops::RangeInclusive<usize>,
    },
}

#[cfg(test)]
#[derive(Debug, thiserror::Error)]
#[error("unittest error: {0}")]
// small error type for unit tests.
pub(crate) struct UnitTestError(String);

#[cfg(test)]
impl UnitTestError {
    pub fn new(message: &str) -> Self {
        Self(message.to_owned())
    }
}
