use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom};

use crate::binpack_error::{BinpackError, Result};

// use crate::reader::move_score_list_reader::BinpackError;

const HEADER_SIZE: usize = 8;

const KI_B: u32 = 1024;
const MI_B: u32 = 1024 * KI_B;

const MAX_CHUNK_SIZE: u32 = 100 * MI_B;

#[derive(Debug)]
struct Header {
    chunk_size: u32,
}

#[derive(Debug)]
pub struct CompressedTrainingDataFile {
    file: File,
}

impl CompressedTrainingDataFile {
    pub fn new(path: &str, append: bool) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(append)
            .open(&path)?;

        Ok(Self { file })
    }

    // pub fn append(&mut self, data: &[u8]) -> io::Result<()> {
    //     let header = Header {
    //         chunk_size: data.len() as u32,
    //     };
    //     self.write_chunk_header(&header)?;
    //     self.file.write_all(data)?;
    //     Ok(())
    // }

    pub fn has_next_chunk(&mut self) -> bool {
        match self.file.stream_position() {
            Ok(pos) => {
                if let Ok(len) = self.file.seek(SeekFrom::End(0)) {
                    if self.file.seek(SeekFrom::Start(pos)).is_ok() {
                        return pos < len;
                    }
                }
            }
            Err(_) => {}
        }
        false
    }

    pub fn read_next_chunk(&mut self) -> Result<Vec<u8>> {
        let header = self.read_chunk_header()?;

        let mut data = vec![0u8; (header.chunk_size) as usize];

        self.file.read_exact(&mut data)?;
        Ok(data)
    }

    // fn write_chunk_header(&mut self, header: &Header) -> io::Result<()> {
    //     let mut buf = [0u8; HEADER_SIZE];
    //     buf[0] = b'B';
    //     buf[1] = b'I';
    //     buf[2] = b'N';
    //     buf[3] = b'P';
    //     buf[4] = (header.chunk_size & 0xFF) as u8;
    //     buf[5] = ((header.chunk_size >> 8) & 0xFF) as u8;
    //     buf[6] = ((header.chunk_size >> 16) & 0xFF) as u8;
    //     buf[7] = ((header.chunk_size >> 24) & 0xFF) as u8;
    //     self.file.write_all(&buf)
    // }

    fn read_chunk_header(&mut self) -> Result<Header> {
        let mut buf = [0u8; HEADER_SIZE];
        self.file.read_exact(&mut buf)?;

        if &buf[0..4] != b"BINP" {
            return Err(BinpackError::InvalidMagic).into();
        }

        let chunk_size = u32::from_le_bytes(buf[4..8].try_into().unwrap());

        if chunk_size > MAX_CHUNK_SIZE {
            return Err(BinpackError::InvalidFormat(
                "Chunk size larger than supported. Malformed file?".to_string(),
            )
            .into());
        }

        Ok(Header { chunk_size })
    }
}
