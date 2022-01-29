use crate::NekosLifeError;

make_text_endpoints! {
    Cat =>
    OwOify @ 'a |> str := owo <| ~~> owoify !# 1..=200 =>
    Why =>
    Fact =>
    Spoiler @ 'b |> str := owo <| !# 1..=1000
}
#[cfg(test)]
mod tests;
