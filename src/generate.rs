pub mod regex_gen;

use crate::config::Config;

use rand::prelude::*;
use rand_pcg::Pcg64;

use self::regex_gen::RegexGen;

fn assemble_pattern(config: &Config) -> Result<String, String> {
    let mut pat = String::new();
    for p in &config.pattern {
        if p.starts_with("@") && p.ends_with("@") {
            let p_name = p.strip_suffix("@").unwrap().strip_prefix("@").unwrap();
            pat += &{
                let mut s: String = Default::default();
                for i in &config.fragments {
                    if i.name == p_name {
                       s = i.pattern.clone();
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

pub fn generate(config: &Config) -> Result<Vec<String>, String> {
    let pat = assemble_pattern(config)?;
    let mut rng = Pcg64::from_entropy();
    if let Some(gen) = RegexGen::new(&pat) {
        let mut strings: Vec<String> = Vec::new();

        for _ in 0 .. config.number {
            match gen.randomize(&mut rng) {
                Ok(s) => strings.push(s),
                Err(_) => return Err(String::from("Failed to generate string!"))
            }
        }
        Ok(strings)
    } else {
        Err(String::from("Failed to parse pattern!"))
    }
}
