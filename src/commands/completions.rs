use clap::{App, ArgMatches, Shell};
use std::io;

pub fn completions_command(app: App, args: &ArgMatches) {
    let shell: Shell;

    match args.value_of("shell").unwrap() {
        "bash" => shell = Shell::Bash,
        "zsh" => shell = Shell::Zsh,
        "powershell" => shell = Shell::PowerShell,
        "fish" => shell = Shell::Fish,
        "elvish" => shell = Shell::Elvish,
        _ => panic!(),
    }

    app.clone()
        .gen_completions_to("diamonds", shell, &mut io::stdout());
    println!();
}
