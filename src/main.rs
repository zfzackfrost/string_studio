use string_studio::generate::*;
use string_studio::output::*;
use string_studio::config::*;

use string_studio::cmdargs::*;

fn create_config(overwrite: bool) -> Result<(), String> {
    if let Some(path) = get_cfg_file_path() {
        if path.is_file() && !overwrite {
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


fn run() -> Result<(), String> {
    create_config(false)?;
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
        AppAction::DumpConfig => {
            create_config(true)?;
        }
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
