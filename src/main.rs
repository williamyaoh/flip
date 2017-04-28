//! Simple Unicode-aware command-line utility to reverse all the characters
//! in each line.

//! If given files as arguments, reverses the contents of each line of
//! each file and prints the results to stdout. Otherwise, does the same
//! to stdin.

extern crate rustc_serialize;
extern crate unicode_segmentation;
extern crate docopt;
extern crate arg_input;

use unicode_segmentation::UnicodeSegmentation;
use docopt::{Docopt, Error};

use std::io::stderr;
use std::io::Write;
use std::process;

static VERSION: &'static str = "1.0.2";
macro_rules! VERSION_INFO {
  () => { "\
flip {}
copyright (c) 2017 William Yao <williamyaoh@gmail.com>
license BSD 3-Clause
no warranty, whether implied or not
" }
}
static USAGE: &'static str = "
flip -- reverse characters in each line

Usage:
    flip [<files>...]
    flip (--help | --version)

Options:
    -h, --help              Display this help message
    --version               Display version info

Like `cat', but reverses the characters in each line before printing them
to stdout. Unicode-aware.
";

#[derive(RustcDecodable)]
struct CLIArgs {
  arg_files: Vec<String>
}

fn main() {
  let cli_parser = Docopt::new(USAGE).unwrap()
    .version(Some(VERSION.to_string()))
    .help(true);

  let cli_args: CLIArgs = cli_parser.decode().map_err(|err| match err { 
    Error::Version(version) => {
      print!(VERSION_INFO!(), version);
      process::exit(0);
    },
    other => other.exit()
  }).unwrap();

  if let Err(msg) = go(cli_args.arg_files) {
    writeln!(stderr(), "flip: {}", msg)
      .expect("failed to write exit message to stderr");
    process::exit(1);
  }
}

fn go(files: Vec<String>) -> Result<(), String> {
  let lines = arg_input::input_lines(files).map_err(|_err| {
    "failed to open one or more files".to_string()
  })?;

  for (line_number, line) in lines.enumerate().map(|(idx, obj)| (idx + 1, obj)) {
    let line = line.map_err(|_err| format!("failed to reverse line# {}", line_number))?;
    let graphemes = UnicodeSegmentation::graphemes(&line as &str, true);

    for grapheme in graphemes.rev() {
      print!("{}", grapheme);
    }

    println!();
  }

  Ok(())
}

