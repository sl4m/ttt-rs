use crate::mark::Mark;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Board {
    grid: Vec<Option<Mark>>,
    row_size: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let grid = vec![None; size];
        let row_size = (size as f64).sqrt() as usize;
        Self { grid, row_size }
    }

    pub fn row_size(&self) -> usize {
        self.row_size
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<Mark>> {
        self.grid.iter()
    }

    pub fn is_occupied(&self, index: usize) -> bool {
        match self._mark(index) {
            Some(_) => true,
            None => false,
        }
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

    fn _mark(&self, index: usize) -> Option<&Mark> {
        self.grid.get(index).unwrap().as_ref()
    }

    fn _set_mark(&mut self, index: usize, mark: Option<Mark>) -> Option<Mark> {
        let old_mark = self.grid[index].take();
        self.grid[index] = mark;
        old_mark
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows: Vec<String> = Vec::new();
        let mut row_iter = self.grid.chunks(self.row_size);
        while let Some(row) = row_iter.next() {
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

impl Board {
    const CELL_WALL: &'static str = "---";

    fn grid_line(&self) -> String {
        let mut line_pieces: Vec<&str> = Vec::new();
        for _ in 0..self.row_size {
            line_pieces.push(Self::CELL_WALL);
        }
        format!("{}\n", line_pieces.join("+"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_row_size() {
        let board = Board::new(9);
        assert_eq!(3, board.row_size());
    }

    #[test]
    fn it_iterates_over_grid() {
        let board = Board::new(9);
        for cell in board.iter() {
            assert_eq!(None, cell.as_ref());
        }
    }

    #[test]
    fn it_sets_and_gets_mark_at_cell() {
        let mut board = Board::new(9);
        assert_eq!(None, board.mark(0));
        assert_eq!(None, board.set_mark(0, Mark::O));
        assert_eq!(Some(&Mark::O), board.mark(0));
    }

    #[test]
    fn it_checks_if_cell_is_occupied() {
        let mut board = Board::new(9);
        assert_eq!(false, board.is_occupied(0));
        assert_eq!(None, board.set_mark(0, Mark::O));
        assert_eq!(true, board.is_occupied(0));
    }

    #[test]
    fn it_clears_cell() {
        let mut board = Board::new(9);
        board.set_mark(0, Mark::X);
        assert_eq!(Some(Mark::X), board.reset_mark(0));
        assert_eq!(None, board.mark(0));
    }

    #[test]
    fn it_can_check_for_board_equality() {
        let mut board1 = Board::new(9);
        let mut board2 = Board::new(9);
        assert_eq!(board1, board2);
        board1.set_mark(0, Mark::O);
        board2.set_mark(0, Mark::O);
        assert_eq!(board1, board2);
        assert_ne!(Board::new(9), Board::new(16));
    }

    #[test]
    fn it_displays_the_board_as_string() {
        let mut board = Board::new(9);
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
}
