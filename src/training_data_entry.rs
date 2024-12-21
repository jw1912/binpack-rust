use crate::{
    arithmetic::unsigned_to_signed,
    chess::{position::Position, r#move::Move},
    compressed_move::CompressedMove,
    compressed_position::CompressedPosition,
};

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

    pub fn read_u16_be(&self, offset: usize) -> u16 {
        ((self.data[offset] as u16) << 8) | (self.data[offset + 1] as u16)
    }

    pub fn unpack_entry(&self) -> TrainingDataEntry {
        let mut offset = 0;

        // Read and decompress position
        let compressed_pos = CompressedPosition::read_from_big_endian(&self.data[offset..]);
        let mut pos = compressed_pos.decompress();
        offset += std::mem::size_of::<CompressedPosition>();

        // Read and decompress move
        let compressed_move = CompressedMove::read_from_big_endian(&self.data[offset..]);
        let mv = compressed_move.decompress();
        offset += std::mem::size_of::<CompressedMove>();

        // Read score
        let score = unsigned_to_signed(self.read_u16_be(offset));
        offset += 2;

        // Read ply and result (packed together)
        let pr = self.read_u16_be(offset);
        let ply = pr & 0x3FFF;
        let result = unsigned_to_signed(pr >> 14);
        offset += 2;

        // Set position's ply
        pos.set_ply(ply);

        // Read and set rule50 counter
        pos.set_rule50_counter(self.read_u16_be(offset));

        TrainingDataEntry {
            pos,
            mv,
            score,
            ply,
            result,
        }
    }
}
