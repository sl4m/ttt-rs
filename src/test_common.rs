use crate::{Board, StdIo};
use std::cell::RefCell;

pub(crate) struct DoubleStdIo<'a> {
    inputs: RefCell<Vec<&'a str>>,
    outputs: RefCell<Vec<String>>,
}

impl<'a> DoubleStdIo<'a> {
    pub fn new(inputs: Vec<&'a str>) -> Self {
        Self {
            inputs: RefCell::new(inputs),
            outputs: RefCell::new(vec![]),
        }
    }

    pub fn pop_output(&mut self) -> String {
        #[allow(clippy::unwrap_used)]
        self.outputs.borrow_mut().pop().unwrap()
    }
}

impl StdIo for DoubleStdIo<'_> {
    fn println(&self, text: &str) {
        self.outputs.borrow_mut().push(text.to_owned());
    }

    fn prompt(&self) -> String {
        #[allow(clippy::unwrap_used)]
        self.inputs.borrow_mut().pop().unwrap().to_owned()
    }
}

pub(crate) fn new_board() -> Board {
    Board::new(9)
}
