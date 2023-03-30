use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Chip {
    One,
    Two,
}


impl Display for Chip {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => write!(f, "O"),
            Self::Two => write!(f, "T"),
        }
    }
}
