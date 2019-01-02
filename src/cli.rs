use command::*;
use std::{env, path::PathBuf};
use structopt::{clap::ArgGroup, StructOpt};

const APP_NAME: &str = "git plz";
const CMD_BRANCH: &str = "BRANCH";

fn branch_arg_group<'a>() -> ArgGroup<'a> {
    ArgGroup::with_name("branch").required(true)
}

#[derive(StructOpt, Debug)]
struct PathArg {
    /// Path to execute command. Defaults to working directory.
    #[structopt(name = "PATH", parse(from_os_str))]
    value: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
#[structopt(raw(bin_name = "APP_NAME"))]
enum RunOption {
    /// Perform bulk local branch operations
    #[structopt(name = "branch", raw(group = "branch_arg_group()"))]
    Branch {
        #[structopt(
            short = "f",
            long = "find",
            group = "branch",
            raw(value_name = "CMD_BRANCH")
        )]
        find: Option<String>,
        #[structopt(
            short = "d",
            long = "delete",
            group = "branch",
            raw(value_name = "CMD_BRANCH")
        )]
        delete: Option<String>,
        #[structopt(flatten)]
        path: PathArg,
    },
    /// Checkout branch across repos
    #[structopt(name = "checkout")]
    Checkout {
        /// Branch name
        #[structopt(name = "BRANCH")]
        branch: String,
        #[structopt(flatten)]
        path: PathArg,
    },
    /// Recursive fetch
    #[structopt(name = "fetch")]
    Fetch {
        #[structopt(flatten)]
        path: PathArg,
    },
    /// Recursive hard reset
    #[structopt(name = "reset")]
    Reset {
        #[structopt(flatten)]
        path: PathArg,
    },
    /// Recursive directory search version of git status
    #[structopt(name = "status")]
    Status {
        #[structopt(flatten)]
        path: PathArg,
    },
}

pub struct MappedArgs {
    command: Box<dyn Command>,
    path: PathBuf,
}

impl MappedArgs {
    fn new(run_option: RunOption) -> Self {
        struct TempArgs(Box<dyn Command>, PathArg);

        let temp_args = match run_option {
            RunOption::Branch { path, delete, find } => {
                // TODO: This needs to be an enum again
                if let Some(branch) = delete {
                    TempArgs(Box::new(BranchDeleteCommand::new(branch)), path)
                } else if let Some(branch) = find {
                    TempArgs(Box::new(BranchFindCommand::new(branch)), path)
                } else {
                    panic!("Invalid branch option");
                }
            }
            RunOption::Checkout { path, branch } => {
                TempArgs(Box::new(CheckoutCommand::new(branch)), path)
            }
            RunOption::Fetch { path } => TempArgs(Box::new(FetchCommand::new()), path),
            RunOption::Reset { path } => TempArgs(Box::new(ResetCommand::new()), path),
            RunOption::Status { path } => TempArgs(Box::new(StatusCommand::new()), path),
        };

        Self {
            command: temp_args.0,
            path: temp_args
                .1
                .value
                .unwrap_or_else(|| env::current_dir().expect("Could not get working directory")),
        }
    }

    pub fn destructure(self) -> (Box<dyn Command>, PathBuf) {
        (self.command, self.path)
    }
}

pub fn handle_args() -> MappedArgs {
    MappedArgs::new(RunOption::from_args())
}
