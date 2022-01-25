/*!
 * nekos.life wrapper library
 */
#![deny(missing_docs)]

mod category;
mod error;
mod r#static;

mod implementation;

use r#static::BASEURL;

pub use {
    category::Category,
    error::NekosLifeError,
    implementation::{get, get_with_client},
};

#[cfg(feature = "blocking")]
pub use implementation::blocking::{
    self, get as blocking_get,
    get_with_client as blocking_get_with_client,
};
