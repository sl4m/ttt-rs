pub(crate) mod computer;
pub mod console;

use crate::board::Board;
use crate::mark::Mark;

pub trait Player {
    fn get_move(&self, board: &Board) -> usize;
    fn mark(&self) -> Mark;
}
