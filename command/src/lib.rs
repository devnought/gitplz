extern crate color_printer;
extern crate gitlib;

mod command;
pub use command::Command;

mod worktype;
pub use worktype::{WorkResult, WorkType};

mod checkout;
pub use checkout::CheckoutCommand;

mod branch_delete;
pub use branch_delete::BranchDeleteCommand;

mod branch_find;
pub use branch_find::BranchFindCommand;

mod reset;
pub use reset::ResetCommand;

mod status;
pub use status::StatusCommand;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
