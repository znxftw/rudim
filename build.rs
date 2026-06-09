// TODO: isolate all build.rs code so that the rest of the binary can make use of other abstractions (e.g. Index for Piece)
const NETWORK_NAME: &str = "v2-gen1";

pub mod common {
    pub mod square {
        #![allow(dead_code)]
        include!("src/common/square.rs");
    }
    pub mod side {
        #![allow(dead_code)]
        include!("src/common/side.rs");
    }
    pub mod constants {
        #![allow(dead_code)]
        include!("src/common/constants.rs");
    }
    pub mod random {
        #![allow(dead_code)]
        include!("src/common/random.rs");
    }
}

// TODO: Clean? Circular Dependency if included
pub mod bitboard {
    #![allow(dead_code)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Bitboard(pub u64);

    impl Bitboard {
        #[inline]
        pub fn get_bit(&self, square: usize) -> u8 {
            ((self.0 >> square) & 1) as u8
        }

        #[inline]
        pub fn set_bit(&mut self, square: usize) {
            self.0 |= 1u64 << square;
        }

        #[inline]
        pub fn clear_bit(&mut self, square: usize) {
            self.0 &= !(1u64 << square);
        }

        #[inline]
        pub fn get_lsb(&self) -> u32 {
            self.0.trailing_zeros()
        }
    }

    pub mod attacks {
        #![allow(dead_code)]
        include!("src/bitboard/attacks.rs");
    }

    pub mod magics {
        #![allow(dead_code)]
        include!("src/bitboard/magics.rs");
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src/bitboard/attacks.rs");
    println!("cargo:rerun-if-changed=src/bitboard/magics.rs");
    println!("cargo:rerun-if-changed=src/common/square.rs");
    println!("cargo:rerun-if-changed=src/common/side.rs");
    println!("cargo:rerun-if-changed=src/common/constants.rs");
    println!("cargo:rerun-if-changed=src/common/random.rs");
    println!("cargo:rerun-if-changed=resources/nnue.bin");

    use bitboard::Bitboard;
    use bitboard::attacks::{
        get_bishop_attacks, get_king_attacks, get_knight_attacks, get_pawn_attacks,
        get_rook_attacks,
    };
    use bitboard::magics::{
        BISHOP_MAGICS, ROOK_MAGICS, get_bishop_mask, get_magic_index, get_occupancy_mapping,
        get_rook_mask,
    };
    use common::side::Side;
    use common::square::Square;
    use std::env::var;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    // 1. Compute basic arrays
    let mut bishop_mask_bits = [0u32; 64];
    for (sq, item) in bishop_mask_bits.iter_mut().enumerate() {
        *item = get_bishop_mask(Square::from(sq)).0.count_ones();
    }

    let mut rook_mask_bits = [0u32; 64];
    for (sq, item) in rook_mask_bits.iter_mut().enumerate() {
        *item = get_rook_mask(Square::from(sq)).0.count_ones();
    }

    let mut bishop_masks = [0u64; 64];
    for (sq, item) in bishop_masks.iter_mut().enumerate() {
        *item = get_bishop_mask(Square::from(sq)).0;
    }

    let mut rook_masks = [0u64; 64];
    for (sq, item) in rook_masks.iter_mut().enumerate() {
        *item = get_rook_mask(Square::from(sq)).0;
    }

    let mut pawn_attacks = [[0u64; 64]; 2];
    for (sq, item) in pawn_attacks[Side::White as usize].iter_mut().enumerate() {
        *item = get_pawn_attacks(Square::from(sq), Side::White).0;
    }
    for (sq, item) in pawn_attacks[Side::Black as usize].iter_mut().enumerate() {
        *item = get_pawn_attacks(Square::from(sq), Side::Black).0;
    }

    let mut knight_attacks = [0u64; 64];
    for (sq, item) in knight_attacks.iter_mut().enumerate() {
        *item = get_knight_attacks(Square::from(sq)).0;
    }

    let mut king_attacks = [0u64; 64];
    for (sq, item) in king_attacks.iter_mut().enumerate() {
        *item = get_king_attacks(Square::from(sq)).0;
    }

