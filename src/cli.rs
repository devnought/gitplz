use clap::{
    crate_authors, crate_version, value_t, App, Arg, ArgGroup, ArgMatches, Shell, SubCommand,
};
use std::{io, path::PathBuf};

const APP_NAME: &str = "git plz";
const CMD_BRANCH: &str = "branch";
const CMD_CHECKOUT: &str = "checkout";
const CMD_COMPLETIONS: &str = "completions";
const CMD_RESET: &str = "reset";
const CMD_STATUS: &str = "status";
const BRANCH: &str = "branch";
const DELETE: &str = "delete";
const FIND: &str = "find";
const PATH: &str = "path";
const SHELL: &str = "shell";

#[derive(Debug)]
crate enum CommandArg {
    Completions {
        shell: Shell,
    },
    Help,
    Run {
        path: Option<PathBuf>,
        option: RunOption,
    },
}

#[derive(Debug)]
crate enum RunOption {
    Branch {
        branch: String,
        option: BranchOption,
    },
    Checkout {
        branch: String,
    },
    Reset,
    Status,
}

#[derive(Debug)]
crate enum BranchOption {
    Delete,
    Find,
}

fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("Git, please")
        .bin_name(APP_NAME)
        .version(crate_version!())
        .author(crate_authors!())
        .about("Run commands on a set of git repositories in a folder tree")
        .subcommand(
            SubCommand::with_name(CMD_BRANCH)
                .about("Perform bulk local branch operations")
                .group(ArgGroup::with_name(CMD_BRANCH).required(true))
                .arg(
                    Arg::with_name(DELETE)
                        .short("d")
                        .long("delete")
                        .value_name("branch")
                        .group(CMD_BRANCH),
                ).arg(
                    Arg::with_name(FIND)
                        .short("f")
                        .long("find")
                        .value_name("branch")
                        .group(CMD_BRANCH),
                ).arg(path_arg()),
        ).subcommand(
            SubCommand::with_name(CMD_CHECKOUT)
                .about("Checkout branch across repos")
                .arg(Arg::with_name(BRANCH).required(true).help("Branch name"))
                .arg(path_arg()),
        ).subcommand(
            SubCommand::with_name(CMD_COMPLETIONS)
                .about("Generates completion scripts for your shell")
                .arg(
                    Arg::with_name(SHELL)
                        .required(true)
                        .possible_values(&Shell::variants())
                        .help("The shell to generate the script for"),
                ),
        ).subcommand(
            SubCommand::with_name(CMD_RESET)
                .about("Recursive hard reset")
                .arg(path_arg()),
        ).subcommand(
            SubCommand::with_name(CMD_STATUS)
                .about("Recursive directory search version of git status")
                .arg(path_arg()),
        )
}

fn path_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(PATH)
        .required(false)
        .value_name(PATH)
        .help("Path to execute command. Defaults to working directory.")
}

fn get_path(matches: Option<&ArgMatches<'_>>) -> Option<PathBuf> {
    if let Some(m) = matches {
        if m.is_present(PATH) {
            return Some(value_t!(m, PATH, String).unwrap().into());
        }
    }

    None
}

crate fn handle_args() -> CommandArg {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        (CMD_BRANCH, Some(branch_matches)) => {
            let option = {
                if branch_matches.is_present(DELETE) {
                    BranchOption::Delete
                } else if branch_matches.is_present(FIND) {
                    BranchOption::Find
                } else {
                    panic!("Invalid branch option");
                }
            };

            CommandArg::Run {
                path: get_path(Some(branch_matches)),
                option: RunOption::Branch {
                    branch: value_t!(branch_matches, CMD_BRANCH, String).unwrap(),
                    option,
                },
            }
        }
        (CMD_CHECKOUT, Some(checkout_matches)) => CommandArg::Run {
            path: get_path(Some(checkout_matches)),
            option: RunOption::Checkout {
                branch: value_t!(checkout_matches, BRANCH, String).unwrap(),
            },
        },
        (CMD_COMPLETIONS, Some(shell_matches)) => CommandArg::Completions {
            shell: value_t!(shell_matches, SHELL, Shell).unwrap(),
        },
        (CMD_RESET, m) => CommandArg::Run {
            path: get_path(m),
            option: RunOption::Reset,
        },
        (CMD_STATUS, m) => CommandArg::Run {
            path: get_path(m),
            option: RunOption::Status,
        },

        // By default, show help.
        _ => CommandArg::Help,
    }
}

crate fn gen_completions_for(shell: Shell) {
    build_cli().gen_completions_to(APP_NAME, shell, &mut io::stdout())
}

crate fn print_help() {
    build_cli()
        .print_help()
        .expect("Could not print command line help message");
}
