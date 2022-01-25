/*!
 * nekos.life wrapper library
 */
#![deny(missing_docs)]

#[macro_use]
mod r#macro;
mod implementation;
mod r#static;
mod types;

#[allow(dead_code)]
use r#static::BASEURL;

pub use {
    implementation::{
        category::{self, Category},
        get, get_with_client,
        types::IntoUrl,
    },
    types::{NekosLifeError, Response, Result, UrlString},
};

#[cfg(feature = "blocking")]
pub use implementation::blocking::{
    self, get as blocking_get,
    get_with_client as blocking_get_with_client,
};
