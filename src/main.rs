use clap::{Arg, ArgAction, Command};
use rust_cli_template::{do_stuff, Config};

fn main() {
    let command = Command::new("rust-cli-template")
        .author("Phil Lavoie")
        .about("This program doesn't do shit.")
        .arg(
            Arg::new("positional")
                .index(1)
                .help("A positional argument that ain't doin' shit")
                .required(true),
        )
        .arg(
            Arg::new("flag")
                .short('f')
                .long("flag")
                .help("A flag (without value) that ain't doin' shit.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .help("An option (with a value) that ain't doin' shit.")
                .action(ArgAction::Set),
        )
        .after_help(
            "This program holds the boilerplate, template code for rust binaries. \
        It isn't meant to be used on its own.",
        );
    let matches = command.get_matches();
    do_stuff(Config {
        flag: matches.get_flag("flag"),
        option: matches.get_one::<String>("option").cloned(),
        positional: matches.get_one::<String>("positional").unwrap().to_string(),
    })
}
