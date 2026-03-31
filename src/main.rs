use changelog::parse_file;
use clap::{Arg, Command};

fn main() {
    let command = Command::new("rs-changelog")
        .author("El Pendeloco")
        .about("This program takes in a markdown file name and parses it.")
        .arg(
            Arg::new("file")
                .index(1)
                .help("The markdown file to parse.")
                .required(true),
        )
        .after_help("This program is a work in progress.");
    let matches = command.get_matches();
    let file_path = matches.get_one::<String>("file").unwrap();
    parse_file(file_path);
}
