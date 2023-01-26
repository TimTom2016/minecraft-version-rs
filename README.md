# minecraft-version
Exhaustive enum of Minecraft versions

## How to use
```rust
use minecraft_version::MinecraftVersion;

fn main() {
    let my_version = MinecraftVersion::from_str("1.19.3").unwrap();
    println!("Your selected version is {}", my_version);
}
```

## But how?
Invoking [their API](https://launchermeta.mojang.com/mc/game/version_manifest.json) by crate named `reqwest` at compile time.

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in minecraft-version by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 
