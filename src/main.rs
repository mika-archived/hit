extern crate atty;
#[macro_use]
extern crate clap;
extern crate regex;
extern crate termcolor;

use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read, Write};
use std::path::Path;
use std::str::from_utf8;

use atty::Stream;
use regex::Regex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod cli;

fn main() -> Result<(), String> {
    let matches = cli::build().get_matches();

    let pattern = matches.value_of("PATTERN").unwrap(); // required, `unwrap` is safe
    let pattern = Regex::new(pattern).map_err(|e| e.to_string())?; // regex:Error to String

    if let Some(path) = matches.value_of("PATH") {
        return read_from_file(path, pattern);
    } else if !atty::is(Stream::Stdin) {
        return read_from_pipe(pattern);
    }

    return Ok(());
}

fn read_from_file(path: &str, pattern: Regex) -> Result<(), String> {
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

    while reader.read_line(&mut buffer).map_err(|e| e.to_string())? > 0 {
        print_line(&buffer, &pattern)?;
        buffer.clear();
    }

    return Ok(());
}

fn read_from_pipe(pattern: Regex) -> Result<(), String> {
    let stdin = stdin();
    let mut lockd = stdin.lock();
    let mut buffer = [0u8; 1024];

    loop {
        match lockd.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let string = from_utf8(&buffer[0..n]).map_err(|e| e.to_string())?;
                print_line(&string, &pattern)?;
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    return Ok(());
}

fn print_line(buffer: &str, pattern: &Regex) -> Result<(), String> {
    if !pattern.is_match(&buffer) {
        print!("{}", buffer);
        return Ok(());
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut color = ColorSpec::new();
    let mut pos = 0;

    for mat in pattern.find_iter(&buffer) {
        if pos < mat.start() {
            write!(&mut stdout, "{}", &buffer[pos..mat.start()]).unwrap();
        }

        color.set_fg(Some(Color::Green));
        color.set_bold(true);
        stdout.set_color(&mut color).map_err(|e| e.to_string())?;
        write!(&mut stdout, "{}", &buffer[mat.start()..mat.end()]).unwrap();

        color.clear();
        stdout.set_color(&mut color).map_err(|e| e.to_string())?;
        pos = mat.end();
    }

    if pos < buffer.len() {
        write!(&mut stdout, "{}", &buffer[pos..]).unwrap();
    }

    return Ok(());
}
