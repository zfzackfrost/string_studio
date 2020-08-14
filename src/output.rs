use crate::config::*;
use prettytable::{Cell, Row, Table};

pub fn output_simple(_config: &Config, strings: &Vec<String>) -> Result<(), String> {
    for s in strings {
        println!("{}", s);
    }
    Ok(())
}

pub fn output_table(_config: &Config, strings: &Vec<String>) -> Result<(), String> {
    // Create the table
    let mut table = Table::new();

    let rows_iter = strings.chunks(4);
    for r in rows_iter {
        table.add_row(Row::new(r.iter().map(|x| Cell::new(x)).collect()));
    }

    // Print the table to stdout
    table.printstd();

    Ok(())
}
pub fn output_json(config: &Config, strings: &Vec<String>) -> Result<(), String> {
    let mut value = json::JsonValue::new_array();
    for s in strings {
        let r = value.push(json::JsonValue::from(s.clone()));
        if let Err(_) = r {
            return Err(String::from("Unknown JSON stringify error!"));
        }
    }
    if config.pretty {
        println!("{}", value.pretty(2));
    } else {
        println!("{}", value.dump());
    }
    Ok(())
}
pub fn output_csv(_config: &Config, strings: &Vec<String>) -> Result<(), String> {
    let csv = strings.join(",");
    println!("{}", csv);
    Ok(())
}
pub fn output(config: &Config, strings: &Vec<String>) -> Result<(), String> {
    match config.format {
        OutputFormat::Simple => output_simple(config, strings),
        OutputFormat::Table => output_table(config, strings),
        OutputFormat::Json => output_json(config, strings),
        OutputFormat::Csv => output_csv(config, strings),
    }
}
