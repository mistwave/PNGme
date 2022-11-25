use std::str;
use std::{fmt::Display, str::FromStr};

use std::error;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bits: [u8; 4],
}

#[derive(Debug, Clone)]
struct InvalidChunkType {}

impl InvalidChunkType {
    fn new() -> InvalidChunkType {
        InvalidChunkType {}
    }
}

impl Display for InvalidChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidChunkType")
    }
}

impl error::Error for InvalidChunkType {}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bits
    }
    pub fn is_valid(&self) -> bool {
        let chars = self.bits.map(|b| char::from_u32(b as u32).unwrap());

        let critical_valid =
            (chars[0].is_ascii_uppercase() && self.is_critical()) ||
                (!chars[0].is_ascii_uppercase() && !self.is_critical());

        let public_valid =
            (chars[1].is_ascii_uppercase() && self.is_public()) ||
                (!chars[1].is_ascii_uppercase() && !self.is_public());

        let reversed_bit_valid =
            chars[2].is_ascii_uppercase() || self.is_reserved_bit_valid();

        let safe_to_copy_valid =
            (chars[3].is_ascii_lowercase() && self.is_safe_to_copy()) ||
                (!chars[3].is_ascii_lowercase() && !self.is_safe_to_copy());

        critical_valid && public_valid && reversed_bit_valid && safe_to_copy_valid
    }
    fn is_critical(&self) -> bool {
        five_bit(self.bits[0]) == 0
    }
    fn is_public(&self) -> bool {
        five_bit(self.bits[1]) == 0
    }
    fn is_reserved_bit_valid(&self) -> bool {
        five_bit(self.bits[2]) == 0
    }
    fn is_safe_to_copy(&self) -> bool {
        five_bit(self.bits[3]) == 1
    }
}

fn five_bit(by: u8) -> u8 {
    (by >> 5) & 1
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { bits: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits: [u8; 4] = s.as_bytes().try_into().unwrap();
        let allalpha = bits.map(|b| char::from_u32(b as u32).unwrap())
            .iter()
            .all(|ch| ch.is_alphabetic());
        if allalpha {
            Ok(ChunkType { bits })
        } else {
            Err(Box::new(InvalidChunkType::new()))
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = str::from_utf8(&self.bits).unwrap_or("unknown chunk type");
        write!(f, "{}", s)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
