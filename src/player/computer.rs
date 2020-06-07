use crate::ai::Ai;
use crate::board::Board;
use crate::mark::Mark;
use crate::player::Player;
use crate::std_io::StdIo;
use crate::ui::Ui;

pub(crate) struct Computer<T: Ai> {
    ai: T,
    mark: Mark,
}

impl<T> Computer<T>
where
    T: Ai,
{
    const MESSAGE: &'static str = "Computer makes a move";

    pub fn new(ai: T, mark: Mark) -> Computer<T> {
        Computer { ai, mark }
    }
}

impl<T> Player for Computer<T>
where
    T: Ai,
{
    fn get_move<U: StdIo>(&self, board: &Board, ui: &Ui<'_, U>) -> usize {
        ui.print(Self::MESSAGE);
        self.ai.search(board, self.mark)
    }

    fn mark(&self) -> &Mark {
        &self.mark
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::{new_board, DoubleStdIo};
    use std::cell::RefCell;

    struct DoubleAi {
        moves: RefCell<Vec<usize>>,
    }

    impl DoubleAi {
        fn new(moves: Vec<usize>) -> Self {
            Self {
                moves: RefCell::new(moves),
            }
        }
    }

    impl Ai for DoubleAi {
        fn search(&self, _board: &Board, _mark: Mark) -> usize {
            self.moves.borrow_mut().pop().unwrap()
        }
    }

    #[test]
    fn it_returns_the_mark() {
        assert_eq!(&Mark::X, new_computer(vec![]).mark());
    }

    #[test]
    fn it_returns_a_valid_move() {
        let std_io = DoubleStdIo::new(vec![]);
        let ui = Ui::new(&std_io);
        let computer = new_computer(vec![1, 8]);
        assert_eq!(8, computer.get_move(&new_board(), &ui));
        assert_eq!(1, computer.get_move(&new_board(), &ui));
    }

    fn new_computer(moves: Vec<usize>) -> Computer<DoubleAi> {
        let ai = DoubleAi::new(moves);
        Computer::new(ai, Mark::X)
    }
}
