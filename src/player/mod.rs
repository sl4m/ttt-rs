mod computer;
mod console;

pub use computer::Computer;
pub use console::Human;

use crate::Board;
use crate::Mark;

pub trait Player {
    fn get_move(&self, board: &Board) -> usize;
    fn mark(&self) -> Mark;
}
