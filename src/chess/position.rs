use crate::chess::{
    attacks::Attacks,
    bitboard::Bitboard,
    castling_rights::{CastleType, CastlingRights},
    color::Color,
    coords::Square,
    piece::Piece,
    piecetype::PieceType,
    r#move::{Move, MoveType},
};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    bb: [u64; 6],       // Bitboards for each piece type (PNBRQK)
    bb_color: [u64; 2], // Bitboards for each color (White, Black)
    stm: Color,         // Side to move
    castling_rights: CastlingRights,
    halfm: u8,         // Halfmove clock for 50-move rule
    fullm: u16,        // Fullmove number
    enpassant: Square, // En passant target square
}

impl Position {
    pub fn new() -> Self {
        Self {
            bb: [0; 6],
            bb_color: [0; 2],
            stm: Color::White,
            castling_rights: CastlingRights::NONE,
            halfm: 0,
            fullm: 1,
            enpassant: Square::NONE,
        }
    }

    pub fn side_to_move(&self) -> Color {
        self.stm
    }

    pub fn occupied(&self) -> Bitboard {
        Bitboard::new(self.bb_color[0] | self.bb_color[1])
    }

    pub fn pieces_bb(&self, color: Color) -> Bitboard {
        let bb = Bitboard::new(self.bb_color[color as usize]);

        assert!(bb.count() > 0);

        bb
    }

    pub fn pieces_bb_color(&self, color: Color, pt: PieceType) -> Bitboard {
        let bb = Bitboard::new(self.bb_color[color as usize] & self.bb[pt.ordinal() as usize]);

        bb
    }

    pub fn piece_at(&self, square: Square) -> Piece {
        assert!(square != Square::NONE);
        let sq_bb = 1u64 << square.to_u32();

        for pt in 0..6 {
            if (self.bb[pt] & sq_bb) != 0 {
                let color = if (self.bb_color[0] & sq_bb) != 0 {
                    Color::White
                } else {
                    Color::Black
                };

                return Piece::new(PieceType::from_ordinal(pt as u8), color);
            }
        }

        Piece::none()
    }

    pub fn castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    pub fn ep_square(&self) -> Square {
        self.enpassant
    }

