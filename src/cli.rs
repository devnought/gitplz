use std::path::PathBuf;
use structopt::{clap::ArgGroup, StructOpt};

const APP_NAME: &str = "git plz";
const CMD_BRANCH: &str = "branch";

fn branch_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("branch").required(true)
}

#[derive(StructOpt, Debug)]
crate struct PathArg {
    /// Path to execute command. Defaults to working directory.
    #[structopt(name = "path", parse(from_os_str))]
    value: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
#[structopt(raw(bin_name = "APP_NAME"))]
crate enum RunOption {
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
        #[structopt(name = "branch")]
        branch: String,
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
    #[structopt(name = "status",)]
    Status {
        #[structopt(flatten)]
        path: PathArg,
    },
}

impl RunOption {
    crate fn path(&self) -> Option<&std::path::Path> {
        match self {
            RunOption::Branch { path, .. } => path.value.as_ref().map(|x| x.as_path()),
            RunOption::Checkout { path, .. } => path.value.as_ref().map(|x| x.as_path()),
            RunOption::Reset { path, .. } => path.value.as_ref().map(|x| x.as_path()),
            RunOption::Status { path, .. } => path.value.as_ref().map(|x| x.as_path()),
        }
    }
}

crate fn handle_args() -> RunOption {
    RunOption::from_args()
}
