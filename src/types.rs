//! Reusable types of nekoslife crate.

/// A specialized [`Result`] type for [nekoslife](crate) crate.
///
/// This type broadly may be used in this crate, such as in the function signatures.
///
/// The purpose of the existence of this type is to prevent writing
/// repetitive type signatures such as `Result<String, Error>`.
///
/// and if you want to use this type in your code,
/// you have to give this another name\
/// to prevent shadowing the prelude's [`Result`],
/// or use a full name, such as `nekoslife::Result`.
///
/// # Examples
///
/// ```rust
/// # use nekoslife::UnitResult;
/// // this function can return the `Error`!
/// #[tokio::main]
/// async fn main() -> UnitResult {
///     // get url from API.
///     let res: nekoslife::Result<nekoslife::UrlString> = nekoslife::get("neko").await;
///     
///     // do something with the url.
///     println!("result url: {}", res?);
///
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;

/// most concise type that represents `()` or [`Error`](error::Error).
///
/// # Usage
///
/// this type is very useful when particular code,\
/// such as uniit testing or doc test etc.
///
/// unlike [`Result`](self::Result) of nekoslife crate,\
/// it has different name then [`Result`](self::Result) of prelude.
///
/// so it can be used without any 'addiontal path', such as `UnitResult` rather than `nekoslife::Result<()>`.
pub type UnitResult = self::Result<()>;

/// Type that represents the result url.
///
/// This is just type alias for [String],
/// and you can clarify the types or signatures
/// by using this type instead of just using String,\
/// which is arbitrary and hard to understand exactly what it means.
///
/// # Examples
///
/// ```rust
/// pub struct Benchmark {
///    pub result_url: nekoslife::UrlString,
///    pub start: std::time::Instant,
///    pub end: std::time::Instant,
/// }
/// ```
pub type UrlString = String;

/// The return type of the [`get`](crate::get) or other functions.
///
/// This type allows you to write single type (without generics)\
/// for the return / result types
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
/// // note that Response is alias for nekoslife::Result,
/// // so you need to unwrap or match it.
/// let image_url: Response<Category> = get(Category::Neko).await;
/// # }
/// ```
/// this is equals to
/// ```
/// # use nekoslife::{
/// #   get,
/// #  Category,
/// # };
/// # #[tokio::main]
/// # async fn main() {
/// let image_url: Result<<Category as nekoslife::IntoUrl>::Response, nekoslife::Error> = get(Category::Neko).await;
/// # }
/// ```
#[rustfmt::skip]
pub type Response<T> =
    self::Result<
        <
            T as crate::IntoUrl
        >::Response
    >;

pub mod error;
use error::Error;
