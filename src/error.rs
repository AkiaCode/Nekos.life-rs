/// Any error that can occur while accessing the api.
#[derive(thiserror::Error, Debug)]
pub enum NekosLifeError {
    /// represents errors from [`reqwest`]
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("invalid url was provided")]
    #[allow(missing_docs)]
    UrlParseError(#[from] url::ParseError),

    /// occurs when failed to create new tokio runtime
    #[error("unable to create runtime")]
    RuntimeError(#[from] std::io::Error),

    #[cfg(test)]
    #[error(
        "error occurred while unit testing: {message}"
    )]
    TestError { message: String },
}

#[cfg(test)]
impl NekosLifeError {
    pub fn new_unittest_error(msg: &str) -> Self {
        Self::TestError {
            message: msg.to_string(),
        }
    }
}
