use rand::{SeedableRng, Rng};
use crate::config::Config;

pub fn generate(config: &Config) -> Result<Vec<String>, String> {
    let mut rng = rand_xorshift::XorShiftRng::from_entropy();
    let gen = rand_regex::Regex::compile(config.pattern.as_str(), 100);

    if let Ok(gen) = gen {
        let strings: Vec<_> =(&mut rng).sample_iter(&gen).take(config.number as usize).collect();
        Ok(strings)
    } else {
        Err(String::from("Invalid pattern!"))
    }
}
