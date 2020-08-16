use crate::config::Fragment;
use serde::de::Deserializer;
use serde::de::{SeqAccess, Visitor};
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct CompositePattern {
    pub parts: Vec<String>,
}

impl CompositePattern {
    pub fn assemble_pattern(&self, fragments: &Vec<Fragment>) -> Result<String, String> {
        let mut pat = String::new();
        for p in &self.parts {
            if p.starts_with("@") && p.ends_with("@") {
                let p_name = p.strip_suffix("@").unwrap().strip_prefix("@").unwrap();
                pat += &{
                    let mut s: String = Default::default();
                    for i in fragments {
                        if i.name == p_name {
                            let tmp_pat = i.pattern.assemble_pattern(fragments)?;
                            s = tmp_pat;
                            break;
                        }
                    }
                    s.clone()
                };
            } else {
                pat += &p;
            }
        }
        Ok(pat)
    }
}

impl Serialize for CompositePattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.parts.len() < 1 {
            serializer.serialize_none()
        } else if self.parts.len() == 1 {
            serializer.serialize_str(&self.parts[0])
        } else {
            let mut seq = serializer.serialize_seq(Some(self.parts.len()))?;
            for p in &self.parts {
                seq.serialize_element(&p)?;
            }
            seq.end()
        }
    }
}

impl<'de> Deserialize<'de> for CompositePattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CompositePatternVisitor;

        impl<'de> Visitor<'de> for CompositePatternVisitor {
            type Value = CompositePattern;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CompositePattern")
            }
            
            fn visit_str<E>(self, value: &str) -> Result<CompositePattern, E>
            {
                Ok(CompositePattern::from(value))
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<CompositePattern, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut strs: Vec<String> = Vec::new();
                loop {
                    if let Ok(Some(s)) = seq.next_element::<&str>() {
                        strs.push(String::from(s));
                    } else {
                        break;
                    }
                }
                Ok(CompositePattern::from(strs.as_slice()))
            }

        }

        deserializer.deserialize_any(CompositePatternVisitor)
    }
}


impl Default for CompositePattern {
    fn default() -> Self {
        Self { parts: Vec::new() }
    }
}

impl From<&str> for CompositePattern {
    fn from(value: &str) -> Self {
        Self {
            parts: vec![String::from(value)],
        }
    }
}
impl From<&[String]> for CompositePattern {
    fn from(value: &[String]) -> Self {
        Self {
            parts: Vec::from(value),
        }
    }
}

impl From<&[&str]> for CompositePattern {
    fn from(value: &[&str]) -> Self {
        Self {
            parts: Vec::from_iter(value.iter().map(|x| String::from(*x))),
        }
    }
}
