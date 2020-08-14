mod output_format;
mod verbose;
mod fragment;

use std::fmt::{self, Display};

pub use self::output_format::*;
pub use self::verbose::*;
pub use self::fragment::*;


use serde::{Deserialize, Serialize};



fn default_format() -> OutputFormat {
    OutputFormat::Simple
}
fn default_verbosity() -> Verbosity {
    Verbosity::NotVerbose
}
fn default_number() -> u32 {
    12
}
fn default_pretty() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_format")]
    pub format: OutputFormat,

    #[serde(default = "default_number")]
    pub number: u32,

    #[serde(default = "default_verbosity")]
    pub verbosity: Verbosity,

    #[serde(default)]
    pub pattern: Vec<String>,

    #[serde(default = "default_pretty")]
    pub pretty: bool,
    
    #[serde(default)]
    pub fragments: Vec<Fragment>,
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(s) = serde_json::to_string(self) {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format: default_format(),
            number: default_number(),
            verbosity: default_verbosity(),
            pattern: Default::default(),
            pretty: default_pretty(),
            fragments: Default::default()
        }
    }
}

pub fn println_verbosity(verbosity: Verbosity, config: &Config, message: &str) {
    if config.verbosity >= verbosity {
        println!("{}", message);
    }
}

#[cfg(feature = "color_messages")]
pub fn println_err(message: &str) {
    use termion::color::{Fg, Red, Reset as ResetColor};
    println!("{}{}{}", Fg(Red), message, Fg(ResetColor));
}
#[cfg(not(feature = "color_messages"))]
pub fn println_err(message: &str) {
    println!("{}", message);
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
