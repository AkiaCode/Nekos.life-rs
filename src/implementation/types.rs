//! important types to interact with the API.

use crate::types::{self, UrlString};

// represents the response body
// for serde deserialization
#[derive(serde::Deserialize, Debug)]
pub(crate) struct ApiResponseBody {
    pub(crate) url: UrlString,
}

/// a trait that must be implemented to be passed by the [`get`](super::get) function.
///
/// this is slightly different from the [`TryInto`] trait in `std`,
/// since Rust does not allow to derive foreign traits to foreign types,\
/// such as implementing [`TryInto<url::Url>`] for [`&str`].
pub trait IntoUrl {
    /// consumes itself and returns a [`Result`](crate::types::Result) of [`UrlString`].
    fn into_url(self) -> types::Result<url::Url>;
}
