use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
pub use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub enum Xform {
    UAfterQ,
    LowerCase,
    UpperCase,
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

    fn xform_lowercase(s: &str) -> String {
        s.to_lowercase()
    }
    
    fn xform_uppercase(s: &str) -> String {
        s.to_uppercase()
    }
    
    fn xform_titlecase(s: &str) -> String {
        lazy_static! {
            static ref TITLECASE_RE: Regex = Regex::new(r"\b(\w)(\w*)\b").unwrap();
        }

        let do_replace = |caps: &Captures| -> String {
            if let Some(s) = caps.get(1) {
                let s1 = s.as_str().to_uppercase();
                let s2 = if let Some(s2) = caps.get(2) {
                    s2.as_str()
                } else {
                    ""
                };
                format!("{}{}", s1, s2)
            } else {
                String::from(caps.get(0).unwrap().as_str())
            }
        };

        (*TITLECASE_RE).replace_all(s, do_replace).into_owned()
    }

    pub fn xform(&self, s: &str) -> String {
        match self {
            Self::UAfterQ => Self::xform_uafterq(s),
            Self::LowerCase => Self::xform_lowercase(s),
            Self::UpperCase => Self::xform_uppercase(s),
            Self::TitleCase => Self::xform_titlecase(s),
        }
    }
}

impl TryFrom<&str> for Xform {
    type Error = &'static str;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        let name = name.trim();
        match name {
            "u_after_q" => Ok(Self::UAfterQ),
            "title_case" => Ok(Self::TitleCase),
            "lower_case" => Ok(Self::LowerCase),
            "upper_case" => Ok(Self::UpperCase),
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
