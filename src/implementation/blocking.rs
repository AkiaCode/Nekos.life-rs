//! blocking version of [`implementation`](crate::implementation).
//!
//! for more information, please read
//! [`implementation`](crate::implementation) documentation,\
//! or read each item-level documentation of this module.

#![cfg(feature = "blocking")]
#![cfg_attr(docsrs, doc(cfg(feature = "blocking")))]

use {
    super::{
        get_with_client as async_get_with_client,
        types::IntoUrl,
    },
    crate::Response,
    reqwest::{self, Client},
};

/// Gets the image url in blocking context with the given client.
///
/// # Note
///
#[doc = include_str!("../../docs/context_blocking.md")]
///
/// [original]: crate::get_with_client
///
#[doc = include_str!("../../docs/reusability_client.md")]
///
/// [get]: crate::blocking::get
///
#[doc = include_str!("../../docs/return.md")]
///
/// # Examples
///
/// ```rust,no_run
#[doc = include_str!("../../examples/blocking_get_with_client.rs")]
/// ```
pub fn get_with_client<T>(
    client: &reqwest::Client,
    endpoint: T,
) -> Response<T>
where
    T: IntoUrl,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()?
        .block_on(async_get_with_client(&client, endpoint))
}

/// Gets the image url in blocking context.
///
/// # Note
///
#[doc = include_str!("../../docs/context_blocking.md")]
///
/// [original]: crate::get_with_client
///
#[doc = include_str!("../../docs/reusability.md")]
///
/// [get_with_client]: crate::blocking::get_with_client
///
#[doc = include_str!("../../docs/return.md")]
///
/// # Examples
///
/// ```rust
#[doc = include_str!("../../examples/blocking_get.rs")]
/// ```
pub fn get<T>(endpoint: T) -> Response<T>
where
    T: IntoUrl,
{
    get_with_client(&Client::new(), endpoint)
}

#[cfg(test)]
mod tests;
