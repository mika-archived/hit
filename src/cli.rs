use clap::{App, Arg};

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
    );
}
