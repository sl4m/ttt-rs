mod console_io;

pub(crate) use console_io::ConsoleIo;

pub trait StdIo {
    fn println(&self, text: &str);
    fn prompt(&self) -> String;
}
