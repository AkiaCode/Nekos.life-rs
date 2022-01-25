//! Reusable types of nekoslife crate.

/// The error type for all kind of errors that can occur while accessing the api.
#[derive(thiserror::Error, Debug)]
pub enum NekosLifeError {
    /// network errors from [`reqwest`]
    ///
    /// in general, this error will occur when the given url is 404,\
    /// such like: `reqwest::Error { kind: Decode, source: Error("missing field url", line: X, column: Y) }`,\
    /// which means reqwest couldn't find the `url` field in the response body.
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("invalid url was provided")]
    /// parsing errors from [`url::ParseError`]
    ///
    /// this error may occur if the provided url was invalid.
    /// for example:
    ///
    /// * when malformed [`BASEURL`] was given
    /// * when invlid Category was given
    UrlParseError(#[from] url::ParseError),

    /// async runtime error from [`std::io::Error`]
    ///
    /// occurs when failed to create new tokio runtime
    #[error("unable to create runtime")]
    RuntimeError(#[from] std::io::Error),
}

#[cfg(test)]
#[derive(Debug, thiserror::Error)]
#[error("unittest error: {0}")]
pub(crate) struct UnitTestError(String);

#[cfg(test)]
impl UnitTestError {
    pub fn new(message: &str) -> Self {
        Self(message.to_owned())
    }
}

/// A specialized [`Result`] type for [nekoslife](crate) crate.
///
/// This type broadly may be used in this crate, such as in the function signatures.
///
/// The purpose of the existence of this type is to prevent writing
/// repetitive type signatures such as `Result<String, NekosLifeError>`.\
/// and if you want to use this type in your code,
/// you have to give this another name to prevent shadowing the prelude's [`Result`],
/// or use a full name, such as `nekoslife::Result`.
pub type Result<T> = std::result::Result<T, NekosLifeError>;

/// Type that represents the result url.
///
/// This is type alias for [String],
/// and you can clarify the types or signatures
/// by using this type instead of just using String,
/// which is arbitrary and hard to understand exactly what it means.
pub type UrlString = String;

/// The return type of the [`get`](crate::get) or other functions.
///
/// This type allows you to wire single type (without generics) for the return / result types
/// of the [`get`](crate::get), [`get_with_client`](crate::get_with_client), [`blocking::get`](crate::blocking::get)
/// and [`blocking::get_with_client`](crate::blocking::get_with_client) functions.
///
/// # Usage
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() {
/// use nekoslife::{
///     get,
///     Category,
///     Response,
/// };
///
/// // note that Response is alias for Result,
/// // so you need to unwrap or match it.
/// let image_url: Response = get(Category::Neko).await;
/// // this is equals to
/// // let image_url: Result<String, nekoslife::NekosLifeError> = get(Category::Neko).await;
/// # }
/// ```
pub type Response = self::Result<UrlString>;
