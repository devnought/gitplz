use clap::{App, Arg, Shell, SubCommand};
use std::io;

const APP_NAME: &str = "git plz";
const CMD_CHECKOUT: &str = "checkout";
const CMD_COMPLETIONS: &str = "completions";
const CMD_RESET: &str = "reset";
const CMD_STATUS: &str = "status";
const BRANCH: &str = "branch";
const SHELL: &str = "shell";

#[derive(Debug)]
pub enum CommandArg {
    Completions { shell: Shell },
    Help,
    Run { option: RunOption },
}

#[derive(Debug)]
pub enum RunOption {
    Checkout { branch: String },
    Reset,
    Status,
}

fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("Git, please")
        .bin_name(APP_NAME)
        .version("0.2")
        .author("Kyle Gretchev")
        .about("Run commands on a set of git repositories in a folder tree")
        .subcommand(
            SubCommand::with_name(CMD_CHECKOUT)
                .about("Checkout branch across repos")
                .arg(Arg::with_name(BRANCH).required(true).help("Branch name")),
        )
        .subcommand(
            SubCommand::with_name(CMD_COMPLETIONS)
                .about("Generates completion scripts for your shell")
                .arg(
                    Arg::with_name(SHELL)
                        .required(true)
                        .possible_values(&Shell::variants())
                        .help("The shell to generate the script for"),
                ),
        )
        .subcommand(SubCommand::with_name(CMD_RESET).about("Recursive hard reset"))
        .subcommand(
            SubCommand::with_name(CMD_STATUS)
                .about("Recursive directory search version of git status"),
        )
}

pub fn handle_args() -> CommandArg {
    let matches = build_cli().get_matches();

    match matches.subcommand_name() {
        Some(CMD_CHECKOUT) => {
            let branch_match = matches.subcommand_matches(CMD_CHECKOUT).unwrap();

            CommandArg::Run {
                option: RunOption::Checkout {
                    branch: value_t!(branch_match, BRANCH, String).unwrap(),
                },
            }
        }
        Some(CMD_COMPLETIONS) => {
            let shell_match = matches.subcommand_matches(CMD_COMPLETIONS).unwrap();

            CommandArg::Completions {
                shell: value_t!(shell_match, SHELL, Shell).unwrap(),
            }
        }
        Some(CMD_RESET) => CommandArg::Run {
            option: RunOption::Reset,
        },
        Some(CMD_STATUS) => CommandArg::Run {
            option: RunOption::Status,
        },

        // By default, show help.
        _ => CommandArg::Help,
    }
}

pub fn gen_completions_for(shell: Shell) {
    build_cli().gen_completions_to(APP_NAME, shell, &mut io::stdout())
}

pub fn print_help() {
    build_cli()
        .print_help()
        .expect("Could not print command line help message");
}
