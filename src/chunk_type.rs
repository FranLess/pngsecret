use std::{
    convert::TryFrom,
    fmt::{Debug, Display},
    str::FromStr,
};
#[derive(Clone)]
pub struct ChunkType {
    ancilliary: u8,
    private: u8,
    reserved: u8,
    safe_to_copy: u8,
}

impl ChunkType {
    pub fn new(ancilliary: u8, private: u8, reserved: u8, safe_to_copy: u8) -> Self {
        ChunkType {
            ancilliary,
            private,
            reserved,
            safe_to_copy,
        }
    }
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ancilliary,
            self.private,
            self.reserved,
            self.safe_to_copy,
        ]
    }
    pub fn is_critical(&self) -> bool {
        self.ancilliary.is_ascii_uppercase()
    }
    pub fn is_public(&self) -> bool {
        self.private.is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.reserved.is_ascii_uppercase()
    }
    pub fn is_valid(&self) -> bool {
        if ![
            self.ancilliary,
            self.private,
            self.reserved,
            self.safe_to_copy,
        ]
        .iter()
        .all(|i| i.is_ascii_alphabetic())
        {
            false
        } else if self.reserved.is_ascii_lowercase() {
            false
        } else {
            true
        }
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.safe_to_copy.is_ascii_lowercase()
    }
}
impl Debug for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkType")
            .field("ancilliary", &self.ancilliary)
            .field("private", &self.private)
            .field("reserved", &self.reserved)
            .field("safe_to_copy", &self.safe_to_copy)
            .finish()
    }
}
impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.ancilliary as char,
            self.private as char,
            self.reserved as char,
            self.safe_to_copy as char
        )
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        if self.bytes().len() == other.bytes().len() {
            let lenght = self.bytes().len();
            for i in 0..lenght {
                if !self.bytes()[i] == other.bytes()[i] {
                    return false;
                }
            }
        }
        true
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if !value.iter().all(|i| i.is_ascii_alphabetic()) {
            Err("All bytes of chunk should be a valid ascii")
        } else if value[2].is_ascii_lowercase() {
            Err("Byte 3 must be lowercase")
        } else {
            Ok(ChunkType {
                ancilliary: value[0],
                private: value[1],
                reserved: value[2],
                safe_to_copy: value[3],
            })
        }
    }
}
impl FromStr for ChunkType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err("String should be exactly 4 lenght")
        } else if !s.is_ascii() {
            Err("String should contain ascii chars only")
        } else {
            let bytes = s.as_bytes();
            for b in bytes {
                if !b.is_ascii_alphabetic() {
                    return Err("String should contain letters only");
                }
            }
            Ok(ChunkType {
                ancilliary: bytes[0],
                private: bytes[1],
                reserved: bytes[2],
                safe_to_copy: bytes[3],
            })
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected: [u8; 4] = [82, 117, 83, 116];
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
