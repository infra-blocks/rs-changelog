use changelog::Linter;
use clap::{Arg, ArgAction, Command};

fn main() -> eyre::Result<()> {
    let command = Command::new("changelog")
        .author("Phil Lavoie")
        .about("This program offers different tools to manipulate and parse changelogs.")
        .subcommand(
            Command::new("lint").about("Lints a changelog file.").arg(
                Arg::new("file")
                    .short('f')
                    .long("file")
                    .default_value("CHANGELOG.md")
                    .action(ArgAction::Set)
                    .help("The changelog file."),
            ),
        );
    let matches = command.get_matches();

    match matches.subcommand_matches("lint") {
        Some(matches) => {
            let file: &String = matches.get_one("file").unwrap();
            let linter = Linter::new(file.to_string());
            linter.lint()
        }
        _ => Ok(()),
    }
}
