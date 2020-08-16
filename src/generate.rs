pub mod regex_gen;

use crate::config::Config;

use rand::prelude::*;
use rand_xoshiro::Xoshiro512StarStar;

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
fn apply_xforms(config: &Config, s: String) -> String {
    let mut s = s;
    for x in &config.xforms {
        s = x.xform(&s);
    }
    s
}

pub fn generate(config: &Config) -> Result<Vec<String>, String> {
    let pat = assemble_pattern(config)?;
    let mut rng = if config.seed == 0 {
        Xoshiro512StarStar::from_entropy()
    } else {
        Xoshiro512StarStar::seed_from_u64(config.seed)
    };
    if let Some(gen) = RegexGen::new(&pat) {
        let mut strings: Vec<String> = Vec::new();

        for _ in 0..config.number {
            match gen.randomize(&mut rng) {
                Ok(s) => strings.push(apply_xforms(config, s)),
                Err(_) => return Err(String::from("Failed to generate string!")),
            }
        }
        Ok(strings)
    } else {
        Err(String::from("Failed to parse pattern!"))
    }
}
