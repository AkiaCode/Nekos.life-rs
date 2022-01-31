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
    /// the response type of this endpoint is [String].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// let cat = nekoslife::get(nekoslife::Cat).await?;
    ///
    /// println!("{}", cat); // such like "(｡･ω･｡)"
    /// # Ok(())
    /// # }
    /// ```
    Cat =>

    /// Represents the `owoify` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// with the text you want to convert
    /// to get the owoified version of the text.
    ///
    /// the response type of this endpoint is [String].
    ///
    /// this struct has one field,
    /// where you have to input the text you want to convert.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// let owo = nekoslife::get(nekoslife::OwOify("hello, world")).await?;
    ///
    /// assert_eq!("hewwo, wowwd", owo);
    /// # Ok(())
    /// # }
    /// ```
    OwOify @ 'a |> str := owo <| ~~> owoify !# 1..=200 =>

    /// Represents the `why` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// to get some intersting questions.
    ///
    /// the response type of this endpoint is [String].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// let why = nekoslife::get(nekoslife::Why).await?;
    ///
    /// println!("{}", why); // such like "why are there seashells on mt everest?"
    /// # Ok(())
    /// # }
    /// ```
    Why =>

    /// Represents the `fact` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// to get the fact about intersting things.
    ///
    /// the response type of this endpoint is [String].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// let fact = nekoslife::get(nekoslife::Fact).await?;
    ///
    /// println!("{}", fact); // such like "There are over 500 different types of bananas"
    /// # Ok(())
    /// # }
    /// ```
    Fact =>

    /// Represents the `spoiler` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// to get the discord-style spoiler text.
    ///
    /// the response type of this endpoint is [String].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// let spoiler = nekoslife::get(nekoslife::Spoiler("Akiacode")).await?;
    ///
    /// assert_eq!(spoiler, "||A||||k||||i||||a||||c||||o||||d||||e||");
    /// # Ok(())
    /// # }
    /// ```
    Spoiler @ 'b |> str := owo <| !# 1..=1000 =>

    /// Represents the `cat` endpoint.
    ///
    /// you can pass this struct directly to [`get`](crate::get) function
    /// to get the cute cat ascii arts.
    ///
    /// the response type of this endpoint is [String].
    ///
    /// this struct has one field,
    /// where you have to input the text you want to convert.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> nekoslife::UnitResult {
    /// let name = nekoslife::get(nekoslife::Name).await?;
    ///
    /// println!("{}", name); // such like "Fence Monster"
    /// # Ok(())
    /// # }
    /// ```
    Name
}

pub mod eight_ball;
pub use eight_ball::EightBall;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_range;
