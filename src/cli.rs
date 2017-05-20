use clap::{Arg, App, SubCommand, Shell};

pub const APP_NAME: &str = "git plz";
pub const CMD_CHECKOUT: &str = "checkout";
pub const CMD_COMPLETIONS: &str = "completions";
pub const CMD_MANIFEST: &str = "manifest";
pub const CMD_PREVIEW: &str = "preview";
pub const CMD_RESET: &str = "reset";
pub const CMD_STATUS: &str = "status";
pub const CMD_UPDATE: &str = "update";
pub const BRANCH: &str = "branch";
pub const SHELL: &str = "shell";

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("Git, please")
        .bin_name(APP_NAME)
        .version("0.1")
        .author("Kyle Gretchev")
        .about("Run commands on a set of git repositories in a folder tree")
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
        .subcommand(SubCommand::with_name(CMD_MANIFEST)
            .about("Inspect or generate manifest files")
            .subcommand(SubCommand::with_name(CMD_UPDATE)
                .about("Update or generate manifest file"))
            .subcommand(SubCommand::with_name(CMD_PREVIEW)
                .about("Preview the repositories that will be present in a manifest")))
        .subcommand(SubCommand::with_name(CMD_RESET)
            .about("Recursive hard reset"))
        .subcommand(SubCommand::with_name(CMD_STATUS)
            .about("Recursive directory search version of git status"))
}
