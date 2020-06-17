use crate::board::Board;
use crate::mark::Mark;

pub mod negamax;

pub trait Ai {
    fn search(&self, board: &Board, mark: Mark) -> usize;
}
