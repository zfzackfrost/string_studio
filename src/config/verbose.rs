use std::fmt::{self, Display};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub enum Verbosity {
    NotVerbose,
    Verbose,
    VeryVerbose,
}

impl From<u8> for Verbosity {
    fn from(verbosity: u8) -> Self {
        match verbosity {
            0 => Self::NotVerbose,
            1 => Self::Verbose,
            2 | _ => Self::VeryVerbose,
        }
    }
}

impl Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotVerbose => write!(f, "NotVerbose"),
            Self::Verbose => write!(f, "Verbose"),
            Self::VeryVerbose => write!(f, "VeryVerbose"),
        }
    }
}
