pub(crate) mod computer;
pub(crate) mod human;

use crate::board::Board;
use crate::mark::Mark;
use crate::std_io::StdIo;
use crate::ui::Ui;

pub(crate) trait Player {
    fn get_move<T: StdIo>(&self, board: &Board, ui: &Ui<'_, T>) -> usize;
    fn mark(&self) -> &Mark;
}
