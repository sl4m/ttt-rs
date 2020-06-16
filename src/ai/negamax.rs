use crate::ai::Ai;
use crate::board::Board;
use crate::mark::Mark;
use core::cmp;
use std::thread;

pub(crate) struct Negamax;

impl Negamax {
    const MIN: i16 = -999;
    const MAX: i16 = 999;
    const DEFAULT_DEPTH: i16 = 5;

    pub fn new() -> Self {
        Self {}
    }

    fn search(board: &Board, mark: Mark) -> usize {
        let handles = Self::concur_search(board, mark);

        let mut scores = handles.into_iter().fold(vec![], |mut acc, h| {
            acc.push(h.join().expect("thread could not be joined"));
            acc
        });
        scores.sort_by(|a, b| b.1.cmp(&a.1));
        scores[0].0
    }

    fn concur_search(board: &Board, mark: Mark) -> Vec<thread::JoinHandle<(usize, i16)>> {
        board
            .empty_cell_indices()
            .into_iter()
            .fold(vec![], |mut acc, index| {
                let mut new_board = board.clone();
                acc.push(thread::spawn(move || {
                    new_board.set_mark(index, mark);
                    let score = -Self::negamax_init(&mut new_board, mark.opposite());
                    (index, score)
                }));
                acc
            })
    }

    fn negamax_init(board: &mut Board, mark: Mark) -> i16 {
        Self::negamax(board, mark, Self::DEFAULT_DEPTH, Self::MIN, Self::MAX)
    }

    fn negamax(board: &mut Board, mark: Mark, depth: i16, alpha: i16, beta: i16) -> i16 {
        if depth == 0 || board.is_game_over() {
            Self::score(board, mark)
        } else {
            let mut alpha_mut = alpha;
            for index in board.empty_cell_indices() {
                board.set_mark(index, mark);
                let score = -Self::negamax(board, mark.opposite(), depth - 1, -beta, -alpha_mut);
                board.reset_mark(index);
                alpha_mut = cmp::max(score + depth, alpha_mut);
                if alpha_mut >= beta {
                    break;
                }
            }
            alpha_mut
        }
    }

    fn score(board: &Board, mark: Mark) -> i16 {
        if let Some(winner) = board.winner() {
            if *winner == mark {
                Self::MAX
            } else {
                Self::MIN
            }
        } else {
            0
        }
    }
}

impl Ai for Negamax {
    fn search(&self, board: &Board, mark: Mark) -> usize {
        Self::search(board, mark)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::new_board;

    #[test]
    fn it_makes_immediate_win() {
        let mut board = new_board();
        board.set_mark(2, Mark::O);
        board.set_mark(4, Mark::X);
        board.set_mark(6, Mark::X);
        board.set_mark(7, Mark::O);
        assert_eq!(0, Negamax::new().search(&board, Mark::O));
        assert_eq!(0, Negamax::search(&board, Mark::O));
    }

    #[test]
    fn it_makes_immediate_win_alternate() {
        let mut board = new_board();
        board.set_mark(0, Mark::X);
        board.set_mark(1, Mark::O);
        board.set_mark(2, Mark::X);
        board.set_mark(4, Mark::O);
        board.set_mark(8, Mark::X);
        assert_eq!(7, Negamax::new().search(&board, Mark::O));
        assert_eq!(7, Negamax::search(&board, Mark::O));
    }

    #[test]
    fn it_blocks_immediate_win() {
        let mut board = new_board();
        board.set_mark(0, Mark::X);
        board.set_mark(2, Mark::X);
        board.set_mark(4, Mark::O);
        assert_eq!(1, Negamax::search(&board, Mark::O));
    }

    #[test]
    fn it_makes_strategic_move() {
        let mut board = new_board();
        board.set_mark(0, Mark::X);
        board.set_mark(4, Mark::O);
        board.set_mark(8, Mark::X);
        assert_eq!(3, Negamax::search(&board, Mark::O));
    }

    #[test]
    fn it_makes_blocks_potential_two_way_win() {
        let mut board = new_board();
        board.set_mark(2, Mark::O);
        board.set_mark(4, Mark::X);
        board.set_mark(6, Mark::X);
        assert_eq!(0, Negamax::search(&board, Mark::O));
    }

    #[test]
    fn it_makes_corner_move() {
        let mut board = new_board();
        board.set_mark(0, Mark::X);
        assert_eq!(4, Negamax::search(&board, Mark::O));
    }
}
