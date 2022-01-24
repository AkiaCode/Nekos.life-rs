/*!
 * nekos.life wrapper library
 */
#![deny(missing_docs)]

mod category;
mod error;
mod r#static;

mod implementation;

#[cfg(feature = "blocking")]
mod blocking;

pub use {
    category::Category,
    error::NekosLifeError,
    implementation::{get, get_with_client},
};

#[cfg(feature = "blocking")]
pub use blocking::{
    blocking_get, blocking_get_with_client,
};
