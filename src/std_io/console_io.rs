use crate::StdIo;
use std::io;

pub(crate) struct ConsoleIo;

impl ConsoleIo {
    pub fn new() -> Self {
        Self {}
    }
}

impl StdIo for ConsoleIo {
    #[allow(clippy::print_stdout)]
    fn println(&self, text: &str) {
        println!("{}", text);
    }

    fn prompt(&self) -> String {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => buffer.trim().to_owned(),
            Err(e) => panic!("Could not read from stdin, {}", e),
        }
    }
}
