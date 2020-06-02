#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Mark {
    O,
    X,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Board {
    grid: Vec<Option<Mark>>,
}

impl Board {
    fn new(size: usize) -> Self {
        let grid = vec![None; size];
        Self { grid }
    }

    fn size(&self) -> usize {
        self.grid.len()
    }

    fn iter(&self) -> impl Iterator<Item = &Option<Mark>> {
        self.grid.iter()
    }

    fn mark_at(&self, index: usize) -> Option<&Mark> {
        self.grid.get(index).unwrap().as_ref()
    }

    fn set_mark_at(&mut self, index: usize, mark: Mark) -> Option<Mark> {
        let old_mark = self.grid[index].take();
        self.grid[index] = Some(mark);
        old_mark
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_board() {
        let board = Board::new(9);
        assert_eq!(board.size(), 9);
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
}
