use clap::{App, Arg};
use termcolor::{Color, ParseColorError};

fn color_validator(color: String) -> Result<(), String> {
  let _: Color = color
    .parse()
    .map_err(|e: ParseColorError| e.to_string())
    .unwrap();

  return Ok(());
}

pub fn build() -> App<'static, 'static> {
  return clap::app_from_crate!()
    .arg(
      Arg::with_name("PATTERN")
        .help("A regular expression used for highlighing.")
        .required(true)
        .index(1),
    )
    .arg(
      Arg::with_name("PATH")
        .help("A file to highlight.")
        .required(false)
        .index(2),
    )
    .arg(
      Arg::with_name("COLOR")
        .short("c")
        .long("color")
        .default_value("green")
        .validator(|s| color_validator(s))
        .help("A color for using highlight.")
        .required(false),
    );
}
