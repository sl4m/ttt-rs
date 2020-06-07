use crate::std_io::StdIo;
use std::io::{self, Read};

pub(crate) struct ConsoleIo;

impl ConsoleIo {
    fn new() -> Self {
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
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        match handle.read_to_string(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => panic!("Could not read from stdin, {}", e),
        }
    }
}
