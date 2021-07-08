# Nekos.life-rs

### Use

Add to dependencies:
```toml
[dependencies]
nekoslife = "0.2.1"
```

### Features

```toml
[dependencies.nekoslife]
version = "0.2.1"

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
    let url: String = nekoslife::get(nekoslife::SfwCategory::Waifu).await?;

    println!("{}", url);

    Ok(())
}
```

With the `blocking` feature, just remove the `.await`.
