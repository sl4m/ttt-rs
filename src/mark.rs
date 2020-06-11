use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mark {
    O = 0,
    X = 1,
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

impl From<&str> for Mark {
    fn from(mark: &str) -> Self {
        match mark {
            "X" | "x" => Self::X,
            "O" | "o" => Self::O,
            _ => panic!("Could not convert {} to a Mark", mark),
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

    #[test]
    fn it_converts_from_str() {
        assert_eq!(Mark::X, Mark::from("X"));
        assert_eq!(Mark::O, Mark::from("O"));
        assert_eq!(Mark::X, Mark::from("x"));
        assert_eq!(Mark::O, Mark::from("o"));
    }
}
