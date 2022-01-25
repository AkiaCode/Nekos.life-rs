/*!
 * nekos.life wrapper library
 */
#![deny(missing_docs)]

mod category;
mod r#static;
mod types;

mod implementation;

pub use {
    category::Category,
    implementation::{
        get, get_with_client, types::IntoUrl,
    },
    r#static::BASEURL,
    types::{NekosLifeError, Response, Result, UrlString},
};

#[cfg(feature = "blocking")]
pub use implementation::blocking::{
    self, get as blocking_get,
    get_with_client as blocking_get_with_client,
};