    // 2. Compute sliding attack tables
    let mut bishop_attacks = vec![[0u64; 512]; 64];
    for sq in 0..64 {
        let mask = get_bishop_mask(Square::from(sq));
        let bits = bishop_mask_bits[sq];
        let index_count = 1usize << bits;

        for index in 0..index_count {
            let occupancy = get_occupancy_mapping(index, bits as i32, mask);
            let magic_index = get_magic_index(Bitboard(occupancy.0), BISHOP_MAGICS[sq], bits);
            bishop_attacks[sq][magic_index] =
                get_bishop_attacks(Square::from(sq), Bitboard(occupancy.0)).0;
        }
    }

    let mut rook_attacks = vec![[0u64; 4096]; 64];
    for sq in 0..64 {
        let mask = get_rook_mask(Square::from(sq));
        let bits = rook_mask_bits[sq];
        let index_count = 1usize << bits;

        for index in 0..index_count {
            let occupancy = get_occupancy_mapping(index, bits as i32, mask);
            let magic_index = get_magic_index(Bitboard(occupancy.0), ROOK_MAGICS[sq], bits);
            rook_attacks[sq][magic_index] =
                get_rook_attacks(Square::from(sq), Bitboard(occupancy.0)).0;
        }
    }

    // 3. Write lookups_gen.rs to OUT_DIR
    let out_dir = var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lookups_gen.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "// Generated by build.rs. Do not edit.\n").unwrap();

    macro_rules! write_table {
        ($name:expr, $type:expr, $items:expr) => {
            writeln!(f, "pub static {}: {} = {:?};", $name, $type, $items).unwrap();
        };
    }

    write_table!("BISHOP_MASK_BITS", "[u32; 64]", bishop_mask_bits);
    write_table!("ROOK_MASK_BITS", "[u32; 64]", rook_mask_bits);
    write_table!("BISHOP_MASKS", "[u64; 64]", bishop_masks);
    write_table!("ROOK_MASKS", "[u64; 64]", rook_masks);
    write_table!("PAWN_ATTACKS", "[[u64; 64]; 2]", pawn_attacks);
    write_table!("KNIGHT_ATTACKS", "[u64; 64]", knight_attacks);
    write_table!("KING_ATTACKS", "[u64; 64]", king_attacks);
    write_table!("BISHOP_ATTACKS", "[[u64; 512]; 64]", bishop_attacks);
    write_table!("ROOK_ATTACKS", "[[u64; 4096]; 64]", rook_attacks);
    download_nnue_if_needed();
}

fn download_nnue_if_needed() {
    use std::fs::{create_dir_all, metadata, write};
    use std::path::Path;
    use std::process::Command;

    let dest_path = Path::new("resources/nnue.bin");

    let acc_size = 64;
    let input_size = 768;
    let struct_size: usize = (input_size * acc_size + acc_size + acc_size * 2 + 1) * 2;
    // Align up to 64 bytes
    let expected_size = struct_size.div_ceil(64) * 64;

    let needs_recreate = if !dest_path.exists() {
        true
    } else {
        match metadata(dest_path) {
            Ok(meta) => meta.len() != expected_size as u64,
            Err(_) => true,
        }
    };

    if needs_recreate {
        create_dir_all("resources").unwrap();
        println!(
            "cargo:warning=Downloading NNUE weights from GitHub (znxftw/rudim-networks {})...",
            NETWORK_NAME
        );

        let url = format!(
            "https://github.com/znxftw/rudim-networks/releases/download/{}/nnue.bin",
            NETWORK_NAME
        );
        let status = Command::new("curl")
            .arg("-L")
            .arg("-s")
            .arg("-f")
            .arg("-o")
            .arg(dest_path)
            .arg(url)
            .status();

        let success = match status {
            Ok(exit_status) => {
                if exit_status.success() {
                    // Validate downloaded file size
                    match metadata(dest_path) {
                        Ok(meta) => meta.len() == expected_size as u64,
                        Err(_) => false,
                    }
                } else {
                    false
                }
            }
            Err(_) => false,
        };

        if !success {
            println!(
                "cargo:warning=Failed to download correct weights from GitHub. Generating zero-initialized weights instead..."
            );
            write(dest_path, vec![0u8; expected_size]).unwrap();
        } else {
            println!("cargo:warning=Successfully downloaded NNUE weights.");
        }
    }
}
