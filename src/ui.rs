use crate::std_io::{console_io::ConsoleIo, StdIo};

pub(crate) struct Ui<T: StdIo> {
    io: T,
}

impl<T> Ui<T>
where
    T: StdIo,
{
    pub fn new(io: T) -> Ui<T> {
        Ui { io }
    }

    pub fn prompt_with_text(&self, text: &str) -> String {
        self.io.println(text);
        self.io.prompt()
    }

    pub fn print(&self, text: &str) {
        self.io.println(text);
    }

    fn io_mut(&mut self) -> &mut T {
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
    fn it_prompts_with_custom_text() {
        let std_io = DoubleStdIo::new(vec!["1"]);
        let mut ui = Ui::new(std_io);
        let text = "Enter a move";
        assert_eq!("1".to_owned(), ui.prompt_with_text(text));
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
