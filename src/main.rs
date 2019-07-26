extern crate atty;
#[macro_use]
extern crate clap;
extern crate regex;
extern crate termcolor;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use regex::Regex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod cli;

fn main() -> Result<(), String> {
    let matches = cli::build().get_matches();

    let pattern = matches.value_of("PATTERN").unwrap(); // required, `unwrap` is safe
    let pattern = Regex::new(pattern).map_err(|e| e.to_string())?; // regex:Error to String

    if let Some(path) = matches.value_of("PATH") {
        return read_from_file(path, pattern);
    }

    return Ok(());
}

fn read_from_file(path: &str, pattern: regex::Regex) -> Result<(), String> {
    let path = Path::new(path);
    if !path.exists() || !path.is_file() {
        return Err(format!(
            "could not access to the file: {}",
            path.to_str().unwrap()
        ));
    }

    let file = File::open(path).map_err(|e| e.to_string())?; // std::io::Error to String
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut color = ColorSpec::new();

    while reader.read_line(&mut buffer).map_err(|e| e.to_string())? > 0 {
        if pattern.is_match(&buffer) {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            let mut pos = 0;

            for mat in pattern.find_iter(&buffer) {
                if pos <= mat.start() {
                    // write buffer
                    let _ = write!(&mut stdout, "{}", &buffer[pos..mat.start()]);
                }

                // highlight keywords
                color.set_fg(Some(Color::Green));
                color.set_bold(true);
                stdout.set_color(&mut color).map_err(|e| e.to_string())?;
                let _ = write!(&mut stdout, "{}", &buffer[mat.start()..mat.end()]);

                // reset color
                color.clear();
                stdout.set_color(&mut color).map_err(|e| e.to_string())?;
                pos = mat.end();
            }

            if pos < buffer.len() {
                let _ = write!(&mut stdout, "{}", &buffer[pos..]);
            }
        } else {
            print!("{}", buffer);
        }
        buffer.clear();
    }

    return Ok(());
}
