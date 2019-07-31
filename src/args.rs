use clap::ArgMatches;
use regex::Regex;
use termcolor::Color;

pub struct Args {
  pub colors: Vec<Color>,
  pub patterns: Vec<Regex>,
}

impl Args {
  pub fn parse(matches: &ArgMatches<'static>) -> Result<Args, String> {
    let mut patterns: Vec<Regex> = Vec::new();

    // <PATTERN>
    if matches.is_present("pattern") {
      let pattern = match matches.value_of("pattern").unwrap().parse::<Regex>() {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
      };

      patterns.push(pattern);
    }

    // -e or --regexp
    if matches.is_present("regexp") {
      let regexps: Vec<_> = matches.values_of("regexp").unwrap().collect();
      for regexp in regexps {
        let pattern = match regexp.parse::<Regex>() {
          Ok(r) => r,
          Err(e) => return Err(e.to_string()),
        };

        patterns.push(pattern);
      }
    }

    let mut colors: Vec<Color> = Vec::new();

    // -c or --color
    if matches.is_present("color") {
      let cls: Vec<_> = matches.values_of("color").unwrap().collect();
      for clr in cls {
        let color = match clr.parse::<Color>() {
          Ok(c) => c,
          Err(e) => return Err(e.to_string()),
        };

        colors.push(color);
      }
    } else {
      colors.push(Color::Green);
    }

    return Ok(Args {
      colors: colors,
      patterns: patterns,
    });
  }

  // #[inline]
  // pub fn colors(&self) -> Vec<Color> {
  //   return self.colors;
  // }

  // #[inline]
  // pub fn patterns(&self) -> Vec<Regex> {
  //   return self.patterns;
  // }
}
