use hex_literal::hex;
use sha1::{digest::Output, Digest, Sha1};

#[derive(Debug, Clone, Copy)]
pub struct MakepkgPatch {
    pub custom_content: &'static str,
    pub custom_sha1sum: [u8; 20],
    pub original_sha1sum: [u8; 20],
}

impl MakepkgPatch {
    pub fn find_patch(list: &[Self], original_content: &[u8]) -> Result<Self, Output<Sha1>> {
        let mut hasher = Sha1::new();
        hasher.update(original_content);
        let hash = hasher.finalize();
        list.iter()
            .copied()
            .find(|patch| {
                patch.original_sha1sum == hash.as_slice() || patch.custom_sha1sum == hash.as_slice()
            })
            .ok_or(hash)
    }
}

pub const MAKEPKG_PATCHES: [MakepkgPatch; 5] = [
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg5"),
        custom_sha1sum: hex!("4ca18588b27b7e10b2329fb4e636ab33208a9915"),
        original_sha1sum: hex!("b3fc29f045a5a00b927e14d43d632efe8aa74c6a"),
    },
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg6"),
        custom_sha1sum: hex!("8fd61bf5635ec3a64b623272c212430496ade15f"),
        original_sha1sum: hex!("7823557b88e5390b38ec91a8bf931a966eda018b"),
    },
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg6"),
        custom_sha1sum: hex!("8fd61bf5635ec3a64b623272c212430496ade15f"),
        original_sha1sum: hex!("c76d418d3ddb285559e18dfeba40f1731b30acdc"),
    },
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg6"),
        custom_sha1sum: hex!("8fd61bf5635ec3a64b623272c212430496ade15f"),
        original_sha1sum: hex!("fb8197785e4985c561264af9ffcc67391e5220d6"),
    },
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg6"),
        custom_sha1sum: hex!("8fd61bf5635ec3a64b623272c212430496ade15f"),
        original_sha1sum: hex!("b91192d0c4d06643e376a04e5fb85335877550e3"),
    },
];

#[test]
fn test_custom_makepkg_sha1sum() {
    use hex_fmt::HexFmt;
    use sha1::{Digest, Sha1};
    for MakepkgPatch {
        custom_content,
        custom_sha1sum,
        ..
    } in MAKEPKG_PATCHES.iter().copied()
    {
        let mut hasher = Sha1::new();
        hasher.update(custom_content);
        let actual = hasher.finalize();
        eprintln!("expect: {}", HexFmt(&custom_sha1sum));
        eprintln!("actual: {}", HexFmt(actual.as_slice()));
        assert_eq!(actual.as_slice(), &custom_sha1sum);
    }
}
