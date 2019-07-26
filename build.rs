#[macro_use]
extern crate clap;

use std::env;
use clap::Shell;

#[path = "src/cli.rs"]
mod cli;

fn main() {
  let out_dir = match env::var_os("OUT_DIR") {
    None => return,
    Some(val) => val
  };

  let mut app = cli::build();
  app.gen_completions("hit", Shell::Bash,       &out_dir);
  app.gen_completions("hit", Shell::Fish,       &out_dir);
  app.gen_completions("hit", Shell::Zsh,        &out_dir);
  app.gen_completions("hiy", Shell::PowerShell, &out_dir);;
}