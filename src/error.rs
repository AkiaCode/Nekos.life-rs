/// Any error that can occur while accessing the api.
#[derive(thiserror::Error, Debug)]
pub enum NekosLifeError {
    /// represents errors from [`reqwest`]
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("invalid url was provided")]
    #[allow(missing_docs)]
    UrlParseError(#[from] url::ParseError),
}