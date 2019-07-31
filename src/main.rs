extern crate atty;
#[macro_use]
extern crate clap;
extern crate regex;
extern crate termcolor;

use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::path::Path;

use atty::Stream;
use regex::Regex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod cli;

fn main() -> Result<(), String> {
    let matches = cli::build().get_matches();

    let pattern = matches.value_of("PATTERN").unwrap(); // required, `unwrap` is safe
    let pattern = Regex::new(pattern).map_err(|e| e.to_string())?; // regex:Error to String
    let color = matches.value_of("COLOR").unwrap();
    let color: Color = color.parse().unwrap();

    if let Some(path) = matches.value_of("PATH") {
        return read_from_file(path, pattern, color);
    } else if !atty::is(Stream::Stdin) {
        return read_from_pipe(pattern, color);
    }

    return Ok(());
}

fn read_from_file(path: &str, pattern: Regex, color: Color) -> Result<(), String> {
    let path = Path::new(path);
    if !path.exists() || !path.is_file() {
        return Err(format!(
            "could not access to the file: {}",
            path.to_str().unwrap()
        ));
    }

    let file = File::open(path).map_err(|e| e.to_string())?; // std::io::Error to String
    let mut reader = BufReader::new(file);

    return read_line(&mut reader, pattern, color);
}

fn read_from_pipe(pattern: Regex, color: Color) -> Result<(), String> {
    let stdin = stdin();
    let mut reader = stdin.lock();

    return read_line(&mut reader, pattern, color);
}

fn read_line(reader: &mut BufRead, pattern: Regex, color: Color) -> Result<(), String> {
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).map_err(|e| e.to_string())? > 0 {
        print_line(&buffer, &pattern, color)?;
        buffer.clear();
    }

    return Ok(());
}

fn print_line(buffer: &str, pattern: &Regex, clr: Color) -> Result<(), String> {
    if !pattern.is_match(&buffer) {
        let stdout = stdout();
        let mut locked = stdout.lock();

        write!(&mut locked, "{}", buffer).unwrap();
        locked.flush().unwrap();
        return Ok(());
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut color = ColorSpec::new();
    let mut pos = 0;

    for mat in pattern.find_iter(&buffer) {
        if pos < mat.start() {
            write!(&mut stdout, "{}", &buffer[pos..mat.start()]).unwrap();
        }

        color.set_fg(Some(clr));
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

    stdout.flush().unwrap();

    return Ok(());
}
