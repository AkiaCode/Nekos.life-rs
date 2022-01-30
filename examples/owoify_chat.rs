use {tokio::io::AsyncBufReadExt, tokio_stream::StreamExt};

#[tokio::main]
async fn main() -> nekoslife::Result<()> {
    let client = reqwest::Client::new();

    println!("[OwO chat]\n\nsay 'hello' to owo!\n");

    Ok(
        while let Some(v) =
            tokio_stream::wrappers::LinesStream::new(
                tokio::io::BufReader::new(
                    tokio::io::stdin(),
                )
                .lines(),
            )
            .filter_map(|line| Some(line.unwrap()))
            .map(|line| {
                let cl = &client;

                async move {
                    nekoslife::get_with_client(
                        cl,
                        nekoslife::OwOify(line.as_str()),
                    )
                    .await
                }
            })
            .next()
            .await
        {
            println!(
                "owo: {}\n(type your message)",
                v.await?
            );
        },
    )
}
