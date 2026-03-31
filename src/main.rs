use std::path::Path;

use changelog::{debug, parse};
use clap::{arg, Arg, Command};
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let command = Command::new("rs-changelog")
        .author("El Pendeloco")
        .subcommand(
            Command::new("lint")
                .about("This program takes in a markdown file name and parses it.")
                .arg(
                    Arg::new("file")
                        .index(1)
                        .help("The markdown file to parse.")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("debug")
                .about("This command outputs the result of running the file through pulldown-cmark")
                .arg(arg!(<file> "The markdown file to parse.")),
        )
        .after_help("This program is a work in progress.");
    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("lint", sub)) => {
            let file = sub.get_one::<String>("file").unwrap();
            let content = read_file(file)?;
            parse(&content);
        }
        Some(("debug", sub)) => {
            let file = sub.get_one::<String>("file").unwrap();
            let content = read_file(file)?;
            debug(&content);
        }
        Some((unknown, _)) => panic!("unknown subcommand: {}", unknown),
        None => panic!("unexpected lack of subcommand"),
    };
    Ok(())
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    std::fs::read_to_string(&path).into_diagnostic()
}
