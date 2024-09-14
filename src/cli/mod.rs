mod csv;
mod genpass;
mod base64;
mod http;

use clap::Parser;
use self::{csv::CsvOpts, genpass::GenPassOpts};
pub use self::{
    csv::OutputFormat,
    base64::Base64SubCommand,
    base64::Base64Format,
    http::HttpSubCommand
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}


fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // Check if intut is "-" or file exists
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
