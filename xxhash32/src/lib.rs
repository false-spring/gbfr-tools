use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

/*
 * Graciously ported from Nenkai's reverse engineering work:
 * https://github.com/Nenkai/GBFRDataTools/blob/db3960b2e3643f012091581b4358ce64a916e5cd/GBFRDataTools.Hashing/XXHash32Custom.cs
 */
const PRIME32_1: u32 = 0x9e3779b1;
const PRIME32_2: u32 = 0x85EBCA77;
const PRIME32_3: u32 = 0xC2B2AE3D;
const PRIME32_4: u32 = 0x27D4EB2F;
const PRIME32_5: u32 = 0x165667B1;

#[inline(always)]
fn xxhash32_rotl(x: u32, r: u32) -> u32 {
    (x << r) | (x >> (32 - r))
}

#[inline(always)]
fn xxhash32_round(seed: u32, input: u32) -> u32 {
    xxhash32_rotl(seed + input * PRIME32_2, 13) * PRIME32_1
}

pub fn xxhash32_custom(input: &str) -> u32 {
    let buffer_len = input.len() as u32;
    let mut cursor = Cursor::new(input.as_bytes());

    let mut h32: u32 = 0x178A54A4;

    if input.len() > 16 {
        let mut v1 = 0x2557311B;
        let mut v2 = 0x871FB76A;
        let mut v3 = 0x0133ECF3;
        let mut v4 = 0x62FC7342;

        while buffer_len - cursor.position() as u32 >= 16 {
            v1 = xxhash32_round(v1, cursor.read_u32::<LittleEndian>().unwrap());
            v2 = xxhash32_round(v2, cursor.read_u32::<LittleEndian>().unwrap());
            v3 = xxhash32_round(v3, cursor.read_u32::<LittleEndian>().unwrap());
            v4 = xxhash32_round(v4, cursor.read_u32::<LittleEndian>().unwrap());
        }

        h32 = xxhash32_rotl(v1, 1)
            + xxhash32_rotl(v2, 7)
            + xxhash32_rotl(v3, 12)
            + xxhash32_rotl(v4, 18);
    }

    h32 += input.len() as u32;

    while buffer_len - cursor.position() as u32 >= 4 {
        h32 = h32.wrapping_add(
            cursor
                .read_u32::<LittleEndian>()
                .unwrap()
                .wrapping_mul(PRIME32_3),
        );
        h32 = xxhash32_rotl(h32, 17).wrapping_mul(PRIME32_4);
    }

    while buffer_len - cursor.position() as u32 > 0 {
        h32 = h32.wrapping_add((cursor.read_u8().unwrap() as u32).wrapping_mul(PRIME32_5));
        h32 = xxhash32_rotl(h32, 11).wrapping_mul(PRIME32_1);
    }

    h32 ^= h32 >> 15;
    h32 = h32.wrapping_mul(PRIME32_2);
    h32 ^= h32 >> 13;
    h32 = h32.wrapping_mul(PRIME32_3);
    h32 ^= h32 >> 16;
    h32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_correctly() {
        assert_eq!(0x887AE0B0, xxhash32_custom(""));
        assert_eq!(0x9AD6310D, xxhash32_custom("hello"));
    }
}
