/*!
 * nekos.life wrapper library
 */
#![deny(missing_docs)]

mod implementation;
mod r#static;
mod types;

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
