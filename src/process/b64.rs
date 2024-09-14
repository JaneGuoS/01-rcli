use  crate::Base64Format;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};

use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read>= if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE.encode(buf),
    };
    println!("{}", encoded.trim());
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut read: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    let mut buf = String::new();
    read.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf),
        Base64Format::UrlSafe => URL_SAFE.decode(buf),
    };

    //TODO: decoded data might not be string ( but for now we assume it is)
    let decoded = String::from_utf8(decoded?)?;
    println!("{:?}", decoded);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_process_encode() -> anyhow::Result<()> {
        let mut file = NamedTempFile::new()?;
        write!(file, "Hello, World!")?;
        let filename = file.path().to_str().unwrap();
        process_encode(filename, Base64Format::Standard)?;
        process_encode(filename, Base64Format::UrlSafe)?;
        Ok(())
    }

    #[test]
    fn test_process_decode() -> anyhow::Result<()> {
        let mut file = NamedTempFile::new()?;
        write!(file, "SGVsbG8gV29ybGQK")?;
        let filename = file.path().to_str().unwrap();
        process_decode(filename, Base64Format::Standard)?;
        process_decode(filename, Base64Format::UrlSafe)?;
        Ok(())
    }
}
