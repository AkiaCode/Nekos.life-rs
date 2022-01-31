use {
    nekoslife::{Category, CategoryIter},
    rand::prelude::IteratorRandom,
};

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();

    println!(
        "randome pick: {}",
        nekoslife::get(
            Category::iter()
                .choose(&mut rng)
                .unwrap()
        )
        .await
        .unwrap()
    );
}
