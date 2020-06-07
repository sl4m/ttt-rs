pub(crate) mod console_io;

pub trait StdIo {
    fn println(&self, text: &str);
    fn prompt(&self) -> String;
}
