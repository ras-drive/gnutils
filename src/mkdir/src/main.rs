use std::error::Error;
use std::path::Path;
use std::fs::{create_dir};
use clap::{Arg, ArgMatches, Command};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("mkdir")
        .version("0.1.0")
        .author("Sarah Petkovic")
        .about("create the directory if it doesn't already exist")
        .arg(Arg::new("dirname").required(true))
        .get_matches();

    run(matches)
}

fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let dirname = &matches.value_of("dirname").unwrap().to_string();

    if Path::exists(Path::new(dirname)) {
        panic!("mkdir: cannot create directory ‘{}’: File exists", dirname)
    }

    Ok(())
}