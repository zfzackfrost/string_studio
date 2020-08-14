use crate::config::Config;
use rand::{Rng, SeedableRng};

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
    let mut rng = rand_xorshift::XorShiftRng::from_entropy();
    let gen = rand_regex::Regex::compile(&pat, 100);

    if let Ok(gen) = gen {
        let strings: Vec<_> = (&mut rng)
            .sample_iter(&gen)
            .take(config.number as usize)
            .collect();
        Ok(strings)
    } else {
        Err(String::from("Invalid pattern!"))
    }
}
