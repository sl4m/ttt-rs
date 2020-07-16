use crate::{Ai, Board, Mark, Negamax, Player};

#[derive(Debug)]
pub struct Computer<T: Ai> {
    ai: T,
    mark: Mark,
}

impl<T> Computer<T>
where
    T: Ai,
{
    pub fn new(ai: T, mark: Mark) -> Computer<T> {
        Computer { ai, mark }
    }
}

impl Computer<Negamax> {
    pub fn with_defaults(mark: Mark) -> Computer<Negamax> {
        let ai = Negamax::default();
        Self::new(ai, mark)
    }
}

impl<T> Player for Computer<T>
where
    T: Ai,
{
    fn get_move(&self, board: &Board) -> usize {
        self.ai.search(board, self.mark)
    }

    fn mark(&self) -> Mark {
        self.mark
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::new_board;
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
        assert_eq!(Mark::X, new_computer(vec![]).mark());
    }

    #[test]
    fn it_returns_a_valid_move() {
        let computer = new_computer(vec![1, 8]);
        assert_eq!(8, computer.get_move(&new_board()));
        assert_eq!(1, computer.get_move(&new_board()));
    }

    fn new_computer(moves: Vec<usize>) -> Computer<DoubleAi> {
        let ai = DoubleAi::new(moves);
        Computer::new(ai, Mark::X)
    }
}
