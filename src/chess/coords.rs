use std::ops::{Add, Sub};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct FlatSquareOffset {
    value: i8,
}

impl FlatSquareOffset {
    pub const fn new(files: i32, ranks: i32) -> Self {
        // Assuming File has 8 variants (standard chess board)
        const FILE_CARDINALITY: i32 = 8;

        let offset = files + ranks * FILE_CARDINALITY;

        // Rust equivalent of the C++ assertions
        debug_assert!(offset >= i8::MIN as i32);
        debug_assert!(offset <= i8::MAX as i32);

        Self {
            value: offset as i8,
        }
    }

    // Default constructor equivalent
    pub const fn default() -> Self {
        Self { value: 0 }
    }

    // Negation operator
    pub const fn neg(&self) -> Self {
        Self { value: -self.value }
    }

    pub const fn to_i8(&self) -> i8 {
        self.value
    }
}

impl std::ops::Neg for FlatSquareOffset {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square {
    index: u32,
}

impl Square {
    pub const NONE: Self = Self { index: 64 };
    pub const A1: Self = Self { index: 0 };
    pub const C1: Self = Self { index: 2 };
    pub const D1: Self = Self { index: 3 };
    pub const E1: Self = Self { index: 4 };
    pub const F1: Self = Self { index: 5 };
    pub const G1: Self = Self { index: 6 };
    pub const H1: Self = Self { index: 7 };
    pub const A8: Self = Self { index: 56 };
    pub const C8: Self = Self { index: 58 };
    pub const D8: Self = Self { index: 59 };
    pub const E8: Self = Self { index: 60 };
    pub const F8: Self = Self { index: 61 };
    pub const G8: Self = Self { index: 62 };
    pub const H8: Self = Self { index: 63 };

    pub const fn new(index: u32) -> Self {
        assert!(index < 64);
        Self { index }
    }

    pub const fn to_u32(&self) -> u32 {
        self.index
    }

    pub const fn from_u32(index: u32) -> Self {
        Self { index }
    }

    pub const fn to_i32(&self) -> i32 {
        self.index as i32
    }

    pub const fn file(&self) -> File {
        File::new(self.index & 7)
    }

    pub const fn rank(&self) -> Rank {
        Rank::new(self.index >> 3)
    }

    pub fn offset(&self, files: i32, ranks: i32) -> Option<Self> {
        const FILE_CARDINALITY: i32 = 8;

        let offset = files + ranks * FILE_CARDINALITY;

        let new_index = self.index as i32 + offset;
        if new_index >= 0 && new_index < 64 {
            Some(Self {
                index: new_index as u32,
            })
        } else {
            None
        }
    }

    pub const fn is_valid(r: i64, f: i64) -> bool {
        r >= 0 && r < 8 && f >= 0 && f < 8
    }

    pub const fn from_rank_file(r: i64, f: i64) -> Self {
        if Self::is_valid(r, f) {
            Self {
                index: (r * 8 + f) as u32,
            }
        } else {
            Square::NONE
        }
    }

    pub fn to_string(&self) -> String {
        let file = self.file();
        let rank = self.rank();

        format!("{}{}", file.to_string(), rank.to_string())
    }
}

impl Add<Square> for Square {
    type Output = Square;

    fn add(self, rhs: Square) -> Square {
        let new_index = self.index as i32 + rhs.index as i32;
        Self {
            index: new_index as u32,
        }
    }
}

impl Add<FlatSquareOffset> for Square {
    type Output = Square;

    fn add(self, rhs: FlatSquareOffset) -> Square {
        let new_index = self.index as i32 + rhs.to_i8() as i32;
        Self {
            index: new_index as u32,
        }
    }
}

impl Sub<Square> for Square {
    type Output = Square;

    fn sub(self, rhs: Square) -> Square {
        let new_index = self.index as i32 - rhs.index as i32;
        Self {
            index: new_index as u32,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct File {
    index: u32,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Rank {
    index: u32,
}

impl File {
    pub const A: Self = Self { index: 0 };
    pub const B: Self = Self { index: 1 };
    pub const C: Self = Self { index: 2 };
    pub const D: Self = Self { index: 3 };
    pub const E: Self = Self { index: 4 };
    pub const F: Self = Self { index: 5 };
    pub const G: Self = Self { index: 6 };
    pub const H: Self = Self { index: 7 };

    pub const fn new(index: u32) -> Self {
        Self { index }
    }

    pub const fn to_u32(&self) -> u32 {
        self.index
    }

    pub const fn from_u32(index: u32) -> Self {
        Self { index }
    }

    pub fn to_string(&self) -> String {
        match self.index {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => panic!(""),
        }
        .to_string()
    }
}

impl Rank {
    pub const First: Self = Self { index: 0 };
    pub const Second: Self = Self { index: 1 };
    pub const Third: Self = Self { index: 2 };
    pub const Fourth: Self = Self { index: 3 };
    pub const Fifth: Self = Self { index: 4 };
    pub const Sixth: Self = Self { index: 5 };
    pub const Seventh: Self = Self { index: 6 };
    pub const Eighth: Self = Self { index: 7 };

    pub const fn new(index: u32) -> Self {
        Self { index }
    }

    pub const fn to_u32(&self) -> u32 {
        self.index
    }

    pub const fn from_u32(index: u32) -> Self {
        Self { index }
    }

    pub fn to_string(&self) -> String {
        match self.index {
            0 => "1",
            1 => "2",
            2 => "3",
            3 => "4",
            4 => "5",
            5 => "6",
            6 => "7",
            7 => "8",
            _ => panic!(""),
        }
        .to_string()
    }
}
