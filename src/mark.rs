use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mark {
    O,
    X,
}

impl Mark {
    pub fn opposite(self) -> Self {
        match self {
            Self::O => Self::X,
            Self::X => Self::O,
        }
    }
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::O => write!(f, "O"),
            Self::X => write!(f, "X"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_mark_as_string() {
        assert_eq!("O", Mark::O.to_string());
        assert_eq!("X", Mark::X.to_string());
    }

    #[test]
    fn it_gets_opposite_mark() {
        assert_eq!(Mark::X, Mark::O.opposite());
        assert_eq!(Mark::O, Mark::X.opposite());
    }
}
