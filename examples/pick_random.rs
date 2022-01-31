use {
    nekoslife::{Category, CategoryIter},
    rand::prelude::IteratorRandom,
};

#[tokio::main]
async fn main() {
    // make new ThreadRng
    let mut rng = rand::thread_rng();

    println!(
        "randome pick: {}",
        nekoslife::get(
            Category::iter()
                .choose(&mut rng) // pick a random one at iterator
                .unwrap()
        )
        .await
        .unwrap()
    );
}
