use crate::board::Board;
use crate::mark::Mark;
use crate::player::Player;
use crate::std_io::StdIo;
use crate::ui::Ui;

pub(crate) struct Human {
    mark: Mark,
}

impl Human {
    const PROMPT_MESSAGE: &'static str = "Make your move";

    pub fn new(mark: Mark) -> Self {
        Self { mark }
    }
}

impl Player for Human {
    fn get_move<T: StdIo>(&self, _board: &Board, ui: &Ui<'_, T>) -> usize {
        loop {
            let move_str = ui.prompt_with_text(&format!("{}, {}", Self::PROMPT_MESSAGE, self.mark));
            if let Ok(value) = move_str.parse::<usize>() {
                break value;
            }
        }
    }

    fn mark(&self) -> &Mark {
        &self.mark
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::{new_board, DoubleStdIo};

    #[test]
    fn it_returns_the_mark() {
        assert_eq!(&Mark::X, new_human().mark());
    }

    #[test]
    fn it_prompts_for_a_valid_move() {
        let std_io = DoubleStdIo::new(vec!["1"]);
        let ui = Ui::new(&std_io);
        assert_eq!(1, new_human().get_move(&new_board(), &ui));
    }

    #[test]
    fn it_retries_if_move_is_invalid() {
        let std_io = DoubleStdIo::new(vec!["2", "bad"]);
        let ui = Ui::new(&std_io);
        assert_eq!(2, new_human().get_move(&new_board(), &ui));
    }

    fn new_human() -> Human {
        Human::new(Mark::X)
    }
}
