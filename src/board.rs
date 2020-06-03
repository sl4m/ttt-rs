use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Mark {
    O,
    X,
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::O => write!(f, "O"),
            Self::X => write!(f, "X"),
        }
    }
}

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

    pub fn mark_at(&self, index: usize) -> Option<&Mark> {
        self.grid.get(index).unwrap().as_ref()
    }

    pub fn set_mark_at(&mut self, index: usize, mark: Mark) -> Option<Mark> {
        let old_mark = self.grid[index].take();
        self.grid[index] = Some(mark);
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
        let mut line = String::new();
        for n in 0..self.row_size {
            if n > 0 {
                line.push('-');
            }
            line.push_str(Self::CELL_WALL);
        }
        line.push_str("\n");
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_board() {
        let board = Board::new(9);
        assert_eq!(board.row_size(), 3);
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
        assert_eq!(None, board.mark_at(0));
        assert_eq!(None, board.set_mark_at(0, Mark::O));
        assert_eq!(Some(&Mark::O), board.mark_at(0));
    }

    #[test]
    fn it_can_check_for_board_equality() {
        assert_eq!(Board::new(9), Board::new(9));
        assert_ne!(Board::new(9), Board::new(16));
    }

    #[test]
    fn it_displays_the_board_as_a_string() {
        let mut board = Board::new(9);
        for n in 0..9 {
            board.set_mark_at(n, Mark::O);
        }

        let board_string = r#" O | O | O
-----------
 O | O | O
-----------
 O | O | O
"#;

        assert_eq!(format!("{}", board), board_string);
    }
}
