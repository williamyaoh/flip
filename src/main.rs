//! Simple Unicode-aware command-line utility to reverse all the characters
//! in each line.

//! If given files as arguments, reverses the contents of each line of
//! each file and prints the results to stdout. Otherwise, does the same
//! to stdin.

extern crate rustc_serialize;
extern crate unicode_segmentation;
extern crate docopt;

use unicode_segmentation::UnicodeSegmentation;
use docopt::Docopt;

use std::io::{stdin, stderr};
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::process;

static VERSION: &'static str = "1.0.0";
static USAGE: &'static str = "
flip -- reverse characters in each line

Usage:
    flip <files>...
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

/// A bunch of Reads collapsed together, in the manner that chain() would.
struct Multichain<R> {
  streams: Vec<R>,
  current: usize
}

impl<R> Multichain<R> where
  R: Read {
  fn new(streams: Vec<R>) -> Self 
  {
    Multichain { streams: streams, current: 0 }
  }
}

impl<R> Read for Multichain<R> where 
  R: Read 
{
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
  {
    let mut bytes_written: usize = 0;
    let max_bytes = buf.len();

    while bytes_written < max_bytes && self.current < self.streams.len() {
      let here = &mut buf[bytes_written..];
      let bytes = self.streams[self.current].read(here)?;

      if bytes == 0 {
        self.current += 1;
      } else {
        bytes_written += bytes;
      }
    }

    Ok(bytes_written)
  }
}

fn main() {
  let cli_parser = Docopt::new(USAGE).unwrap()
    .version(Some(VERSION.to_string()))
    .help(true);

  let cli_args: CLIArgs = cli_parser.decode().map_err(|err| err.exit())
    .unwrap();

  if let Err(msg) = go(cli_args.arg_files) {
    writeln!(stderr(), "flip: {}", msg)
      .expect("failed to write exit message to stderr");
    process::exit(1);
  }
}

/// Map the function over the given iterator, propagating an error immediately
/// if any of the mappings fail.
/// We assume that the given iterator is finite. We can't exactly check
/// for an error *somewhere* in an infinite list :V
/// Returns the first error found, if there is one.
fn attempt_map<T, E, F, I, M>(elements: I, mapper: F) -> Result<Vec<T>, E> where
  F: FnMut(M) -> Result<T, E>,
  I: IntoIterator<Item=M>
{
  let mapped: Vec<Result<T, E>> = elements.into_iter().map(mapper).collect();
  let mut results = Vec::with_capacity(mapped.len());

  for result in mapped {
    match result {
      Ok(obj) => results.push(obj),
      Err(e) => return Err(e)
    }
  }

  Ok(results)
}

fn go(files: Vec<String>) -> Result<(), String> {
  let input: Box<Read>;

  if files.is_empty() {
    input = Box::new(stdin());
  } else {
    let files = attempt_map(files, |path| File::open(path)).map_err(|_err| {
      "one or more files could not be opened (are you sure they exist?)"
    })?;

    input = Box::new(Multichain::new(files));
  }

  reverse_lines(input)
}

fn reverse_lines(input: Box<Read>) -> Result<(), String> {
  let buf = BufReader::new(input);
  let lines = buf.lines();

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

#[cfg(test)]
mod flip_tests {
  fn make_result(truthity: &bool) -> Result<(), ()> {
    if *truthity { Ok(()) } else { Err(()) }
  }

  #[test]
  fn test_attempt_try_success() {
    let success = [true; 32];
    let results = super::attempt_map(&success, &make_result);

    let unpacked = results.expect("attempt_map() returned an Err when there were only Ok");

    assert_eq!(unpacked.len(), 32);
  }

  #[test]
  fn test_attempt_try_failure() {
    let attempts = [true, true, false, true, true];
    let results = super::attempt_map(&attempts, &make_result);

    assert!(results.is_err());
  }

  struct ReadableStr<'a> {
    contents: &'a [u8],
    needle: usize
  }

  impl<'a> ReadableStr<'a> {
    fn new(contents: &'a str) -> Self {
      ReadableStr {
        contents: contents.as_bytes(),
        needle: 0
      }
    }
  }

  use ::std::io::{self, Read};

  impl<'a> Read for ReadableStr<'a> {
    fn read(& mut self, buf: &mut [u8]) -> io::Result<usize> {
      let mut i = 0;
      let max_bytes = buf.len();
      
      while i < max_bytes && i + self.needle < self.contents.len() {
        buf[i] = self.contents[i + self.needle];
        i += 1;
      }

      self.needle += i;

      Ok(i)
    }
  }

  /// This one is just to make sure our Read impl is correct for ReadableStr.
  #[test]
  fn test_read_instance() {
    let str = "It is a truth universally acknowledged";
    let mut readable = ReadableStr::new(str);

    let mut result_string = String::new();
    let result = readable.read_to_string(&mut result_string);

    assert!(result.is_ok());
    assert_eq!(str, &result_string);
  }

  #[test]
  fn test_multichain() {
    let str1 = "In my younger and more vulnerable years my father gave me some advice";
    let str2 = "Whenever you feel like criticizing any one, just remember that all";
    let str3 = "the people in the world haven't had the advantages that you've had";

    let mut joined = String::new();

    joined = joined + str1 + str2 + str3;

    let str1 = ReadableStr::new(str1);
    let str2 = ReadableStr::new(str2);
    let str3 = ReadableStr::new(str3);

    let readers = vec![str1, str2, str3];
    let mut chain = ::Multichain::new(readers);

    let mut result_string = String::new();
    let result = chain.read_to_string(&mut result_string);

    assert!(result.is_ok());
    assert_eq!(result_string, joined);
  }

  #[test]
  fn test_multichain_partial_read() {
    let str1 = "In my younger and more vulnerable years my father gave me some advice";
    let str2 = "Whenever you feel like criticizing any one, just remember that all";
    let str3 = "the people in the world haven't had the advantages that you've had";

    let str1 = ReadableStr::new(str1);
    let str2 = ReadableStr::new(str2);
    let str3 = ReadableStr::new(str3);

    let readers = vec![str1, str2, str3];
    let mut chain = ::Multichain::new(readers);

    let mut result_buf = [0; 4];
    let result = chain.read(&mut result_buf);

    assert!(result.is_ok());
    assert_eq!(result_buf, [b'I', b'n', b' ', b'm']);

    let result = chain.read(&mut result_buf);

    assert!(result.is_ok());
    assert_eq!(result_buf, [b'y', b' ', b'y', b'o']);
  }
}
