use crate::board::Board;
use crate::mark::Mark;
use crate::messages;
use crate::player::Player;
use crate::std_io::StdIo;
use crate::ui::Ui;

#[derive(Debug)]
pub struct Human<T: StdIo> {
    mark: Mark,
    ui: Ui<T>,
}

impl<T> Human<T>
where
    T: StdIo,
{
    pub fn new(mark: Mark, ui: Ui<T>) -> Human<T> {
        Human { mark, ui }
    }
}

impl<T> Player for Human<T>
where
    T: StdIo,
{
    fn get_move(&self, _board: &Board) -> usize {
        loop {
            let move_str =
                self.ui
                    .prompt_with_text(&format!("{}, {}", messages::PROMPT_MESSAGE, self.mark));
            if let Ok(value) = move_str.parse::<usize>() {
                break value;
            }
        }
    }

    fn mark(&self) -> Mark {
        self.mark
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::{new_board, DoubleStdIo};

    #[test]
    fn it_returns_the_mark() {
        let std_io = DoubleStdIo::new(vec![]);
        let ui = Ui::new(std_io);
        assert_eq!(Mark::X, new_human(ui).mark());
    }

    #[test]
    fn it_prompts_for_a_valid_move() {
        let std_io = DoubleStdIo::new(vec!["0"]);
        let ui = Ui::new(std_io);
        assert_eq!(0, new_human(ui).get_move(&new_board()));
    }

    #[test]
    fn it_retries_if_move_is_invalid() {
        let std_io = DoubleStdIo::new(vec!["2", "bad"]);
        let ui = Ui::new(std_io);
        assert_eq!(2, new_human(ui).get_move(&new_board()));
    }

    fn new_human(ui: Ui<DoubleStdIo<'_>>) -> Human<DoubleStdIo<'_>> {
        Human::new(Mark::X, ui)
    }
}
