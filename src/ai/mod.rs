use crate::board::Board;
use crate::mark::Mark;

mod negamax;

pub(crate) trait Ai {
    fn search(&self, board: &Board, mark: Mark) -> usize;
}
