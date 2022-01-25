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

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, NekosLifeError>;

#[allow(missing_docs)]
pub type UrlString = String;

#[allow(missing_docs)]
pub type Response = self::Result<UrlString>;
