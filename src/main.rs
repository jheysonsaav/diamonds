mod commands;
mod parser;
mod utils;

use std::env;

use clap::{crate_description, crate_version, App, Arg, SubCommand};
fn main() {
    let app = App::new("diamonds")
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("log-level")
                .short("L")
                .long("log-level")
                .takes_value(true)
                .global(true),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Run script files")
                .arg(Arg::with_name("file").value_name("FILE").required(true)),
        )
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate shell completions")
                .arg(
                    Arg::with_name("shell")
                        .possible_values(&["bash", "zsh", "powershell", "fish", "elvish"])
                        .required(true),
                ),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("log-level") {
        env::set_var("LOG_LEVEL", matches.value_of("log-level").unwrap());
    }

    match matches.subcommand() {
        ("completions", Some(args)) => commands::completions::completions_command(app, args),
        _ => {
            app.clone().print_help().expect("Cannot print help message");
            println!();
        }
    }
}
