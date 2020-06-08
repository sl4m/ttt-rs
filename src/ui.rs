use crate::std_io::{console_io::ConsoleIo, StdIo};

#[derive(Debug)]
pub struct Ui<T: StdIo> {
    io: T,
}

impl<T> Ui<T>
where
    T: StdIo,
{
    const CPU_MESSAGE: &'static str = "Computer makes a move";
    const DRAW_MESSAGE: &'static str = "It is a draw game";
    const PROMPT_MESSAGE: &'static str = "Make your move";
    const WIN_MESSAGE: &'static str = "The winner is";

    pub fn new(io: T) -> Ui<T> {
        Ui { io }
    }

    pub fn prompt_for_move(&self, mark_string: &str) -> String {
        self.io
            .println(&format!("{}, {}", Self::PROMPT_MESSAGE, mark_string));
        self.io.prompt()
    }

    pub fn print_for_computer_move(&self) {
        self.io.println(Self::CPU_MESSAGE);
    }

    pub fn print_winner(&self, mark_string: &str) {
        self.io
            .println(&format!("{} {}", Self::WIN_MESSAGE, mark_string));
    }

    pub fn print_draw(&self) {
        self.io.println(Self::DRAW_MESSAGE);
    }

    pub fn print(&self, text: &str) {
        self.io.println(text);
    }

    pub fn io_mut(&mut self) -> &mut T {
        #![allow(dead_code)]
        &mut self.io
    }
}

impl Ui<ConsoleIo> {
    pub fn with_defaults() -> Ui<ConsoleIo> {
        let io = ConsoleIo::new();
        Self::new(io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::DoubleStdIo;

    #[test]
    fn it_prompts_for_move() {
        let std_io = DoubleStdIo::new(vec!["1"]);
        let mut ui = Ui::new(std_io);
        let text = "Make your move, X";
        assert_eq!("1".to_owned(), ui.prompt_for_move("X"));
        assert_eq!(text, ui.io_mut().pop_output());
    }

    #[test]
    fn it_prints_for_computer_move() {
        let std_io = DoubleStdIo::new(vec![]);
        let mut ui = Ui::new(std_io);
        let text = "Computer makes a move";
        ui.print_for_computer_move();
        assert_eq!(text, ui.io_mut().pop_output());
    }

    #[test]
    fn it_prints_winner() {
        let std_io = DoubleStdIo::new(vec![]);
        let mut ui = Ui::new(std_io);
        let text = "The winner is X";
        ui.print_winner("X");
        assert_eq!(text, ui.io_mut().pop_output());
    }

    #[test]
    fn it_prints_draw() {
        let std_io = DoubleStdIo::new(vec![]);
        let mut ui = Ui::new(std_io);
        let text = "It is a draw game";
        ui.print_draw();
        assert_eq!(text, ui.io_mut().pop_output());
    }

    #[test]
    fn it_prints_custom_text() {
        let std_io = DoubleStdIo::new(vec![]);
        let mut ui = Ui::new(std_io);
        let text = "Computer makes a move";
        ui.print(text);
        assert_eq!(text, ui.io_mut().pop_output());
    }
}
