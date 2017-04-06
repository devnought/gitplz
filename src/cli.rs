use clap::{Arg, App, SubCommand, Shell};

pub const APP_NAME: &'static str = "git plz";
pub const CMD_STATUS: &'static str = "status";
pub const CMD_CHECKOUT: &'static str = "checkout";
pub const CMD_COMPLETIONS: &'static str = "completions";
pub const BRANCH: &'static str = "branch";
pub const SHELL: &'static str = "shell";

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("Git, please")
        .bin_name(APP_NAME)
        .version("0.1")
        .author("Kyle Gretchev")
        .about("Run commands on a set of git repositories in a folder tree")
        .subcommand(SubCommand::with_name(CMD_STATUS)
            .about("Recursive directory search version of git status"))
        .subcommand(SubCommand::with_name(CMD_CHECKOUT)
            .about("Checkout branch across repos")
            .arg(Arg::with_name(BRANCH)
                .required(true)
                .help("Branch name")))
        .subcommand(SubCommand::with_name(CMD_COMPLETIONS)
            .about("Generates completion scripts for your shell")
            .arg(Arg::with_name(SHELL)
                .required(true)
                .possible_values(&Shell::variants())
                .help("The shell to generate the script for")))
}
