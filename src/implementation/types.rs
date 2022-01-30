//! important types to interact with the API.

/// a trait that must be implemented to be passed by the [`get`](super::get) function.
///
/// this is slightly different from the [`TryInto`] trait in `std`,
/// since Rust does not allow to derive foreign traits to foreign types,\
/// such as implementing [`TryInto<url::Url>`] for [`&str`].
pub trait IntoUrl {
    /// Response type
    type Response;

    /// Future type for async method
    type Fut: std::future::Future<
        Output = crate::Result<Self::Response>,
    >;

    /// consumes itself and returns a [`Result`](crate::types::Result) of [`UrlString`].
    fn into_url(self) -> crate::Result<url::Url>;

    /// parse the body of the response
    fn parse(res: reqwest::Response) -> Self::Fut;
}
