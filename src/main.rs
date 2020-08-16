use clap::{App, Arg, SubCommand};
use std::str::FromStr;
use string_studio::config::*;
use string_studio::generate::*;
use string_studio::output::*;

use string_studio::xform::Xform;

use std::fs;
use std::path::Path;

#[derive(PartialEq)]
enum AppAction {
    Root,
    Generate,
    DumpConfig,
}

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
fn require_u64_str(v: String) -> Result<(), String> {
    require_parsed_str::<u64>(v, "The value was not an integer or was out of range")
}

fn require_existing_file(v: String) -> Result<(), String> {
    let p = Path::new(&v);
    if !p.is_file() {
        Err(String::from("The value was not a path to an existing file"))
    } else {
        Ok(())
    }
}

fn create_config() -> Result<(), String> {
    if let Some(path) = get_cfg_file_path() {
        if path.is_file() {
            return Ok(());
        }
        let config = Config::default();
        let dir = path.parent();
        if dir.is_none() {
            return Err(String::from(
                "Could not find directory portion of config file path!",
            ));
        }
        let dir = dir.unwrap();
        if !dir.is_dir() {
            if let Err(err) = std::fs::create_dir_all(path.parent().unwrap()) {
                return Err(err.to_string());
            }
        }
        if let Ok(contents) = toml::to_string_pretty(&config) {
            if let Err(err) = std::fs::write(path, contents) {
                return Err(err.to_string());
            }
        } else {
            return Err(String::from("Error serializing default config!"));
        }
    }
    Ok(())
}

fn process_args() -> Result<(AppAction, Config), String> {
    let cfg_path = get_cfg_file_path();

    let matches = App::new("String Studio")
        .version("0.1.0")
        .author("Zachary Frost")
        .about("Generate randomized strings")
        .arg(
            {
                let mut a = Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets the config file to load flag and option values from. Values specified as command line args take priority.")
                    .takes_value(true)
                    .validator(require_existing_file);

                if let Some(cfg_path) = &cfg_path {
                    a = a.default_value(cfg_path.to_str().unwrap())
                }
                a
            }
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity. Repeat to increase level (capped at 2)."),
        )
        .subcommand(SubCommand::with_name("gen")
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
                    .default_value("15"),
            )
            .arg(
                Arg::with_name("seed")
                    .long("seed")
                    .value_name("INTEGER")
                    .help("Sets the random seed to use when generating strings. Set to 0 to auto pick seed.")
                    .takes_value(true)
                    .validator(require_u64_str)
                    .default_value("0"),
            )
            .arg(
                Arg::with_name("xform")
                    .long("xform")
                    .short("x")
                    .value_name("XFORM_NAME")
                    .help("Adds a transformation to the generated strings.")
                    .takes_value(true)
                    .possible_values(&["u_after_q", "title_case"])
                    .multiple(true)
            )
            .arg(
                Arg::with_name("format")
                    .short("f")
                    .long("format")
                    .value_name("FORMAT")
                    .help("Sets the output format")
                    .takes_value(true)
                    .possible_values(if cfg!(feature = "table_format") {
                        &["simple", "table", "json", "csv"]
                    } else {
                        &["simple", "json", "csv"]
                    })
                    .default_value("simple"),
            )
            .arg(
                Arg::with_name("pattern")
                    .value_name("PATTERN")
                    .help("Sets the pattern to generate strings from. If multiple values are supplied, they will be concatenated. Pattern fragments must be separate values (one argument for each fragment).")
                    .required_unless("config")
                    .multiple(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("pretty")
                    .long("pretty")
                    .help("Use nice formatting when `--format` is `json`"),
            )
        )
    .get_matches();

    let verbosity = matches.occurrences_of("verbosity") as u8;

    let action: AppAction = match matches.subcommand_name() {
        Some("gen") => AppAction::Generate,
        Some("dump_cfg") => AppAction::DumpConfig,
        _ => AppAction::Root,
    };

    if let Some(sub_matches) = matches.subcommand_matches("gen") {
        let num = sub_matches.value_of("number");
        if let None = num {
            return Err(String::from(
                "No value found for `number`! This should not happen.",
            ));
        }
        let num = num.unwrap().parse::<u32>().unwrap_or(1);
        let format = sub_matches.value_of("format");
        if let None = format {
            return Err(String::from(
                "No value found for `format`! This should not happen.",
            ));
        }

        let pretty = sub_matches.is_present("pretty");
        let pattern = sub_matches.values_of_lossy("pattern").unwrap_or(vec![]);
        let xforms: Vec<Xform> = sub_matches
            .values_of_lossy("xform")
            .unwrap_or(vec![])
            .iter()
            .map(|x| {
                let s = x.as_str();
                match Xform::try_from(s) {
                    Ok(x) => x,
                    Err(s) => panic!(s),
                }
            })
            .collect();

        let seed = sub_matches
            .value_of("seed")
            .unwrap_or("")
            .parse::<u64>()
            .unwrap_or(0);

        let cmd_config = Config {
            format: OutputFormat::from(format.unwrap()),
            number: num,
            verbosity: Verbosity::from(verbosity),
            pattern: pattern.clone(),
            pretty: pretty,
            fragments: Default::default(),
            seed,
            xforms: Default::default(),
        };
        let cfg = if let Some(cfg_path) = matches.value_of("config") {
            if let Ok(s) = fs::read_to_string(cfg_path) {
                if let Ok(mut c) = toml::from_str::<Config>(s.as_str()) {
                    c.verbosity = Verbosity::from(verbosity); // Ignore verbosity in config file
                    c.pattern = pattern; // Ignore pattern in config file
                    c.seed = seed; // Ignore seed in config file
                    c.xforms = xforms;
                    c.number = num;

                    if sub_matches.occurrences_of("format") > 0 {
                        c.format = OutputFormat::from(format.unwrap());
                    }
                    if sub_matches.is_present("pretty") {
                        c.pretty = true;
                    }

                    c
                } else {
                    return Err(String::from("Failed to parse config file!"));
                }
            } else {
                cmd_config
            }
        } else {
            cmd_config
        };

        Ok((action, cfg))
    } else {
        Err(String::from("Failed to process cmd args!"))
    }
}

fn run() -> Result<(), String> {
    create_config()?;
    let (action, config) = process_args()?;

    match action {
        AppAction::Generate => {
            println_v2(&config, format!("Full Configuration: {}", config).as_str());
            println_v1(
                &config,
                format!("Generating {} strings...\n", config.number).as_str(),
            );
            let strings = generate(&config)?;
            output(&config, &strings)?;
        }
        AppAction::DumpConfig => {}
        _ => {}
    }

    Ok(())
}

fn main() {
    let result = run();
    if let Err(err) = result {
        println_err(format!("Error: {}", err).as_str());
        std::process::exit(1);
    }
}
