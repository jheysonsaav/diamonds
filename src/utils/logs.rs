use std::env;

use colored::*;

#[allow(dead_code)]
pub struct Log {
    message: String,
}

#[allow(dead_code)]
impl Log {
    pub fn error(message: &str) -> Self {
        eprintln!("{} {}", "Error:".bold().red(), message);

        Self {
            message: String::from(message),
        }
    }

    pub fn warning(message: &str) -> Self {
        println!("{} {}", "Warning:".bold().yellow(), message);

        Self {
            message: String::from(message),
        }
    }

    pub fn info(message: &str) -> Self {
        println!("{} {}", "Info:".bold().green(), message);

        Self {
            message: String::from(message),
        }
    }

    pub fn debug(message: &str) -> Self {
        let log_level = match env::var("LOG_LEVEL") {
            Ok(value) => value,
            Err(_) => String::from("info"),
        };

        if log_level.as_str() == "debug" {
            println!("{} {}", "Debug:".bold().blue(), message);
        };

        Self {
            message: String::from(message),
        }
    }
}
