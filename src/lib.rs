#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::cargo)]
#![warn(clippy::pedantic, clippy::nursery)]

mod gen;

pub use gen::MinecraftVersion;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn ord_is_release_order() {
        assert!(MinecraftVersion::_1_19_3 > MinecraftVersion::_1_19_2);
    }

    #[test]
    fn eq_reflexibity() {
        assert!(MinecraftVersion::_1_19_3 == MinecraftVersion::_1_19_3);
    }

    #[test]
    fn from_str_takes_version() {
        assert!(MinecraftVersion::from_str("1.19.3").unwrap() == MinecraftVersion::_1_19_3);
    }

    #[test]
    fn version_is_display() {
        assert_eq!(MinecraftVersion::_1_19_3.to_string(), "1.19.3".to_string());
    }
}
