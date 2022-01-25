/*!
 * nekos.life wrapper library
 */
#![deny(missing_docs)]

mod category;
mod r#static;
mod types;

#[cfg(not(feature = "blocking"))]
mod implementation;

#[cfg(feature = "blocking")]
#[path = "blocking.rs"]
mod implementation;

pub use {
    category::Category,
    implementation::{get, get_with_client},
    types::{NekosLifeError, Response, Result, UrlString},
};
