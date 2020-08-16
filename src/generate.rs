pub mod regex_gen;

use crate::config::Config;

use rand::prelude::*;
use rand_xoshiro::Xoshiro512StarStar;

use self::regex_gen::RegexGen;

fn apply_xforms(config: &Config, s: String) -> String {
    let mut s = s;
    for x in &config.xforms {
        s = x.xform(&s);
    }
    s
}

pub fn generate(config: &Config) -> Result<Vec<String>, String> {
    let pat = config.pattern.assemble_pattern(&config.fragments)?;
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