    pub fn do_move(&mut self, mv: Move) {
        assert!(self.bb[PieceType::King.ordinal() as usize].count_ones() == 2);

        let from = mv.from();
        let to = mv.to();
        let piece = self.piece_at(from);
        let captured = self.piece_at(to);
        let genuine_capture = captured != Piece::none() && mv.mtype() != MoveType::Castle;

        assert!(from != Square::NONE);
        assert!(to != Square::NONE);
        assert!(piece != Piece::none());

        // update state

        // Update halfmove clock
        if genuine_capture || piece.piece_type() == PieceType::Pawn {
            self.halfm = 0;
        } else {
            self.halfm += 1;
        }

        // Update fullmove number
        if self.stm == Color::Black {
            self.fullm += 1;
        }

        // clear piece from start
        self.toggle(self.stm, piece, from);

        // capture piece
        if genuine_capture {
            self.toggle(!self.stm, captured, to);
        }

        if mv.mtype() == MoveType::Promotion {
            let promotion = mv.promoted_piece();
            self.toggle(self.stm, promotion, to);
        } else if mv.mtype() != MoveType::Castle {
            self.toggle(self.stm, piece, to);
        }

        if mv.mtype() == MoveType::EnPassant {
            assert!(piece.piece_type() == PieceType::Pawn,);

            let captured_sq = Square::new(to.to_u32() ^ 8);
            self.toggle(!self.stm, self.piece_at(captured_sq), captured_sq);
        }

        if mv.mtype() == MoveType::Castle {
            if mv.castle_type() == CastleType::Short {
                let rook_to = if self.stm == Color::White {
                    Square::F1
                } else {
                    Square::F8
                };

                let king_to = if self.stm == Color::White {
                    Square::G1
                } else {
                    Square::G8
                };

                let rook = self.piece_at(to);

                self.toggle(self.stm, rook, to);
                self.toggle(self.stm, rook, rook_to);
                self.toggle(self.stm, piece, king_to);
            } else {
                let rook_to = if self.stm == Color::White {
                    Square::D1
                } else {
                    Square::D8
                };

                let king_to = if self.stm == Color::White {
                    Square::C1
                } else {
                    Square::C8
                };

                let rook = self.piece_at(to);

                self.toggle(self.stm, rook, to);
                self.toggle(self.stm, rook, rook_to);
                self.toggle(self.stm, piece, king_to);
            }
        }

        self.update_castling_rights(from, to);

        self.enpassant = Square::NONE;

        // Update en passant square
        if piece.piece_type() == PieceType::Pawn && (to.to_i32() - from.to_i32()).abs() == 16 {
            let ep = Square::new(to.to_u32() ^ 8);

            // check if enemy pawn can legally capture the pawn
            // if so set the ep square

            let ep_mask = Attacks::pawn(self.stm, ep);
            let enemy_mask = self.pieces_bb_color(!self.stm, PieceType::Pawn);

            // enemy pawn can pseudo capture the pawn
            if (ep_mask & enemy_mask).count() > 0 {
                // check if enemy pawn can legally capture the pawn
                // play the move

                // loop over enemy mask
                let mut enemy_mask = ep_mask & enemy_mask;

                while enemy_mask != Bitboard::new(0) {
                    let enemy_sq = Square::new(enemy_mask.to_u64().trailing_zeros() as u32);
                    enemy_mask = enemy_mask & Bitboard::new(enemy_mask.to_u64() - 1);

                    // move the enemy pawn
                    let enemy_pawn = self.piece_at(enemy_sq);
                    self.toggle(!self.stm, enemy_pawn, enemy_sq);
                    self.toggle(!self.stm, enemy_pawn, ep);

                    // remove our pawn
                    self.toggle(self.stm, piece, to);

                    // check if the side which made the move is in check
                    let is_checked = self.is_checked(!self.stm);

                    // undo the move

                    // move the enemy pawn
                    self.toggle(!self.stm, enemy_pawn, enemy_sq);
                    self.toggle(!self.stm, enemy_pawn, ep);

                    // place our pawn
                    self.toggle(self.stm, piece, to);

                    if !is_checked {
                        self.enpassant = ep;
                        break;
                    }
                }
            }
        }

        // Switch side to move
        self.stm = !self.stm;

        assert!(self.bb[PieceType::King.ordinal() as usize].count_ones() == 2);
    }

    fn update_castling_rights(&mut self, from: Square, to: Square) {
        // Remove castling rights if king or rook moves
        if from == Square::E1 || to == Square::E1 {
            self.castling_rights &= !CastlingRights::WHITE;
        }
        if from == Square::E8 || to == Square::E8 {
            self.castling_rights &= !CastlingRights::BLACK;
        }
        if from == Square::A1 || to == Square::A1 {
            self.castling_rights &= !CastlingRights::WHITE_QUEEN_SIDE;
        }
        if from == Square::H1 || to == Square::H1 {
            self.castling_rights &= !CastlingRights::WHITE_KING_SIDE;
        }
        if from == Square::A8 || to == Square::A8 {
            self.castling_rights &= !CastlingRights::BLACK_QUEEN_SIDE;
        }
        if from == Square::H8 || to == Square::H8 {
            self.castling_rights &= !CastlingRights::BLACK_KING_SIDE;
        }
    }

    pub fn set_castling_rights(&mut self, rights: CastlingRights) {
        self.castling_rights = rights;
    }

    pub fn set_ep_square_unchecked(&mut self, sq: Square) {
        self.enpassant = sq;
    }

    pub fn add_castling_rights(&mut self, rights: CastlingRights) {
        self.castling_rights |= rights;
    }

    pub fn set_side_to_move(&mut self, side: Color) {
        self.stm = side;
    }

    pub fn set_ply(&mut self, ply: u16) {
        self.fullm = (ply / 2) + 1;
    }

    pub fn ply(&self) -> u16 {
        ((self.fullm - 1) * 2) + (self.stm as u16)
    }

    pub fn set_rule50_counter(&mut self, counter: u16) {
        self.halfm = counter as u8;
    }

