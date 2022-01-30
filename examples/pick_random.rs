use {
    nekoslife,
    rand::{self, prelude::IteratorRandom},
};

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();

    println!(
        "randome pick: {}",
        nekoslife::get(
            <
                nekoslife::Category as strum::IntoEnumIterator
            >::iter()
                .choose(&mut rng)
                .unwrap()
        )
            .await
            .unwrap()
    );
}
