# Nekos.life-rs

### Use

Add to dependencies:
```toml
[dependencies]
nekoslife-rs = "0.1.2"
```

### Features

```toml
[dependencies.nekoslife-rs]
version = "0.1.2"

# disable nsfw
default-features = false
features = ["default-tls", "sfw"]

# or disable sfw
default-features = false
features = ["default-tls", "nsfw"]
```

### In code

Example: get a waifu:
```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String = nekoslife_rs::get(nekoslife_rs::SfwCategory::Waifu).await?;

    println!("{}", url);

    Ok(())
}
```

With the `blocking` feature, just remove the `.await`.
