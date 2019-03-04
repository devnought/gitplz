// TODO:
// This is going to use regex for now, but is a good candidate for
// nom or other parser combinators.

use std::{fs, path::Path};

pub fn username_for_host(_hostname: &str) -> Option<String> {
    let config_path = Path::new("~/.ssh/config");
    let config_data =
        fs::read_to_string(config_path).expect("Could not read contents of ssh config file");

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
