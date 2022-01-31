//! The public api of the implementation.
//!
//! This module consists of four parts:
//!
//! 1. implementations part:
//! these are the most used functions of this crate.
//!
//!     - [`get`]: just get the response from the given endpoint.
//!
//!     - [`get_with_client`]: same as `get`, but user can pass a [`client`](reqwest::Client) to it.
//!
//! 2. [blocking](blocking) part:
//! these are the blocking versions of the above.
//!
//!     - [`blocking_get`](blocking::get): same as [`get`], but it is blocking.
//!     
//!     - [`blocking_get_with_client`](blocking::get_with_client): same as [`get_with_client`], but it is blocking as well.
//!
//! 3. endpoints part:
//! these are the endpoints that can be consumed by the above functions.
//!
//!     - [`category`]: the enum that represents the image categories.
//!
//!     - [`text`]: a module that contains all of the text based endpoints.
//!
//! 4. [types](types) part:
//! this module contains the [`IntoUrl`](crate::IntoUrl) trait\
//! which determines how the endpoints are converted to [`Url`](reqwest::Url).
//!
//! please see each module level documentation for more details.

use {
    crate::{IntoUrl, Response},
    reqwest::{self, Client},
};

/// Gets the image url with the given client.
///
/// # Note
///
#[doc = include_str!("../docs/context.md")]
///
/// [blocking_version]: crate::blocking_get_with_client
///
#[doc = include_str!("../docs/reusability_client.md")]
///
/// [get]: crate::get
///
#[doc = include_str!("../docs/return.md")]
///
/// # Examples
///
/// ```rust,no_run
#[doc = include_str!("../examples/get_with_client.rs")]
/// ```
pub async fn get_with_client<T>(
    client: &reqwest::Client,
    endpoint: T,
) -> Response<T>
where
    T: IntoUrl,
{
    Ok(<T as IntoUrl>::parse(
        client
            .get(
                <T as IntoUrl>::into_url(endpoint)?
                    .to_string()
                    .as_str(),
            )
            .send()
            .await?,
    )
    .await?)
}

/// Gets the image url.
///
/// # Note
///
#[doc = include_str!("../docs/context.md")]
///
/// [blocking_version]: crate::blocking_get
///
#[doc = include_str!("../docs/reusability.md")]
///
/// [get_with_client]: crate::get_with_client
///
#[doc = include_str!("../docs/return.md")]
///
/// # Examples
///
/// ```rust
#[doc = include_str!("../examples/get.rs")]
/// ```
///
/// you can also do it by passing a string as an argument.
///
/// ```rust
#[doc = include_str!("../examples/get_with_str.rs")]
/// ```
///
/// for more information, see [`Category`](crate::Category) or
/// [`IntoUrl`](crate::IntoUrl) documentation.
///
/// and you can use other endpoints as well.\
/// in below example, we use [`Cat`](crate::Cat) as an endpoint.
///
/// ```rust
#[doc = include_str!("../examples/get_cat.rs")]
/// ```
///
/// ```rust
#[doc = include_str!("../examples/get_owoify.rs")]
/// ```
pub async fn get<T>(endpoint: T) -> Response<T>
where
    T: IntoUrl,
{
    get_with_client(&Client::new(), endpoint).await
}

pub mod category;
pub mod text;
pub mod types;

#[cfg(test)]
mod tests;

/// blocking version of implementation
#[cfg(feature = "blocking")]
pub mod blocking;
