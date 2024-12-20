use crate::binpack_error::{BinpackError, Result};

#[derive(Debug)]
pub struct BitReader<'a> {
    movetext: &'a [u8],
    read_bits_left: usize,
    read_offset: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(movetext: &'a [u8]) -> Self {
        Self {
            movetext,
            read_bits_left: 8,
            read_offset: 0,
        }
    }

    pub fn extract_bits_le8(&mut self, count: usize) -> Result<u8> {
        if count == 0 {
            return Ok(0);
        }

        if self.read_bits_left == 0 {
            self.read_offset += 1;
            self.read_bits_left = 8;
        }

        if self.read_offset >= self.movetext.len() {
            return Err(BinpackError::InvalidFormat("Unexpected end of data".into()));
        }

        let byte = self.movetext[self.read_offset] << (8 - self.read_bits_left);
        let mut bits = byte >> (8 - count);

        if count > self.read_bits_left {
            let spill_count = count - self.read_bits_left;

            bits |= self.movetext[self.read_offset + 1] >> (8 - spill_count);
            self.read_bits_left += 8;
            self.read_offset += 1;
        }

        self.read_bits_left -= count;
        Ok(bits)
    }

    pub fn extract_vle16(&mut self, block_size: usize) -> Result<u16> {
        let mask = (1 << block_size) - 1;
        let mut v = 0u16;
        let mut offset = 0;

        loop {
            let block = self.extract_bits_le8(block_size + 1)? as u16;
            v |= (block & mask) << offset;
            if (block >> block_size) == 0 {
                break;
            }
            offset += block_size;
        }

        Ok(v)
    }

    pub fn num_read_bytes(&self) -> usize {
        self.read_offset + (self.read_bits_left != 8) as usize
    }
}
