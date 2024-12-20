use std::io::{self};
use thiserror::Error;

use crate::arithmetic::unsigned_to_signed;
use crate::binpack_error::BinpackError;
use crate::chess::position::Position;
use crate::chess::r#move::Move;
use crate::compressed_move::CompressedMove;
use crate::packed_position::PackedPosition;
use crate::training_data_file::CompressedTrainingDataFile;

use super::move_score_list_reader::PackedMoveScoreListReader;

const SUGGESTED_CHUNK_SIZE: usize = 8192;

#[derive(Debug, Error)]
pub enum CompressedReaderError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
    #[error("End of file reached")]
    EndOfFile,
    #[error("Binpack error: {0}")]
    BinpackError(#[from] BinpackError),
}

pub type Result<T> = std::result::Result<T, CompressedReaderError>;

#[derive(Debug)]
pub struct CompressedTrainingDataEntryReader {
    chunk: Vec<u8>,
    movelist_reader: Option<OwnedMoveScoreListReader>,
    input_file: CompressedTrainingDataFile,
    offset: usize,
    is_end: bool,
}

#[derive(Debug)]
struct OwnedMoveScoreListReader {
    reader: PackedMoveScoreListReader<'static>,
}

impl CompressedTrainingDataEntryReader {
    pub fn new(path: &str) -> Result<Self> {
        let chunk = Vec::with_capacity(SUGGESTED_CHUNK_SIZE);

        let mut reader = Self {
            chunk,
            movelist_reader: None,
            input_file: CompressedTrainingDataFile::new(path, false)?,
            offset: 0,
            is_end: false,
        };

        if !reader.input_file.has_next_chunk() {
            reader.is_end = true;
            return Err(CompressedReaderError::EndOfFile);
        } else {
            reader.chunk = match reader.input_file.read_next_chunk() {
                Ok(chunk) => chunk,
                Err(e) => return Err(CompressedReaderError::BinpackError(e)),
            };
        }

        Ok(reader)
    }

    pub fn has_next(&self) -> bool {
        !self.is_end
    }

    pub fn next(&mut self) -> TrainingDataEntry {
        if let Some(ref mut reader) = self.movelist_reader {
            let entry = reader.reader.next_entry();

            if !reader.reader.has_next() {
                self.offset += reader.reader.num_read_bytes();
                self.movelist_reader = None;
                self.fetch_next_chunk_if_needed();
            }

            return entry;
        }

        // Read packed entry
        let mut packed = PackedTrainingDataEntry::default();

        debug_assert!(
            self.offset + std::mem::size_of::<PackedTrainingDataEntry>() <= self.chunk.len()
        );

        packed.copy_from_slice(
            &self.chunk[self.offset..self.offset + std::mem::size_of::<PackedTrainingDataEntry>()],
        );

        self.offset += std::mem::size_of::<PackedTrainingDataEntry>();

        // Read number of plies
        let num_plies =
            ((self.chunk[self.offset] as u16) << 8) | (self.chunk[self.offset + 1] as u16);
        self.offset += 2;

        let entry = unpack_entry(&packed);

        if num_plies > 0 {
            let chunk_ref = &self.chunk[self.offset..];

            // should be safe lol, someone rewrite this please
            let reader = unsafe {
                std::mem::transmute::<
                    PackedMoveScoreListReader<'_>,
                    PackedMoveScoreListReader<'static>,
                >(PackedMoveScoreListReader::new(
                    entry,
                    chunk_ref,
                    num_plies,
                ))
            };

            self.movelist_reader = Some(OwnedMoveScoreListReader { reader });
        } else {
            self.fetch_next_chunk_if_needed();
        }

        entry
    }

    fn fetch_next_chunk_if_needed(&mut self) {
        if self.offset + std::mem::size_of::<PackedTrainingDataEntry>() + 2 > self.chunk.len() {
            if self.input_file.has_next_chunk() {
                let chunk = self.input_file.read_next_chunk().unwrap();
                self.chunk = chunk;
                self.offset = 0;
            } else {
                self.is_end = true;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TrainingDataEntry {
    pub pos: Position,
    pub mv: Move,
    pub score: i16,
    pub ply: u16,
    pub result: i16,
}

#[derive(Debug, Default, Clone)]
pub struct PackedTrainingDataEntry {
    pub data: [u8; 32],
}

impl PackedTrainingDataEntry {
    pub fn copy_from_slice(&mut self, slice: &[u8]) {
        self.data.copy_from_slice(slice);
    }

    fn read_u16_be(&self, offset: usize) -> u16 {
        ((self.data[offset] as u16) << 8) | (self.data[offset + 1] as u16)
    }
}

fn unpack_entry(packed: &PackedTrainingDataEntry) -> TrainingDataEntry {
    let mut offset = 0;

    // Read and decompress position
    let compressed_pos = PackedPosition::read_from_big_endian(&packed.data[offset..]);
    let mut pos = compressed_pos.decompress();
    offset += std::mem::size_of::<PackedPosition>();

    // Read and decompress move
    let compressed_move = CompressedMove::read_from_big_endian(&packed.data[offset..]);
    let mv = compressed_move.decompress();
    offset += std::mem::size_of::<CompressedMove>();

    // Read score
    let score = unsigned_to_signed(packed.read_u16_be(offset));
    offset += 2;

    // Read ply and result (packed together)
    let pr = packed.read_u16_be(offset);
    let ply = pr & 0x3FFF;
    let result = unsigned_to_signed(pr >> 14);
    offset += 2;

    // Set position's ply
    pos.set_ply(ply);

    // Read and set rule50 counter
    pos.set_rule50_counter(packed.read_u16_be(offset));

    TrainingDataEntry {
        pos,
        mv,
        score,
        ply,
        result,
    }
}

impl Drop for CompressedTrainingDataEntryReader {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {}
