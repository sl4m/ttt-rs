pub(crate) mod computer;
pub(crate) mod human;

use crate::board::Board;
use crate::mark::Mark;

pub(crate) trait Player {
    fn get_move(&self, board: &Board) -> usize;
    fn mark(&self) -> &Mark;
}
