//! Nekos.life wrapper for Rust.
//!
//! # About
//!
//! this is nekos.life implementation for the rust programming language,\
//! you can find out more information about nekos.life at
//! [their website][nekos.life] and [github][github].
//!
//! this crate provides a way to interact with thier API,
//! to convert the result into useful and readable types.
//!
//! and provides both of async and blocking api as well.
//!
//! [nekos.life]: https://nekos.life/
//! [github]: https://github.com/Nekos-life/
//!
//! # Quick Start
//!
//! first of all, you need to add below to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! nekoslife = "0.2.1"
//! ```
//!
//! the easiest way to use this crate is
//! seding single request to `img` endpoint,
//!
//! ```rust
//! // we need async context to use 'get' method.
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 'get' method will return 'Future'.
//!     // so you have to use '.await' to get the result.
//!     // in this case, the return type
//!     // of the method is 'Result<String, Error>',
//!     // so we can use '?' operator here.
//!     let url: String =
//!         nekoslife::get(nekoslife::Category::Waifu).await?;
//!
//!     // print out the recieved url
//!     println!("{url}");
//!
//!     Ok(())
//! }
//! ```
//!
//! the [`get`](crate::get) function is one of the most important functions,\
//! it takes any type that implements [`IntoUrl`](crate::IntoUrl) trait,
//! and this case, the [`Category`](crate::Category) enum is that type.
//!
//! then it will return a [`Future`](std::future::Future) of
//! [`Result<UrlString, Error>`](crate::UrlString).
//!
//! you can also pass string instead of [`Category`](crate::Category).
//!
//! ```rust
//! # #[tokio::main]
//! # async fn main() -> nekoslife::UnitResult {
//! let result = nekoslife::get("neko").await?;
//! # Ok(())
//! # }
//! ```
//!
//! more information about strings and full list of available category variants,
//! check out [`Category`](crate::Category) document.
//!
//! # Blocking
//!
//! you can use `blocking` version of [`get`](crate::get) function,
//!
//! first, you need to enable the `blocking` feature from this crate.
//!
//! ```toml
//! [dependencies.nekoslife]
//! features = ["blocking"]
//! ```
//!
//! then, replace the [`get`](crate::get) function
//! with [`blocking::get`](crate::blocking::get).
//!
//! ```rust
//! // get the image url from 'Neko' category
//! let url = nekoslife::blocking::get(
//!     nekoslife::Category::Neko
//! )?;
//! // in this case, the return type will be 'String'
//!
//! // then do something with the url.
//! println!("{url}");
//! # Ok::<(), nekoslife::Error>(())
//! ```
//!
//! for more information, check out the [`implementation`](crate::implementation) and
//! the [`blocking`](crate::blocking) module.
//!
//! # Other Endpoints
//!
//! you can use more endpoints (not just img endpoint),
//!
//! for example, below uses [`OwOify`](crate::OwOify) endpoint.
//!
//! ```rust
//! # use nekoslife::UnitResult;
//! # #[tokio::main]
//! # async fn main() -> UnitResult {
//! // get owoified version of "hello, world"
//! let owo =
//!     nekoslife::get(nekoslife::OwOify("hello, world"))
//!         .await?;
//!
//! // this will be converted version of our text.
//! assert_eq!(owo, "hewwo, wowwd");
//! # Ok(())
//! # }
//! ```
//!
//! for more information about text based endpoints,
//! check out [`text`](crate::text) module.
//!
//! # License
//!
//! this crate is licensed under [MIT](https://opensource.org/licenses/MIT) license.

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

#[macro_use]
mod r#macro;
pub mod implementation;
pub mod r#static;
pub mod types;

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
#[doc(inline)]
pub use implementation::blocking::{
    self, get as blocking_get,
    get_with_client as blocking_get_with_client,
};
