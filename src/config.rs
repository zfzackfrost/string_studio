mod fragment;
mod output_format;
mod verbose;

pub use self::fragment::*;
pub use self::output_format::*;
pub use self::verbose::*;
use std::fmt::{self, Display};
use std::path::PathBuf;

pub use crate::pattern::*;
pub use crate::xform::*;

use serde::{Deserialize, Serialize};

fn default_format() -> OutputFormat {
    OutputFormat::Simple
}
fn default_verbosity() -> Verbosity {
    Verbosity::NotVerbose
}
fn default_number() -> u32 {
    15
}
fn default_pretty() -> bool {
    false
}
fn default_xforms() -> Vec<Xform> {
    vec![]
}

fn default_fragments() -> Vec<Fragment> {
    lazy_static! {
        static ref LOWER_VOWELS_RE: String = String::from("(a|e|i|o|u)");
        static ref UPPER_VOWELS_RE: String = String::from("(A|E|I|O|U)");
        static ref LOWER_VOWELS_Y_RE: String = String::from("(a|e|i|o|u|y)");
        static ref UPPER_VOWELS_Y_RE: String = String::from("(A|E|I|O|U|Y)");
        static ref LOWER_CONS_RE: String =
            String::from("(b|c|d|f|g|h|j|k|l|m|n|p|q|r|s|t|v|w|x|y|z)");
        static ref UPPER_CONS_RE: String =
            String::from("(B|C|D|F|G|H|J|K|L|M|N|P|Q|R|S|T|V|W|X|Y|Z)");
        static ref VOWEL_CLUSTER_RE: String =
            String::from("(ae|ai|ou|ia|ei|ou|ou|ui|iu|ea|oi|ua|au|ao|oa|ee|oo)");
    }
    vec![
        Fragment::new(
            "lower_vowel",
            CompositePattern::from((*LOWER_VOWELS_RE).as_str()),
            "Lowercase vowels, excluding `y`",
        ),
        Fragment::new(
            "upper_vowel",
            CompositePattern::from((*UPPER_VOWELS_RE).as_str()),
            "Uppercase vowels, excluding `y`",
        ),
        Fragment::new(
            "lower_vowel_y",
            CompositePattern::from((*LOWER_VOWELS_Y_RE).as_str()),
            "Lowercase vowels, including `y`",
        ),
        Fragment::new(
            "upper_vowel_y",
            CompositePattern::from((*UPPER_VOWELS_Y_RE).as_str()),
            "Uppercase vowels, including `y`",
        ),
        Fragment::new(
            "lower_cons",
            CompositePattern::from((*LOWER_CONS_RE).as_str()),
            "Lowercase consonants.",
        ),
        Fragment::new(
            "upper_cons",
            CompositePattern::from((*UPPER_CONS_RE).as_str()),
            "Uppercase consonants.",
        ),
        Fragment::new(
            "vowel_cluster",
            CompositePattern::from((*VOWEL_CLUSTER_RE).as_str()),
            "A cluster of readable vowels.",
        ),
        Fragment::new(
            "syllable",
            CompositePattern::from(&[
                "(",
                "@lower_cons@",
                "(",
                "@vowel_cluster@",
                "|",
                "@lower_vowel@",
                ")",
                "@lower_cons@",
                "?)",
            ] as &[&str]),
            "A basic syllable.",
        ),
    ]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_format")]
    pub format: OutputFormat,
    #[serde(default = "default_number")]
    pub number: u32,

    #[serde(default = "default_verbosity", skip)]
    pub verbosity: Verbosity,

    #[serde(default, skip)]
    pub pattern: CompositePattern,

    #[serde(default = "default_pretty")]
    pub pretty: bool,
    #[serde(default, skip)]
    pub seed: u64,

    #[serde(default)]
    pub fragments: Vec<Fragment>,

    #[serde(default, skip)]
    pub xforms: Vec<Xform>,
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
            seed: 0,
            fragments: default_fragments(),
            xforms: default_xforms(),
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

pub fn get_cfg_file_path() -> Option<PathBuf> {
    use directories::ProjectDirs;

    if let Some(proj_dirs) = ProjectDirs::from("com", "", "String Studio") {
        let dir = proj_dirs.config_dir();
        let path = dir.join("string_studio.json");
        return Some(path);
    }
    None
}
