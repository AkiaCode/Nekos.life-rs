/*!
 * nekos.life wrapper library
 */
#![deny(missing_docs)]

#[macro_use]
mod r#macro;
pub mod implementation;
pub mod r#static;
pub mod types;

#[allow(unused_imports)]
#[doc(inline)]
pub use {
    implementation::{
        category::{self, Category},
        get, get_with_client,
        text::{
            self,
            eight_ball::{
                self, EightBallMessage, EightBallResponse,
            },
            Cat, EightBall, Fact, Name, OwOify, Spoiler,
            Why,
        },
        types::IntoUrl,
    },
    r#static::BASEURL,
    strum::IntoEnumIterator as CategoryIter,
    types::{
        error::{self, Error},
        Response, Result, UnitResult, UrlString,
    },
};

#[cfg(feature = "blocking")]
pub use implementation::blocking::{
    self, get as blocking_get,
    get_with_client as blocking_get_with_client,
};