    pub fn place(&mut self, pc: Piece, sq: Square) {
        assert!(pc != Piece::none());
        assert!(sq != Square::NONE);

        let mask = 1u64 << (sq.to_u32());
        self.bb_color[pc.color() as usize] |= mask;
        self.bb[pc.piece_type().ordinal() as usize] |= mask;
    }

    fn toggle(&mut self, side: Color, pc: Piece, sq: Square) {
        assert!(pc != Piece::none());
        assert!(sq != Square::NONE);

        let mask = 1u64 << (sq.to_u32());
        self.bb_color[side as usize] ^= mask;
        self.bb[pc.piece_type().ordinal() as usize] ^= mask;
    }

    pub fn fen(&self) -> String {
        let mut fen = String::new();

        // pieces
        for rank in (0..8).rev() {
            let mut empty_squares = 0;

            for file in 0..8 {
                let square = Square::from_u32((rank * 8 + file) as u32);
                let piece = self.piece_at(square);

                if piece == Piece::none() {
                    empty_squares += 1;
                } else {
                    if empty_squares > 0 {
                        fen.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }

                    let mut c = match piece.piece_type() {
                        PieceType::Pawn => 'p',
                        PieceType::Knight => 'n',
                        PieceType::Bishop => 'b',
                        PieceType::Rook => 'r',
                        PieceType::Queen => 'q',
                        PieceType::King => 'k',
                        _ => panic!("Invalid piece type"),
                    };

                    if piece.color() == Color::White {
                        c = c.to_ascii_uppercase();
                    }
                    fen.push(c);
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        // color
        fen.push(' ');
        fen.push(if self.stm == Color::White { 'w' } else { 'b' });

        // castling
        fen.push(' ');
        let castling = self.castling_rights();
        if castling == CastlingRights::NONE {
            fen.push('-');
        } else {
            if castling.contains(CastlingRights::WHITE_KING_SIDE) {
                fen.push('K');
            }
            if castling.contains(CastlingRights::WHITE_QUEEN_SIDE) {
                fen.push('Q');
            }
            if castling.contains(CastlingRights::BLACK_KING_SIDE) {
                fen.push('k');
            }
            if castling.contains(CastlingRights::BLACK_QUEEN_SIDE) {
                fen.push('q');
            }
        }

        // ep square
        fen.push(' ');
        if self.enpassant == Square::NONE {
            fen.push('-');
        } else {
            // let file = (self.enpassant.to_u32() % 8) as u8;
            // let rank = (self.enpassant.to_u32() / 8) as u8;
            // fen.push((b'a' + file) as char);
            // fen.push((b'1' + rank) as char);
            fen.push_str(&self.enpassant.to_string());
        }

        // halfmove clock
        fen.push(' ');
        fen.push_str(&self.halfm.to_string());

        // fullmove number
        fen.push(' ');
        fen.push_str(&self.fullm.to_string());

        fen
    }

    pub fn is_attacked(&self, sq: Square, c: Color) -> bool {
        if (Attacks::pawn(!c, sq) & self.pieces_bb_color(c, PieceType::Pawn)).count() > 0 {
            return true;
        }

        if (Attacks::knight(sq) & self.pieces_bb_color(c, PieceType::Knight)).count() > 0 {
            return true;
        }

        if (Attacks::king(sq) & self.pieces_bb_color(c, PieceType::King)).count() > 0 {
            return true;
        }

        if (Attacks::bishop(sq, self.occupied())
            & (self.pieces_bb_color(c, PieceType::Bishop)
                | self.pieces_bb_color(c, PieceType::Queen)))
        .count()
            > 0
        {
            return true;
        }

        if (Attacks::rook(sq, self.occupied())
            & (self.pieces_bb_color(c, PieceType::Rook)
                | self.pieces_bb_color(c, PieceType::Queen)))
        .count()
            > 0
        {
            return true;
        }

        return false;
    }

    pub fn king_sq(&self, c: Color) -> Square {
        Square::new(
            self.pieces_bb_color(c, PieceType::King)
                .to_u64()
                .trailing_zeros() as u32,
        )
    }

    pub fn is_checked(&self, c: Color) -> bool {
        self.is_attacked(self.king_sq(c), !c)
    }
}
