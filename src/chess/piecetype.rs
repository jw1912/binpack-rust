#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

impl PieceType {
    #[inline(always)]
    pub const fn from_ordinal(value: u8) -> Self {
        debug_assert!(value < 7);
        unsafe { std::mem::transmute(value) }
    }

    #[inline(always)]
    pub const fn ordinal(&self) -> u8 {
        *self as u8
    }
}
