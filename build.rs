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

    // 1. Compute basic arrays
    let mut bishop_mask_bits = [0u32; 64];
    for sq in 0..64 {
        bishop_mask_bits[sq] = bitboard::magics::get_bishop_mask(common::square::Square::from(sq))
            .0
            .count_ones();
    }

    let mut rook_mask_bits = [0u32; 64];
    for sq in 0..64 {
        rook_mask_bits[sq] = bitboard::magics::get_rook_mask(common::square::Square::from(sq))
            .0
            .count_ones();
    }

    let mut bishop_masks = [0u64; 64];
    for sq in 0..64 {
        bishop_masks[sq] = bitboard::magics::get_bishop_mask(common::square::Square::from(sq)).0;
    }

    let mut rook_masks = [0u64; 64];
    for sq in 0..64 {
        rook_masks[sq] = bitboard::magics::get_rook_mask(common::square::Square::from(sq)).0;
    }

    let mut pawn_attacks = [[0u64; 64]; 2];
    for sq in 0..64 {
        pawn_attacks[common::side::Side::White as usize][sq] = bitboard::attacks::get_pawn_attacks(
            common::square::Square::from(sq),
            common::side::Side::White,
        )
        .0;
        pawn_attacks[common::side::Side::Black as usize][sq] = bitboard::attacks::get_pawn_attacks(
            common::square::Square::from(sq),
            common::side::Side::Black,
        )
        .0;
    }

    let mut knight_attacks = [0u64; 64];
    for sq in 0..64 {
        knight_attacks[sq] =
            bitboard::attacks::get_knight_attacks(common::square::Square::from(sq)).0;
    }

    let mut king_attacks = [0u64; 64];
    for sq in 0..64 {
        king_attacks[sq] = bitboard::attacks::get_king_attacks(common::square::Square::from(sq)).0;
    }

    // 2. Compute sliding attack tables
    let mut bishop_attacks = [[0u64; 512]; 64];
    for sq in 0..64 {
        let mask = bitboard::magics::get_bishop_mask(common::square::Square::from(sq));
        let bits = bishop_mask_bits[sq];
        let index_count = 1usize << bits;

        for index in 0..index_count {
            let occupancy = bitboard::magics::get_occupancy_mapping(index, bits as i32, mask);
            let magic_index = occupancy
                .0
                .wrapping_mul(bitboard::magics::BISHOP_MAGICS[sq])
                .wrapping_shr(64 - bits) as usize;
            bishop_attacks[sq][magic_index] = bitboard::attacks::get_bishop_attacks(
                common::square::Square::from(sq),
                bitboard::Bitboard(occupancy.0),
            )
            .0;
        }
    }

    let mut rook_attacks = [[0u64; 4096]; 64];
    for sq in 0..64 {
        let mask = bitboard::magics::get_rook_mask(common::square::Square::from(sq));
        let bits = rook_mask_bits[sq];
        let index_count = 1usize << bits;

        for index in 0..index_count {
            let occupancy = bitboard::magics::get_occupancy_mapping(index, bits as i32, mask);
            let magic_index = occupancy
                .0
                .wrapping_mul(bitboard::magics::ROOK_MAGICS[sq])
                .wrapping_shr(64 - bits) as usize;
            rook_attacks[sq][magic_index] = bitboard::attacks::get_rook_attacks(
                common::square::Square::from(sq),
                bitboard::Bitboard(occupancy.0),
            )
            .0;
        }
    }

    // 3. Write lookups_gen.rs to OUT_DIR
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("lookups_gen.rs");
    let mut f = std::fs::File::create(&dest_path).unwrap();

    use std::io::Write;
    writeln!(f, "// Generated by build.rs. Do not edit.\n").unwrap();

    write!(f, "pub static BISHOP_MASK_BITS: [u32; 64] = [\n").unwrap();
    for &bits in &bishop_mask_bits {
        write!(f, "    {},\n", bits).unwrap();
    }
    write!(f, "];\n\n").unwrap();

    write!(f, "pub static ROOK_MASK_BITS: [u32; 64] = [\n").unwrap();
    for &bits in &rook_mask_bits {
        write!(f, "    {},\n", bits).unwrap();
    }
    write!(f, "];\n\n").unwrap();

    write!(f, "pub static BISHOP_MASKS: [u64; 64] = [\n").unwrap();
    for &mask in &bishop_masks {
        write!(f, "    {},\n", mask).unwrap();
    }
    write!(f, "];\n\n").unwrap();

    write!(f, "pub static ROOK_MASKS: [u64; 64] = [\n").unwrap();
    for &mask in &rook_masks {
        write!(f, "    {},\n", mask).unwrap();
    }
    write!(f, "];\n\n").unwrap();

    write!(f, "pub static PAWN_ATTACKS: [[u64; 64]; 2] = [\n").unwrap();
    for side in 0..2 {
        write!(f, "    [\n").unwrap();
        for &atk in &pawn_attacks[side] {
            write!(f, "        {},\n", atk).unwrap();
        }
        write!(f, "    ],\n").unwrap();
    }
    write!(f, "];\n\n").unwrap();

    write!(f, "pub static KNIGHT_ATTACKS: [u64; 64] = [\n").unwrap();
    for &atk in &knight_attacks {
        write!(f, "    {},\n", atk).unwrap();
    }
    write!(f, "];\n\n").unwrap();

    write!(f, "pub static KING_ATTACKS: [u64; 64] = [\n").unwrap();
    for &atk in &king_attacks {
        write!(f, "    {},\n", atk).unwrap();
    }
    write!(f, "];\n\n").unwrap();

    write!(f, "pub static BISHOP_ATTACKS: [[u64; 512]; 64] = [\n").unwrap();
    for sq in 0..64 {
        write!(f, "    [\n").unwrap();
        for index in 0..512 {
            write!(f, "        {},\n", bishop_attacks[sq][index]).unwrap();
        }
        write!(f, "    ],\n").unwrap();
    }
    write!(f, "];\n\n").unwrap();

    write!(f, "pub static ROOK_ATTACKS: [[u64; 4096]; 64] = [\n").unwrap();
    for sq in 0..64 {
        write!(f, "    [\n").unwrap();
        for index in 0..4096 {
            write!(f, "        {},\n", rook_attacks[sq][index]).unwrap();
        }
        write!(f, "    ],\n").unwrap();
    }
    write!(f, "];\n").unwrap();
}
