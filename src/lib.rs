/*!
 * nekos.life wrapper library
 */
#![deny(missing_docs)]

#[macro_use]
mod r#macro;
mod implementation;
pub mod r#static;
mod types;

#[allow(unused_imports)]
#[doc(inline)]
pub use {
    implementation::{
        category::{self, Category},
        get, get_with_client,
        text::{self, Cat, Fact, OwOify, Spoiler, Why},
        types::IntoUrl,
    },
    r#static::BASEURL,
    strum::IntoEnumIterator as CategoryIter,
    types::{NekosLifeError, Response, Result, UrlString},
};

#[cfg(feature = "blocking")]
pub use implementation::blocking::{
    self, get as blocking_get,
    get_with_client as blocking_get_with_client,
};
