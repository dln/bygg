extern crate rustc_serialize;
extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate toml;

use std::env;
use std::io::{Error, ErrorKind, Result};
use std::io::prelude::*;
use std::fs::File;
use docopt::Docopt;


const USAGE: &'static str = "
Bygg.

Usage:
  bygg [-h | --help] [-v | --verbose] [--manifest=<path>]

Options:
  -h --help          Show this help
  -v --verbose       Loudness
  --manifest=<path>  Path to manifest file [default: bygg.toml]
";

fn read_manifest(path: String) -> Result<toml::Table> {
    let mut f = try!(File::open(path));
    let mut buffer = String::new();
    try!(f.read_to_string(&mut buffer));

    let mut parser = toml::Parser::new(&buffer);
    parser.parse()
          .ok_or(Error::new(ErrorKind::Other, "Parse error"))
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_manifest: String,
    flag_verbose: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_verbose {
        env::set_var("RUST_LOG", "trace")
    }

    env_logger::init().unwrap();

    info!("Starting");
 
    match read_manifest(args.flag_manifest) {
        Ok(value) => info!("found manifest: {:?}", value),
        Err(err)  => error!("{:?}", err)
    }

}
