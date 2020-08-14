mod output_format;
mod verbose;

use std::fmt::{self, Display};

pub use self::output_format::*;
pub use self::verbose::*;

use termion::color::{Fg, Red, Reset as ResetColor};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub format: OutputFormat,
    pub number: u32,
    pub verbosity: Verbosity,
    pub pattern: String,
    pub pretty: bool,
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(s) = serde_json::to_string(self) {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

pub fn println_verbosity(verbosity: Verbosity, config: &Config, message: &str) {
    if config.verbosity >= verbosity {
        println!("{}", message);
    }
}

pub fn println_err(message: &str) {
    println!("{}{}{}", Fg(Red), message, Fg(ResetColor));
}
pub fn println_v0(config: &Config, message: &str) {
    println_verbosity(Verbosity::NotVerbose, config, message);
}
pub fn println_v1(config: &Config, message: &str) {
    println_verbosity(Verbosity::Verbose, config, message);
}
pub fn println_v2(config: &Config, message: &str) {
    println_verbosity(Verbosity::VeryVerbose, config, message);
}
