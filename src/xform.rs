use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
pub use std::convert::TryFrom;
use inflector::cases::titlecase;

#[derive(Debug, Serialize, Deserialize)]
pub enum Xform {
    UAfterQ,
    TitleCase,
}

impl Xform {
    fn xform_uafterq(s: &str) -> String {
        lazy_static! {
            static ref UAFTERQ_RE: Regex = Regex::new("(q|Q)((?:u|U)+)?").unwrap();
        }

        let do_replace = |caps: &Captures| -> String {
            if let None = caps.get(2) {
                let s = caps.get(1).unwrap().as_str();
                format!("{}u", s)
            } else {
                String::from(caps.get(0).unwrap().as_str())
            }
        };

        (*UAFTERQ_RE).replace_all(s, do_replace).into_owned()
    }

    fn xform_titlecase(s: &str) -> String {
        titlecase::to_title_case(s)
    }
    pub fn xform(&self, s: &str) -> String {
        match self {
            Self::UAfterQ => Self::xform_uafterq(s),
            Self::TitleCase => Self::xform_titlecase(s),
        }
    }
}

impl TryFrom<&str> for Xform {
    type Error = &'static str;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name {
            "u_after_q" => Ok(Self::UAfterQ),
            "title_case" => Ok(Self::TitleCase),
            _ => Err("Invalid transform type!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xform_uafterq_test() {
        let x = Xform::UAfterQ;
        assert_eq!(x.xform("qit"), "quit");
        assert_eq!(x.xform("Qit"), "Quit");
        assert_eq!(x.xform("quit"), "quit");
        assert_eq!(x.xform("Quit"), "Quit");
        assert_eq!(x.xform("foo"), "foo");
    }
}
