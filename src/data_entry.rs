use crate::chess::{position::Position, r#move::Move};

#[derive(Debug, Clone, Copy)]
pub struct TrainingDataEntry {
    pub pos: Position,
    pub mv: Move,
    pub score: i16,
    pub ply: u16,
    pub result: i16,
}
