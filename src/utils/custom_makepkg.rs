use hex_literal::hex;
pub const CUSTOM_MAKEPKG: &str = include_str!("./patches/makepkg");
pub const CUSTOM_MAKEPKG_SHA1SUM: [u8; 20] = hex!("4ca18588b27b7e10b2329fb4e636ab33208a9915");
pub const ORIGINAL_MAKEPKG_SHA1SUM: [u8; 20] = hex!("b3fc29f045a5a00b927e14d43d632efe8aa74c6a");
