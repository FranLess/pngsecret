use crc::Crc;
use std::{
    fmt::Display,
    io::{BufReader, Read},
    process::{ExitCode, Termination},
};

use crate::{chunk_type::ChunkType, Error};
#[derive(Debug, Clone)]
pub struct Chunk {
    data_length: [u8; 4],
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: [u8; 4],
}
impl Termination for Chunk {
    fn report(self) -> std::process::ExitCode {
        ExitCode::SUCCESS
    }
}
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: &[u8]) -> Self {
        let data_length = data.len() as u32;
        let crc = Chunk::calculate_crc(&chunk_type.bytes(), &data);
        Chunk {
            data_length: data_length.to_be_bytes(),
            chunk_type,
            data: data.to_vec(),
            crc,
        }
    }
    pub fn crc(&self) -> u32 {
        u32::from_be_bytes(self.crc)
    }
    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data_as_string(&self) -> Result<String, Error> {
        let string = String::from_utf8(self.data.iter().cloned().collect())?;
        Ok(string)
    }
    pub fn calculate_crc(chunk: &[u8], data: &[u8]) -> [u8; 4] {
        let data_check: Vec<u8> = chunk.iter().chain(data.iter()).copied().collect();
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        crc.checksum(&data_check).to_be_bytes()
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.data_length
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.iter())
            .cloned()
            .collect()
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Data length: {}\nChunk:{}\nData:{}\nCrc:{}",
            u32::from_be_bytes(self.data_length),
            self.chunk_type().to_string(),
            self.data_as_string().unwrap(),
            self.crc()
        )
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // preparing buffer and reader to read &[u8]
        let mut reader = BufReader::new(value);
        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        // reads the data length
        reader.read_exact(&mut buffer)?;
        let data_length = u32::from_be_bytes(buffer);

        // reads the chunk type
        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::new(buffer[0], buffer[1], buffer[2], buffer[3]);
        if !chunk_type.is_valid() {
            return Err(Error::from("Not a valid chunk"));
        }

        // reads the data
        let mut buffer = vec![0; data_length as usize];
        reader.read_exact(&mut buffer)?;
        let data = buffer;

        //reads the crc
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut buffer)?;
        let crc = buffer;
        if crc != Chunk::calculate_crc(&chunk_type.bytes(), &data) {
            return Err(Error::from("Not a valid crc"));
        }

        Ok(Chunk {
            data_length: data_length.to_be_bytes(),
            chunk_type,
            data,
            crc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    #[test]
    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data.as_ref());
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
