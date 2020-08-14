use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
pub enum OutputFormat {
    Simple,
    Table,
    Json,
    Csv,
}

impl From<&str> for OutputFormat {
    fn from(fmt: &str) -> Self {
        match fmt {
            "table" => Self::Table,
            "json" => Self::Json,
            "csv" => Self::Csv,
            "simple" | _ => Self::Simple
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple => write!(f, "Simple"),
            Self::Table => write!(f, "Table"),
            Self::Json => write!(f, "Json"),
            Self::Csv => write!(f, "Csv"),
        }
    }
}
