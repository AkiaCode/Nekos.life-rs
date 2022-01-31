//! Text-based endpoints.
//!
//! these endpoints all implement the [`IntoUrl`](crate::IntoUrl) trait.\
//! so you can use them as arguments to [`get`](crate::get) function,
//! and they will give you the response [String].
//!
//! you may input some text in certain endpoints.

make_text_endpoints! {
    /// Represents the `cat` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// to get the cute cat ascii arts.
    ///
    #[doc = include_str!("../../docs/response_is_string.md")]
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_str!("../../examples/get_cat.rs")]
    /// ```
    Cat =>

    /// Represents the `owoify` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// with the text you want to convert
    /// to get the owoified version of the text.
    ///
    #[doc = include_str!("../../docs/fields.md")]
    ///
    #[doc = include_str!("../../docs/response_is_string.md")]
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_str!("../../examples/get_owoify.rs")]
    /// ```
    OwOify @ 'a |> str := owo <| ~~> owoify !# 1..=200 =>

    /// Represents the `why` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// to get some intersting questions.
    ///
    #[doc = include_str!("../../docs/response_is_string.md")]
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_str!("../../examples/get_why.rs")]
    /// ```
    Why =>

    /// Represents the `fact` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// to get the fact about intersting things.
    ///
    #[doc = include_str!("../../docs/response_is_string.md")]
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_str!("../../examples/get_fact.rs")]
    /// ```
    Fact =>

    /// Represents the `spoiler` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// to get the discord-style spoiler text.
    ///
    #[doc = include_str!("../../docs/response_is_string.md")]
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_str!("../../examples/get_spoiler.rs")]
    /// ```
    Spoiler @ 'b |> str := owo <| !# 1..=1000 =>

    /// Represents the `name` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// to get the funny names.
    ///
    #[doc = include_str!("../../docs/fields.md")]
    ///
    #[doc = include_str!("../../docs/response_is_string.md")]
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_str!("../../examples/get_name.rs")]
    /// ```
    Name
}

pub mod eight_ball;
pub use eight_ball::EightBall;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_range;
