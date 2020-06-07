use crate::mark::Mark;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Board {
    grid: Vec<Option<Mark>>,
    row_size: usize,
    size: usize,
    win_combos: Vec<Vec<usize>>,
}

impl Board {
    const CELL_WALL: &'static str = "---";

    pub fn new(size: usize) -> Self {
        let grid = vec![None; size];
        let row_size = Self::sqrt(size);
        let win_combos: Vec<Vec<usize>> = Self::gen_win_combos(size, row_size);
        Self {
            grid,
            row_size,
            size,
            win_combos,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn row_size(&self) -> usize {
        #![allow(dead_code)]
        self.row_size
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<Mark>> {
        #![allow(dead_code)]
        self.grid.iter()
    }

    pub fn empty_cell_indices(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(i, n)| if n.is_none() { Some(i) } else { None })
            .collect()
    }

    pub fn is_occupied(&self, index: usize) -> bool {
        self._mark(index).is_some()
    }

    pub fn is_all_occupied(&self) -> bool {
        self.grid.iter().all(Option::is_some)
    }

    pub fn mark(&self, index: usize) -> Option<&Mark> {
        self._mark(index)
    }

    pub fn reset_mark(&mut self, index: usize) -> Option<Mark> {
        self._set_mark(index, None)
    }

    pub fn set_mark(&mut self, index: usize, mark: Mark) -> Option<Mark> {
        self._set_mark(index, Some(mark))
    }

    pub fn is_game_over(&self) -> bool {
        self.is_all_occupied() || self.winner().is_some()
    }

    pub fn winner(&self) -> Option<&Mark> {
        let maybe_row = &self.win_combos.iter().find(|&row| {
            let mark = self.mark(row[0]);
            row.iter().all(|n| {
                let cell = self.mark(*n);
                cell.is_some() && cell == mark
            })
        });
        if let Some(row) = maybe_row {
            self.mark(row[0])
        } else {
            None
        }
    }

    fn win_combos(&self) -> &Vec<Vec<usize>> {
        #![allow(dead_code)]
        &self.win_combos
    }

    fn gen_win_combos(board_size: usize, row_size: usize) -> Vec<Vec<usize>> {
        let grid: Vec<usize> = (0..board_size).collect();
        let mut combos: Vec<Vec<usize>> = grid.chunks(row_size).map(|row| row.to_vec()).collect();

        for col in 0..row_size {
            let mut col_combo = vec![];
            for row in grid.chunks(row_size) {
                col_combo.push(row[col]);
            }
            combos.push(col_combo);
        }

        combos.push(
            grid.chunks(row_size)
                .enumerate()
                .map(|(index, row)| row[index])
                .collect(),
        );

        combos.push(
            grid.chunks(row_size)
                .rev()
                .enumerate()
                .map(|(index, row)| row[index])
                .collect(),
        );
        combos
    }

    fn _mark(&self, index: usize) -> Option<&Mark> {
        self.grid[index].as_ref()
    }

    fn _set_mark(&mut self, index: usize, mark: Option<Mark>) -> Option<Mark> {
        let old_mark = self.grid[index].take();
        self.grid[index] = mark;
        old_mark
    }

    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    fn sqrt(size: usize) -> usize {
        (size as f64).sqrt() as usize
    }

    fn grid_line(&self) -> String {
        let mut line_pieces: Vec<&str> = vec![];
        for _ in 0..self.row_size {
            line_pieces.push(Self::CELL_WALL);
        }
        format!("{}\n", line_pieces.join("+"))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows: Vec<String> = vec![];
        let row_iter = self.grid.chunks(self.row_size);
        for row in row_iter {
            let row = row
                .iter()
                .map(|cell| match cell {
                    Some(cell) => cell.to_string(),
                    None => " ".to_string(),
                })
                .collect::<Vec<String>>();
            rows.push(format!(" {}\n", row.join(" | ")));
        }

        write!(f, "{}", rows.join(&self.grid_line()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::new_board;

    #[test]
    fn it_gets_size() {
        let board = new_board();
        assert_eq!(9, board.size());
    }

    #[test]
    fn it_gets_row_size() {
        let board = new_board();
        assert_eq!(3, board.row_size());
    }

    #[test]
    fn it_iterates_over_grid() {
        let board = new_board();
        for cell in board.iter() {
            assert_eq!(None, cell.as_ref());
        }
    }

    #[test]
    fn it_returns_empty_cell_indices() {
        let mut board = new_board();
        board.set_mark(0, Mark::O);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], board.empty_cell_indices());
        for n in 1..9 {
            board.set_mark(n, Mark::O);
        }
        let empty_vec: Vec<usize> = vec![];
        assert_eq!(empty_vec, board.empty_cell_indices());
    }

    #[test]
    fn it_sets_and_gets_mark_at_cell() {
        let mut board = new_board();
        assert_eq!(None, board.mark(0));
        assert_eq!(None, board.set_mark(0, Mark::O));
        assert_eq!(Some(&Mark::O), board.mark(0));
    }

    #[test]
    fn it_checks_if_cell_is_occupied() {
        let mut board = new_board();
        assert_eq!(false, board.is_occupied(0));
        assert_eq!(None, board.set_mark(0, Mark::O));
        assert_eq!(true, board.is_occupied(0));
    }

    #[test]
    fn it_checks_if_all_cells_occupied() {
        let mut board = new_board();
        board.set_mark(0, Mark::O);
        assert_eq!(false, board.is_all_occupied());
        for n in 1..9 {
            board.set_mark(n, Mark::O);
        }
        assert_eq!(true, board.is_all_occupied());
    }

    #[test]
    fn it_clears_cell() {
        let mut board = new_board();
        board.set_mark(0, Mark::X);
        assert_eq!(Some(Mark::X), board.reset_mark(0));
        assert_eq!(None, board.mark(0));
    }

    #[test]
    fn it_displays_the_board_as_string() {
        let mut board = new_board();
        for n in 0..9 {
            board.set_mark(n, Mark::O);
        }

        let board_string = r#" O | O | O
---+---+---
 O | O | O
---+---+---
 O | O | O
"#;

        assert_eq!(board_string, format!("{}", board));
    }

    #[test]
    fn it_generates_win_combos_for_3x3() {
        let board = new_board();
        let expectation: Vec<Vec<usize>> = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![0, 3, 6],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![0, 4, 8],
            vec![6, 4, 2],
        ];
        assert_eq!(&expectation, board.win_combos());
    }

    #[test]
    fn it_generates_win_combos_for_4x4() {
        let board = Board::new(16);
        let expectation: Vec<Vec<usize>> = vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 15],
            vec![0, 4, 8, 12],
            vec![1, 5, 9, 13],
            vec![2, 6, 10, 14],
            vec![3, 7, 11, 15],
            vec![0, 5, 10, 15],
            vec![12, 9, 6, 3],
        ];
        assert_eq!(&expectation, board.win_combos());
    }

