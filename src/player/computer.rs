use crate::ai::{negamax::Negamax, Ai};
use crate::board::Board;
use crate::mark::Mark;
use crate::messages;
use crate::player::Player;
use crate::std_io::StdIo;
use crate::ui::Ui;

pub(crate) struct Computer<T: Ai, U: StdIo> {
    ai: T,
    mark: Mark,
    ui: Ui<U>,
}

impl<T, U> Computer<T, U>
where
    T: Ai,
    U: StdIo,
{
    pub fn new(ai: T, mark: Mark, ui: Ui<U>) -> Computer<T, U> {
        Computer { ai, mark, ui }
    }
}

impl<U> Computer<Negamax, U>
where
    U: StdIo,
{
    pub fn with_defaults(mark: Mark, ui: Ui<U>) -> Computer<Negamax, U> {
        let ai = Negamax::new();
        Self::new(ai, mark, ui)
    }
}

impl<T, U> Player for Computer<T, U>
where
    T: Ai,
    U: StdIo,
{
    fn get_move(&self, board: &Board) -> usize {
        self.ui.print(messages::CPU_MESSAGE);
        self.ai.search(board, self.mark)
    }

    fn mark(&self) -> Mark {
        self.mark
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
        let std_io = DoubleStdIo::new(vec![]);
        let ui = Ui::new(std_io);
        assert_eq!(Mark::X, new_computer(vec![], ui).mark());
    }

    #[test]
    fn it_returns_a_valid_move() {
        let std_io = DoubleStdIo::new(vec![]);
        let ui = Ui::new(std_io);
        let computer = new_computer(vec![1, 8], ui);
        assert_eq!(8, computer.get_move(&new_board()));
        assert_eq!(1, computer.get_move(&new_board()));
    }

    fn new_computer(
        moves: Vec<usize>,
        ui: Ui<DoubleStdIo<'_>>,
    ) -> Computer<DoubleAi, DoubleStdIo<'_>> {
        let ai = DoubleAi::new(moves);
        Computer::new(ai, Mark::X, ui)
    }
}
