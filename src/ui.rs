use crate::std_io::StdIo;

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

    pub fn prompt_with_text(&self, text: &str) -> String {
        self.io.println(text);
        self.io.prompt()
    }

    pub fn print(&self, text: &str) {
        self.io.println(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::DoubleStdIo;

    #[test]
    fn it_prompts_with_custom_text() {
        let mut std_io = DoubleStdIo::new(vec!["1"]);
        let ui = Ui::new(&std_io);
        let text = "Enter a move";
        assert_eq!("1".to_owned(), ui.prompt_with_text(text));
        assert_eq!(text, std_io.pop_output());
    }

    #[test]
    fn it_prints_custom_text() {
        let mut std_io = DoubleStdIo::new(vec![]);
        let ui = Ui::new(&std_io);
        let text = "Computer makes a move";
        ui.print(text);
        assert_eq!(text, std_io.pop_output());
    }
}
