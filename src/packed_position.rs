use crate::{
    chess::bitboard::Bitboard,
    chess::castling_rights::CastlingRights,
    chess::color::Color,
    chess::coords::{FlatSquareOffset, Rank, Square},
    chess::piece::Piece,
    chess::position::Position,
};

#[derive(Debug, Clone, Copy)]
pub struct PackedPosition {
    occupied: Bitboard,
    packed_state: [u8; 16],
}

impl PackedPosition {
    pub fn read_from_big_endian(data: &[u8]) -> Self {
        let occupied = Bitboard::new(
            ((data[0] as u64) << 56)
                | ((data[1] as u64) << 48)
                | ((data[2] as u64) << 40)
                | ((data[3] as u64) << 32)
                | ((data[4] as u64) << 24)
                | ((data[5] as u64) << 16)
                | ((data[6] as u64) << 8)
                | (data[7] as u64),
        );

        let mut packed_state = [0u8; 16];
        packed_state.copy_from_slice(&data[8..24]);

        Self {
            occupied,
            packed_state,
        }
    }

    pub fn decompress(&self) -> Position {
        let mut pos = Position::new();
        pos.set_castling_rights(CastlingRights::NONE);

        let mut decompress_piece = |sq: Square, nibble: u8| {
            match nibble {
                0..=11 => {
                    pos.place(Piece::from_id(nibble as i32), sq);
                }
                12 => {
                    let rank = sq.rank();
                    if rank == Rank::FOURTH {
                        pos.place(Piece::WhitePawn, sq);
                        pos.set_ep_square_unchecked(sq + FlatSquareOffset::new(0, -1));
                    } else {
                        // rank == Rank::FIFTH
                        pos.place(Piece::BlackPawn, sq);
                        pos.set_ep_square_unchecked(sq + FlatSquareOffset::new(0, 1));
                    }
                }
                13 => {
                    pos.place(Piece::WhiteRook, sq);
                    if sq == Square::A1 {
                        pos.add_castling_rights(CastlingRights::WHITE_QUEEN_SIDE);
                    } else {
                        // sq == Square::H1
                        pos.add_castling_rights(CastlingRights::WHITE_KING_SIDE);
                    }
                }
                14 => {
                    pos.place(Piece::BlackRook, sq);
                    if sq == Square::A8 {
                        pos.add_castling_rights(CastlingRights::BLACK_QUEEN_SIDE);
                    } else {
                        // sq == Square::H8
                        pos.add_castling_rights(CastlingRights::BLACK_KING_SIDE);
                    }
                }
                15 => {
                    pos.place(Piece::BlackKing, sq);
                    pos.set_side_to_move(Color::Black);
                }
                _ => unreachable!(),
            }
        };

        let mut squares_iter = self.occupied.iter();
        for (_, chunk) in self.packed_state.iter().enumerate() {
            if let Some(sq) = squares_iter.next() {
                decompress_piece(sq, chunk & 0xF);
            } else {
                break;
            }

            if let Some(sq) = squares_iter.next() {
                decompress_piece(sq, chunk >> 4);
            } else {
                break;
            }
        }

        pos
    }
}