    #[test]
    fn it_checks_for_winner_3x3() {
        let mut board = new_board();
        assert_eq!(None, board.winner());
        board.set_mark(0, Mark::O);
        board.set_mark(1, Mark::O);
        board.set_mark(2, Mark::O);
        assert_eq!(Some(&Mark::O), board.winner());
    }

    #[test]
    fn it_checks_for_winner_4x4() {
        let mut board = Board::new(16);
        assert_eq!(None, board.winner());
        board.set_mark(0, Mark::X);
        board.set_mark(5, Mark::X);
        board.set_mark(10, Mark::X);
        board.set_mark(15, Mark::X);
        assert_eq!(Some(&Mark::X), board.winner());
    }

    #[test]
    fn it_is_game_is_over_if_all_cells_occupied() {
        let mut board = new_board();
        assert_eq!(false, board.is_game_over());
        for n in 0..9 {
            board.set_mark(n, Mark::O);
        }
        assert_eq!(true, board.is_game_over());
    }

    #[test]
    fn it_is_game_is_over_if_there_is_a_winner() {
        let mut board = new_board();
        assert_eq!(false, board.is_game_over());
        for n in 0..3 {
            board.set_mark(n, Mark::O);
        }
        assert_eq!(true, board.is_game_over());
    }
}
