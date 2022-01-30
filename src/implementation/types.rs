//! important types to interact with the API.

/// A trait which must be implemented to be passed by the [`get`](super::get) function.
///
/// this is slightly different from the [`TryInto`] trait in `std`,
/// since Rust does not allow to derive foreign traits to foreign types,\
/// such as implementing [`TryInto<url::Url>`] for [`&str`].
///
/// # Usage
///
/// this trait has two important methods:
///
/// * `into_url`: this method convert the type into a [`url::Url`], or [`Error`](crate::Error).
/// * `parse`: this method parses the response body into the `Self::Response`.
///
/// for example, if you need `MyEndpoint` to be used as an argument of [`get`](super::get),
///
/// you may implement this trait as follows:
///
/// ```rust
/// use std::{
///     pin::Pin,
///     future::Future,
/// };
///
/// # #[tokio::main]
/// # async fn main() -> nekoslife::UnitResult {
/// // define the type
/// struct MyEndpoint;
///
/// impl nekoslife::IntoUrl for MyEndpoint {
///     // the return type of the `parse` method.
///     type Response = String;
///
///     // the future type which will be actually returned by the `parse` method.
///     type Fut = Pin<Box<
///         dyn Future<Output = nekoslife::Result<Self::Response>>
///     >>;
///
///     // convert the type into a Url.
///     fn into_url(self) -> nekoslife::Result<url::Url> {
///         // make some url the way you want.
///         Ok(url::Url::parse("https://jsonplaceholder.typicode.com/todos/1")?)
///
///         // you may also extend the BASEURL like below.
///         // Ok(nekoslife::BASEURL.join("img/neko")?)
///         // for more information, check out the BASEURL document.
///     }
///
///     // parse the response body into the `Self::Response`.
///     fn parse(res: reqwest::Response) -> Self::Fut {
///         // parse the response body way you want as well.
///
///         // in this case, we just deserialize the response body
///         // into a struct below.
///         #[derive(serde::Deserialize)]
///         struct MyResponse {
///             title: String,
///         };
///
///         Box::pin(async move {
///             Ok(
///                 // parse the response body into the struct.
///                 serde_json::from_str::<MyResponse>(
///                     &res
///                         .text()
///                         .await?
///                         // this will return some text
///                         // which is a valid json we want,
///                         // such like:
///                         // {
///                         //     "userId": 1,
///                         //     "id": 1,
///                         //     "title": "delectus aut autem",
///                         //     "completed": false
///                         // }
///                         // so we can deserialize it.
///                 )
///                     .unwrap()
///                     .title
///             )
///         })
///     }
/// }
///
/// assert_ne!(
///     // you can pass any type
///     // that implements `IntoUrl` to get function.
///     nekoslife::get(MyEndpoint)
///         .await?
///         .len(),
///     0usize
/// );
/// # Ok(())
/// # }
/// ```
pub trait IntoUrl {
    /// Response type that will be returned by the `get` like methods.
    type Response;

    /// Future type which will be actually returned by the `parse` method.
    type Fut: std::future::Future<
        Output = crate::Result<Self::Response>,
    >;

    /// consumes itself and returns a [`Result`](crate::Result) of [`url::Url`].
    ///
    /// Examples
    ///
    /// ```rust
    /// // define the custom endpoint type
    /// struct OwOify(String);
    ///
    /// impl nekoslife::IntoUrl for OwOify {
    ///     type Response = String;
    ///
    ///     type Fut = nekoslife::into_url_fut!();
    ///
    ///     fn into_url(self) -> nekoslife::Result<url::Url> {
    ///         Ok({
    ///             // we will extend the BASEURL here.
    ///             let mut url = nekoslife::BASEURL
    ///                 .join("owoify")?;
    ///                 // it now looks like this:
    ///                 // "https://nekos.life/api/v2/owoify"
    ///
    ///             url
    ///                 .query_pairs_mut()
    ///                 .append_pair("text", self.0.as_str());
    ///                 // then add some query parameters to it.
    ///                 // now it may look like:
    ///                 // "https://nekos.life/api/v2/owoify?text=<self.0>"
    ///             url
    ///         })
    ///     }
    ///
    ///     // implement nothing in this time.
    ///     fn parse(res: reqwest::Response) -> Self::Fut {
    ///         unimplemented!()
    ///     }
    /// }
    /// ```
    fn into_url(self) -> crate::Result<url::Url>;

    /// Parse the body of the response
    ///
    /// note that the first argument is [`reqwest::Response`].\
    /// this means you may need some async operations to get entire response body.
    fn parse(res: reqwest::Response) -> Self::Fut;
}
