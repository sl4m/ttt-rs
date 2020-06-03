use crate::board::Board;

pub trait StdIo {
    fn println(&self, text: &str);
    fn prompt(&self) -> String;
}

pub(crate) struct Ui<'a, T: StdIo> {
    io: &'a T,
}

impl<'a, T> Ui<'a, T>
where
    T: StdIo,
{
    pub fn new(io: &T) -> Ui<'_, T> {
        Ui { io }
    }

    pub fn prompt_for_move(&self) -> String {
        self.io.prompt()
    }

    pub fn display_board(&self, board: &Board) {
        self.io.println(&board.to_string());
    }

    pub fn print(&self, text: &str) {
        self.io.println(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Mark;
    use std::cell::RefCell;

    struct DoubleStdIo<'a> {
        inputs: RefCell<Vec<&'a str>>,
        outputs: RefCell<Vec<String>>,
    }

    impl<'a> DoubleStdIo<'a> {
        fn new(inputs: Vec<&'a str>) -> Self {
            Self {
                inputs: RefCell::new(inputs),
                outputs: RefCell::new(vec![]),
            }
        }

        fn pop_output(&mut self) -> String {
            self.outputs.borrow_mut().pop().unwrap()
        }
    }

    impl StdIo for DoubleStdIo<'_> {
        fn println(&self, text: &str) {
            self.outputs.borrow_mut().push(text.to_owned());
        }

        fn prompt(&self) -> String {
            self.inputs.borrow_mut().pop().unwrap().to_owned()
        }
    }

    #[test]
    fn it_prompts_for_move() {
        let std_io = DoubleStdIo::new(vec!["1"]);
        let ui = Ui::new(&std_io);
        assert_eq!(ui.prompt_for_move(), "1".to_owned());
    }

    #[test]
    fn it_displays_the_board() {
        let mut std_io = DoubleStdIo::new(vec![]);
        let ui = Ui::new(&std_io);
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
        ui.display_board(&board);
        assert_eq!(std_io.pop_output(), board_string);
    }

    #[test]
    fn it_prints_custom_text() {
        let mut std_io = DoubleStdIo::new(vec![]);
        let ui = Ui::new(&std_io);
        let text = "Hello world";
        ui.print(text);
        assert_eq!(std_io.pop_output(), text);
    }
}
