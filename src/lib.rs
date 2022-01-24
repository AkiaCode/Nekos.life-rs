/*!
 * nekos.life wrapper library
 */
#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod category;
mod error;
mod r#static;

#[cfg(not(feature = "blocking"))]
mod implementation;

#[cfg(feature = "blocking")]
#[path = "blocking.rs"]
mod implementation;

pub use {
    category::Category,
    error::NekosLifeError,
    implementation::{get, get_with_client},
};
