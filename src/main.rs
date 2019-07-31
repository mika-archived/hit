extern crate atty;
#[macro_use]
extern crate clap;
extern crate regex;
extern crate termcolor;

mod args;
mod cli;

use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::path::Path;

use args::Args;
use atty::Stream;
use regex::{Match as RegexMatch, Regex};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

struct Pattern<'t> {
    pattern: &'t Regex,
    color: Color,
}

impl<'t> Pattern<'t> {
    fn new(pattern: &'t Regex, color: Color) -> Pattern<'t> {
        return Pattern {
            pattern: pattern,
            color: color,
        };
    }
}

struct Match<'t> {
    start: usize,
    end: usize,
    pattern: &'t Pattern<'t>,
}

impl<'t> Match<'t> {
    fn new(pattern: &'t Pattern<'t>, mat: RegexMatch) -> Match<'t> {
        return Match {
            pattern: pattern,
            start: mat.start(),
            end: mat.end(),
        };
    }

    fn start(&self) -> usize {
        return self.start;
    }

    fn end(&self) -> usize {
        return self.end;
    }

    fn color(&self) -> Color {
        return self.pattern.color;
    }
}

fn main() -> Result<(), String> {
    let matches = cli::build().get_matches();
    let args = Args::parse(&matches).unwrap(); // not safe, but throw String

    if let Some(path) = matches.value_of("path") {
        return read_from_file(&path, args);
    } else if !atty::is(Stream::Stdin) {
        return read_from_pipe(args);
    } else {
        return Err("stdin or <PATH> argument were not provided".to_string());
    }
}

fn read_from_file(path: &str, args: Args) -> Result<(), String> {
    let path = Path::new(path);
    if !path.exists() || !path.is_file() {
        return Err(format!(
            "could not access to the file: {}",
            path.to_str().unwrap()
        ));
    }

    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);

    return read_line(args, &mut reader);
}

fn read_from_pipe(args: Args) -> Result<(), String> {
    let stdin = stdin();
    let mut reader = stdin.lock();

    return read_line(args, &mut reader);
}

fn read_line(args: Args, reader: &mut BufRead) -> Result<(), String> {
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).map_err(|e| e.to_string())? > 0 {
        print_line(&args, &buffer)?;
        buffer.clear();
    }

    return Ok(());
}

// buffer &str ('buffer) is alive while this function is running.
fn print_line<'buffer>(args: &Args, buffer: &'buffer str) -> Result<(), String> {
    // if found matched pattern(s), processing colors.
    if let Some(patterns) = matched_patterns(&args.patterns, &buffer) {
        let patterns = dispatch_colors(patterns, &args.colors);
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut clspec = ColorSpec::new();
        let mut cursor = 0;

        let matches = find_matches(&patterns, &buffer);
        for mat in matches {
            // if cursor is ahead of mat.start(), ignore this match
            if cursor > mat.start() {
                continue;
            }
            if cursor < mat.start() {
                write!(&mut stdout, "{}", &buffer[cursor..mat.start()]).unwrap();
            }

            clspec.set_fg(Some(mat.color()));
            clspec.set_bold(true);
            stdout.set_color(&mut clspec).map_err(|e| e.to_string())?;
            write!(&mut stdout, "{}", &buffer[mat.start()..mat.end()]).unwrap();

            // reset colors
            clspec.clear();
            stdout.set_color(&mut clspec).map_err(|e| e.to_string())?;

            cursor = mat.end();
        }

        if cursor < buffer.len() {
            write!(&mut stdout, "{}", &buffer[cursor..]).unwrap();
        }

        stdout.flush().unwrap();
    } else {
        let stdout = stdout();
        let mut locked = stdout.lock();

        write!(&mut locked, "{}", buffer).unwrap();
        locked.flush().unwrap();
    }

    return Ok(());
}

fn matched_patterns<'pattern>(
    patterns: &'pattern Vec<Regex>,
    buffer: &str,
) -> Option<Vec<(&'pattern Regex, usize)>> {
    let mut matches: Vec<(&'pattern Regex, usize)> = Vec::new();

    for (i, pattern) in patterns.iter().enumerate() {
        if pattern.is_match(&buffer) {
            matches.push((pattern, i));
        }
    }

    return if matches.len() == 0 {
        None
    } else {
        Some(matches)
    };
}

fn dispatch_colors<'t>(values: Vec<(&'t Regex, usize)>, colors: &Vec<Color>) -> Vec<Pattern<'t>> {
    let mut patterns: Vec<Pattern<'t>> = Vec::new();

    for value in values {
        if value.1 >= colors.len() {
            patterns.push(Pattern::new(value.0, Color::Green)); // use default color
        } else {
            patterns.push(Pattern::new(value.0, colors[value.1]));
        }
    }

    return patterns;
}

fn find_matches<'t>(patterns: &'t Vec<Pattern>, buffer: &'t str) -> Vec<Match<'t>> {
    let mut matches: Vec<Match<'t>> = Vec::new();

    for pattern in patterns {
        for mat in pattern.pattern.find_iter(&buffer) {
            matches.push(Match::new(pattern, mat));
        }
    }

    matches.sort_by_key(|v| v.start());
    return matches;
}
