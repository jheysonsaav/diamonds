mod commands;
mod parser;

use clap::{crate_description, crate_version, App, Arg, SubCommand};
fn main() {
    let app = App::new("diamonds")
        .version(crate_version!())
        .about(crate_description!())
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

    match matches.subcommand() {
        ("completions", Some(args)) => commands::completions::completions_command(app, args),
        _ => {
            app.clone().print_help().expect("Cannot print help message");
            println!();
        }
    }
}
