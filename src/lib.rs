pub struct Config {
    filename: String,
    query: String,
    case_sensitive: bool,
}
impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        use std::env;
        args.next();
        if let Some(filename) = args.next() {
            if let Some(query) = args.next() {
                if let Ok(val) = env::var("CASE_SENSITIVE") {
                    if val != "0" {
                        return Ok(Config {
                            filename,
                            query,
                            case_sensitive: true,
                        });
                    }
                }
                Ok(Config {
                    filename,
                    query,
                    case_sensitive: false,
                })
            } else {
                Err("invalid string\n")
            }
        } else {
            Err("invalid filename\n")
        }
    }
    pub fn fname(&self) -> &str {
        &self.filename[..]
    }
    pub fn str(&self) -> &str {
        &self.query[..]
    }
    pub fn case_sensitive(&self) -> bool {
        self.case_sensitive
    }
}

pub mod tasks;
pub use tasks::{init_work_ui, parse_config, show_dest_lines};

#[cfg(test)]
mod tests;
