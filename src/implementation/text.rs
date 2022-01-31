//! other text-based endpoints

make_text_endpoints! {
    /// `/cat` endpoint
    Cat =>
    /// `/owoifu` endpoint
    OwOify @ 'a |> str := owo <| ~~> owoify !# 1..=200 =>
    /// `/why` endpoint
    Why =>
    /// `/fact` endpoint
    Fact =>
    /// `/spoiler` endpoint
    Spoiler @ 'b |> str := owo <| !# 1..=1000 =>
    /// `/name` endpoint
    Name
}

pub mod eight_ball;
pub use eight_ball::EightBall;

#[cfg(test)]
mod tests;
