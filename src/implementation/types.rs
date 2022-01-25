use crate::types::UrlString;

// represents the response body
// for serde deserialization
#[derive(serde::Deserialize)]
pub(crate) struct ApiResponseBody {
    pub(crate) url: UrlString,
}
