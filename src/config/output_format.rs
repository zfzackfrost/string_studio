use std::fmt::{self, Display};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum OutputFormat {
    Simple,
    #[cfg(feature = "table_format")]
    Table,
    Json,
    Csv,
}

impl From<&str> for OutputFormat {
    fn from(fmt: &str) -> Self {
        match fmt {
            #[cfg(feature = "table_format")]
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
            #[cfg(feature = "table_format")]
            Self::Table => write!(f, "Table"),
            Self::Simple => write!(f, "Simple"),
            Self::Json => write!(f, "Json"),
            Self::Csv => write!(f, "Csv"),
        }
    }
}
