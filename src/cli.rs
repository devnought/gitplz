use clap::{crate_authors, crate_version, value_t, App, Arg, ArgGroup, Shell, SubCommand};
use std::io;

const APP_NAME: &str = "git plz";
const CMD_BRANCH: &str = "branch";
const CMD_CHECKOUT: &str = "checkout";
const CMD_COMPLETIONS: &str = "completions";
const CMD_RESET: &str = "reset";
const CMD_STATUS: &str = "status";
const BRANCH: &str = "branch";
const DELETE: &str = "delete";
const FIND: &str = "find";
const SHELL: &str = "shell";

#[derive(Debug)]
crate enum CommandArg {
    Completions { shell: Shell },
    Help,
    Run { option: RunOption },
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

fn build_cli() -> App<'a, 'b> {
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
                        .value_name("BRANCH")
                        .group(CMD_BRANCH),
                )
                .arg(
                    Arg::with_name(FIND)
                        .short("f")
                        .long("find")
                        .value_name("BRANCH")
                        .group(CMD_BRANCH),
                ),
        )
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
                option: RunOption::Branch {
                    branch: value_t!(branch_matches, CMD_BRANCH, String).unwrap(),
                    option,
                },
            }
        }
        (CMD_CHECKOUT, Some(checkout_matches)) => CommandArg::Run {
            option: RunOption::Checkout {
                branch: value_t!(checkout_matches, BRANCH, String).unwrap(),
            },
        },
        (CMD_COMPLETIONS, Some(shell_matches)) => CommandArg::Completions {
            shell: value_t!(shell_matches, SHELL, Shell).unwrap(),
        },
        (CMD_RESET, _) => CommandArg::Run {
            option: RunOption::Reset,
        },
        (CMD_STATUS, _) => CommandArg::Run {
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
