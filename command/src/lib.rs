extern crate color_printer;
extern crate gitlib;

macro_rules! mods {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
        )*
    };
}

mod command;
pub use command::Command;

mod worktype;
pub use worktype::{WorkResult, WorkType};

include!(concat!(env!("OUT_DIR"), "/generated-commands.rs"));

pub use checkout::CheckoutCommand;
pub use branch_delete::BranchDeleteCommand;
pub use branch_find::BranchFindCommand;
pub use reset::ResetCommand;
pub use status::StatusCommand;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
