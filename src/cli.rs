use clap::{App, Arg};
use regex::{Error, Regex};
use termcolor::{Color, ParseColorError};

fn regexp_validator(value: String) -> Result<(), String> {
  let _: Regex = value.parse().map_err(|e: Error| e.to_string()).unwrap();

  return Ok(());
}

fn color_validator(value: String) -> Result<(), String> {
  let _: Color = value
    .parse()
    .map_err(|e: ParseColorError| e.to_string())
    .unwrap();

  return Ok(());
}

pub fn build() -> App<'static, 'static> {
  return clap::app_from_crate!()
    .arg(
      Arg::with_name("pattern")
        .value_name("PATTERN")
        .help("A regular expression used for highlighting.")
        .takes_value(true)
        .empty_values(false)
        .required(false)
        // .required_unless("regexp")
        // .conflicts_with("regexp")
        .index(1),
    )
    .arg(
      Arg::with_name("path")
        .value_name("PATH")
        .help("A file to highlight.")
        .takes_value(true)
        .required(false)
        .index(2),
    )
    .arg(
      Arg::with_name("regexp")
        .value_name("REGEXP")
        .short("e")
        .long("regexp")
        .alias("pattern")
        .takes_value(true)
        .empty_values(false)
        // .conflicts_with("pattern")
        .multiple(true)
        .require_delimiter(true)
        .validator(|s| regexp_validator(s))
        .help("A pattern to search for."),
    )
    .arg(
      Arg::with_name("color")
        .value_name("COLOR")
        .short("c")
        .long("color")
        .takes_value(true)
        .empty_values(false)
        .multiple(true)
        .require_delimiter(true)
        .validator(|s| color_validator(s))
        .help("A color for using highlight."),
    );
}
