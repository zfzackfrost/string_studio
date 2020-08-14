use clap::{App, Arg};
use std::str::FromStr;
use string_studio::config::*;
use string_studio::generate::*;
use string_studio::output::*;

fn require_parsed_str<I: FromStr>(v: String, message: &str) -> Result<(), String> {
    if let Ok(_) = v.parse::<I>() {
        Ok(())
    } else {
        Err(String::from(message))
    }
}

fn require_u32_str(v: String) -> Result<(), String> {
    require_parsed_str::<u32>(v, "The value was not an integer or was out of range")
}

fn process_args() -> Result<Config, String> {
    let matches = App::new("String Builder")
        .version("0.1.0")
        .author("Zachary Frost")
        .about("Generate randomized strings")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .value_name("INTEGER")
                .help("Sets the number of strings to generate")
                .takes_value(true)
                .validator(require_u32_str)
                .default_value("12"),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Sets the output format")
                .takes_value(true)
                .possible_value("simple")
                .possible_value("table")
                .possible_value("json")
                .possible_value("csv")
                .default_value("simple"),
        )
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .help("Sets the pattern generate strings from.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity. Repeat to increase level (capped at 2)."),
        )
        .arg(
            Arg::with_name("pretty")
                .long("pretty")
                .help("Use nice formatting when `--format` is `json`"),
        )
        .get_matches();

    {
        let num = matches.value_of("number");
        if let None = num {
            return Err(String::from(
                "No value found for `number`! This should not happen.",
            ));
        }
        let num = num.unwrap().parse::<u32>().unwrap_or(1);
        let format = matches.value_of("format");
        if let None = format {
            return Err(String::from(
                "No value found for `format`! This should not happen.",
            ));
        }
        let format = format.unwrap();

        let verbosity = matches.occurrences_of("verbosity") as u8;

        let pretty = matches.is_present("pretty");

        Ok(Config {
            format: OutputFormat::from(format),
            number: num,
            verbosity: Verbosity::from(verbosity),
            pattern: String::from(matches.value_of("pattern").unwrap()),
            pretty: pretty
        })
    }
}

fn run() -> Result<(), String> {
    let config = process_args()?;

    println_v2(&config, format!("Full Configuration: {}", config).as_str());
    println_v1(
        &config,
        format!("Generating {} strings...", config.number).as_str(),
    );

    let strings = generate(&config)?;

    output(&config, &strings)?;

    Ok(())
}

fn main() {
    let result = run();
    if let Err(err) = result {
        println_err(format!("Error: {}", err).as_str());
        std::process::exit(1);
    }
}
