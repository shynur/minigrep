use crate::Config;

mod search_tools;
use search_tools::*;

pub fn parse_config() -> Config {
    Config::new(std::env::args()).unwrap_or_else(|err| {
        use std::process;
        eprintln!("\nProblem parsing arguments:\n{}", err);
        process::exit(1);
    })
}
fn read_contents(fname: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::fs;
    Ok(fs::read_to_string(fname)?)
}
pub fn init_work_ui(config: Config) -> (Config, String) {
    println!("Searching for");
    session_box(config.str());
    println!("In file {}", config.fname());
    match read_contents(config.fname()) {
        Ok(contents) => {
            println!("With text:");
            session_box(&contents);
            (config, contents)
        }
        Err(e) => {
            use std::process;
            eprintln!("\nApplication error:\n{}", e);
            process::exit(1);
        }
    }
}
fn session_box(paragraph: &str) {
    let wide_underline = "________________________________________";
    let wide_overline = "‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾";
    if paragraph.len() == 0 {
        println!("{}\n{}", wide_underline, wide_overline);
    } else {
        if paragraph.as_bytes()[paragraph.len() - 1] == b'\n' {
            println!("{}\n{}{}", wide_underline, paragraph, wide_overline)
        } else {
            println!("{}\n{}\n{}", wide_underline, paragraph, wide_overline)
        }
    }
}

pub fn show_dest_lines(query: &str, contents: &str, case_sensitive: bool) -> (usize, usize, usize) {
    let dest_lines = if case_sensitive {
        search(query, contents)
    } else {
        search_case_insensitive(query, contents)
    };
    let total_rows_num = contents.lines().count();
    let amount_rows = dest_lines.len();
    let mut str_frequency = 0;
    if amount_rows == 0 {
        println!("No target row found!\n")
    } else {
        println!("Target row(s) found:");
        session_box("");
        for line in dest_lines {
            println!("{}", line);
            str_frequency += if case_sensitive {
                line.matches(query).count()
            } else {
                line.to_lowercase()
                    .matches(query.to_lowercase().as_str())
                    .count()
            };
        }
        session_box("");
    }
    (total_rows_num, amount_rows, str_frequency)
}
