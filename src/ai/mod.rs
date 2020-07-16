use crate::{Board, Mark};

mod negamax;

pub(crate) use negamax::Negamax;

pub trait Ai {
    fn search(&self, board: &Board, mark: Mark) -> usize;
}
